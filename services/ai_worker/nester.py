"""
Модуль для 2D nesting (упаковки) разверток на листах бумаги.
Реализует алгоритмы bottom-left и NFP (No-Fit Polygon).
"""
import math
import json
from typing import List, Tuple, Optional, Dict, Any
from dataclasses import dataclass
try:
    import svgwrite
    SVGWRITE_AVAILABLE = True
except ImportError:
    SVGWRITE_AVAILABLE = False


@dataclass
class Rectangle:
    """Прямоугольник с позицией и размерами."""
    x: float
    y: float
    width: float
    height: float
    rotation: float = 0.0
    id: Optional[int] = None
    name: Optional[str] = None

    def area(self) -> float:
        return self.width * self.height

    def intersects(self, other: 'Rectangle') -> bool:
        """Проверка пересечения двух прямоугольников (без учёта поворота)."""
        return (self.x < other.x + other.width and
                self.x + self.width > other.x and
                self.y < other.y + other.height and
                self.y + self.height > other.y)

    def contains(self, other: 'Rectangle') -> bool:
        """Проверка, содержит ли этот прямоугольник другой."""
        return (self.x <= other.x and
                self.x + self.width >= other.x + other.width and
                self.y <= other.y and
                self.y + self.height >= other.y + other.height)


@dataclass
class Sheet:
    """Лист бумаги."""
    width: float
    height: float
    margin: float = 5.0
    gap: float = 2.0
    parts: List[Rectangle] = None

    def __post_init__(self):
        if self.parts is None:
            self.parts = []

    def add_part(self, part: Rectangle) -> bool:
        """Добавить часть на лист, если она помещается и не пересекается."""
        # Проверка на пересечение с существующими частями
        for existing in self.parts:
            if part.intersects(existing):
                return False
        # Проверка, что часть внутри листа с учётом отступа
        if (part.x >= self.margin and
            part.y >= self.margin and
            part.x + part.width <= self.width - self.margin and
            part.y + part.height <= self.height - self.margin):
            self.parts.append(part)
            return True
        return False

    def utilization(self) -> float:
        """Вычисление утилизации листа (заполненная площадь / общая площадь)."""
        total_area = self.width * self.height
        if total_area == 0:
            return 0.0
        parts_area = sum(p.area() for p in self.parts)
        return parts_area / total_area * 100.0


class Nester:
    """Класс для выполнения 2D nesting."""

    def __init__(self, algorithm: str = "bottom-left"):
        """
        :param algorithm: "bottom-left" или "nfp"
        """
        self.algorithm = algorithm

    def nest(self, parts: List[Tuple[float, float]], sheet_size: Tuple[float, float],
             gap: float = 2.0) -> List[Sheet]:
        """
        Разместить части на листах.

        :param parts: список кортежей (width, height) для каждой части
        :param sheet_size: кортеж (width, height) размера листа
        :param gap: зазор между частями (в мм)
        :return: список листов с размещёнными частями
        """
        if self.algorithm == "bottom-left":
            return self._bottom_left_nest(parts, sheet_size, gap)
        elif self.algorithm == "nfp":
            return self._nfp_nest(parts, sheet_size, gap)
        else:
            raise ValueError(f"Неизвестный алгоритм: {self.algorithm}")

    def _bottom_left_nest(self, parts: List[Tuple[float, float]],
                          sheet_size: Tuple[float, float], gap: float) -> List[Sheet]:
        """
        Алгоритм Bottom-Left (простейший).
        Размещает части последовательно, начиная с левого нижнего угла,
        двигаясь вправо и вверх.
        """
        sheet_width, sheet_height = sheet_size
        sheets = [Sheet(sheet_width, sheet_height, gap=gap)]
        # Сортируем части по убыванию площади для лучшего заполнения
        sorted_parts = sorted([(w, h, i) for i, (w, h) in enumerate(parts)],
                              key=lambda x: x[0] * x[1], reverse=True)

        for w, h, idx in sorted_parts:
            placed = False
            for sheet in sheets:
                # Попробовать разместить без поворота
                if self._try_place_in_sheet(sheet, w, h, idx, gap):
                    placed = True
                    break
                # Попробовать с поворотом на 90 градусов
                if self._try_place_in_sheet(sheet, h, w, idx, gap, rotated=True):
                    placed = True
                    break
            if not placed:
                # Создать новый лист
                new_sheet = Sheet(sheet_width, sheet_height, gap=gap)
                if self._try_place_in_sheet(new_sheet, w, h, idx, gap):
                    sheets.append(new_sheet)
                else:
                    # Если даже на пустом листе не помещается - ошибка
                    raise RuntimeError(f"Часть {idx} слишком велика для листа")
        return sheets

    def _try_place_in_sheet(self, sheet: Sheet, w: float, h: float, idx: int,
                            gap: float, rotated: bool = False) -> bool:
        """
        Попытаться разместить часть на листе, используя стратегию bottom-left.
        """
        # Генерируем кандидатов позиций
        candidates = self._generate_candidates(sheet, w, h, gap)
        for x, y in candidates:
            rect = Rectangle(x, y, w, h, id=idx,
                             name=f"Part{idx}{'_rot' if rotated else ''}")
            if sheet.add_part(rect):
                return True
        return False

    def _generate_candidates(self, sheet: Sheet, w: float, h: float,
                             gap: float) -> List[Tuple[float, float]]:
        """
        Генерирует кандидатов позиций для размещения части.
        Упрощённо: сканируем сетку с шагом gap.
        """
        candidates = []
        step = gap
        for y in range(int(sheet.margin), int(sheet.height - h - sheet.margin) + 1, int(step)):
            for x in range(int(sheet.margin), int(sheet.width - w - sheet.margin) + 1, int(step)):
                candidates.append((float(x), float(y)))
        # Также добавляем позиции вдоль границ уже размещённых частей
        for part in sheet.parts:
            # справа от части
            candidates.append((part.x + part.width + gap, part.y))
            # сверху от части
            candidates.append((part.x, part.y + part.height + gap))
        # Убираем дубликаты и сортируем по y, затем по x (bottom-left)
        candidates = list(set(candidates))
        candidates.sort(key=lambda pos: (pos[1], pos[0]))
        return candidates

    def _nfp_nest(self, parts: List[Tuple[float, float]],
                  sheet_size: Tuple[float, float], gap: float) -> List[Sheet]:
        """
        Алгоритм NFP (No-Fit Polygon) - более продвинутый.
        Пока заглушка.
        """
        # TODO: реализовать NFP алгоритм
        raise NotImplementedError("NFP алгоритм ещё не реализован")

    def export_to_svg(self, sheets: List[Sheet], filename: str):
        """
        Экспортировать размещение в SVG файл.
        """
        if not SVGWRITE_AVAILABLE:
            raise ImportError("Модуль svgwrite не установлен. Установите его: pip install svgwrite")
        dwg = svgwrite.Drawing(filename, size=(f"{sheets[0].width}mm", f"{sheets[0].height}mm"))
        # Рисуем листы
        for i, sheet in enumerate(sheets):
            # Создаём группу для каждого листа
            g = dwg.g(id=f"sheet_{i}")
            # Контур листа
            g.add(dwg.rect(insert=(0, 0), size=(sheet.width, sheet.height),
                           fill='none', stroke='black', stroke_width='0.5'))
            # Части
            for part in sheet.parts:
                g.add(dwg.rect(insert=(part.x, part.y), size=(part.width, part.height),
                               fill='none', stroke='blue', stroke_width='0.2',
                               fill_opacity=0.1))
                # Подпись id
                g.add(dwg.text(f"{part.id}", insert=(part.x + part.width/2, part.y + part.height/2),
                               text_anchor="middle", font_size="3"))
            dwg.add(g)
        dwg.save()

    def calculate_metrics(self, sheets: List[Sheet]) -> Dict[str, Any]:
        """
        Рассчитать метрики упаковки.
        """
        total_sheets = len(sheets)
        total_parts = sum(len(s.parts) for s in sheets)
        total_parts_area = sum(sum(p.area() for p in s.parts) for s in sheets)
        total_sheets_area = sum(s.width * s.height for s in sheets)
        avg_utilization = (total_parts_area / total_sheets_area * 100) if total_sheets_area > 0 else 0
        return {
            "total_sheets": total_sheets,
            "total_parts": total_parts,
            "total_parts_area": total_parts_area,
            "total_sheets_area": total_sheets_area,
            "avg_utilization_percent": avg_utilization,
            "sheet_utilizations": [s.utilization() for s in sheets]
        }


# Функция для использования из командной строки
def main():
    import sys
    if len(sys.argv) < 2:
        print("Использование: python nester.py <config.json>")
        sys.exit(1)
    config_file = sys.argv[1]
    with open(config_file, 'r') as f:
        config = json.load(f)
    parts = config.get("parts", [])
    sheet_size = config.get("sheet_size", [210, 297])
    gap = config.get("gap", 2.0)
    algorithm = config.get("algorithm", "bottom-left")

    nester = Nester(algorithm=algorithm)
    sheets = nester.nest(parts, tuple(sheet_size), gap)
    metrics = nester.calculate_metrics(sheets)
    print(f"Размещено листов: {metrics['total_sheets']}")
    print(f"Утилизация: {metrics['avg_utilization_percent']:.2f}%")
    # Экспорт в SVG
    if SVGWRITE_AVAILABLE:
        nester.export_to_svg(sheets, "nesting_result.svg")
        print("Результат сохранён в nesting_result.svg")
    else:
        print("Предупреждение: svgwrite не установлен, экспорт SVG пропущен.")


if __name__ == "__main__":
    main()