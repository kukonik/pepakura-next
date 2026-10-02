#!/usr/bin/env python3
"""
Тестирование AI-функционала Pepakura Next.
Проверяет доступность модулей и базовую работоспособность.
"""
import sys
import subprocess
import json
from pathlib import Path

def run_test(name, check_func):
    """Запускает тест и возвращает результат."""
    try:
        success, message = check_func()
        status = "OK" if success else "FAIL"
        print(f"{name}: {status} - {message}")
        return success, message
    except Exception as e:
        print(f"{name}: FAIL - исключение: {e}")
        return False, str(e)

def check_triposr():
    """Проверка TripoSR."""
    try:
        sys.path.insert(0, str(Path('services/ai_worker')))
        from triposr_generator import TripoSRGenerator
        # Проверяем, есть ли каталог TripoSR
        triposr_dir = Path('services/ai_worker/TripoSR')
        if not triposr_dir.exists():
            return False, "Каталог TripoSR отсутствует"
        # Проверяем наличие модели (хотя бы файлов)
        if not (triposr_dir / 'tsr').exists():
            return False, "Подмодуль tsr отсутствует"
        # Пытаемся импортировать tsr (может не установлены зависимости)
        try:
            import torch
        except ImportError:
            return False, "Torch не установлен"
        try:
            import omegaconf
        except ImportError:
            return False, "omegaconf не установлен (требуется для TripoSR)"
        return True, "Модуль загружен, зависимости в порядке"
    except ImportError as e:
        return False, f"Импорт не удался: {e}"

def check_hunyuan():
    """Проверка Hunyuan3D-2."""
    try:
        sys.path.insert(0, str(Path('services/ai_worker')))
        from hunyuan_generator import HunyuanGenerator, HAS_HUNYUAN
        if HAS_HUNYUAN:
            return True, "Модуль Hunyuan3D-2 доступен"
        else:
            # Это ожидаемо, так как модель не установлена
            return True, "Модуль заглушки работает (реальная модель не установлена)"
    except ImportError as e:
        return False, f"Импорт не удался: {e}"

def check_xatlas():
    """Проверка xatlas."""
    try:
        import xatlas
        return True, "xatlas установлен"
    except ImportError:
        # Попробуем установить через pip? (пропустим)
        return False, "xatlas не установлен"

def check_nesting():
    """Проверка nesting."""
    try:
        sys.path.insert(0, str(Path('services/ai_worker')))
        from nester import Nester
        # Проверяем, что класс создаётся
        nester = Nester()
        # Простой тест с dummy данными
        parts = [(50, 30), (40, 20), (60, 40)]
        sheets = nester.nest(parts, (210, 297), gap=2.0)
        if len(sheets) > 0:
            return True, f"Упаковка выполнена, листов: {len(sheets)}"
        else:
            return False, "Упаковка не создала листы"
    except Exception as e:
        return False, f"Ошибка nesting: {e}"

def check_overall():
    """Общая проверка окружения."""
    # Проверяем Python версию
    python_version = sys.version.split()[0]
    # Проверяем наличие torch, trimesh, numpy, Pillow
    deps = ['torch', 'trimesh', 'numpy', 'PIL']
    missing = []
    for dep in deps:
        try:
            __import__(dep if dep != 'PIL' else 'PIL.Image')
        except ImportError:
            missing.append(dep)
    if missing:
        return False, f"Отсутствуют зависимости: {', '.join(missing)}"
    return True, f"Python {python_version}, зависимости в порядке"

def main():
    print("=== Комплексное тестирование AI-функционала ===")
    results = []
    
    # Общая проверка
    overall_ok, overall_msg = run_test("Общее окружение", check_overall)
    results.append(("Общее окружение", overall_ok, overall_msg))
    
    # Тесты
    tests = [
        ("TripoSR (Image-to-3D)", check_triposr),
        ("Hunyuan3D-2 (Text-to-3D)", check_hunyuan),
        ("xatlas (UV Unwrap)", check_xatlas),
        ("Nesting", check_nesting),
    ]
    
    for name, func in tests:
        ok, msg = run_test(name, func)
        results.append((name, ok, msg))
    
    print("\n=== Сводка результатов ===")
    all_ok = all(ok for _, ok, _ in results)
    for name, ok, msg in results:
        status = "[OK]" if ok else "[FAIL]"
        print(f"{status} {name}: {msg}")
    
    if all_ok:
        print("\n[SUCCESS] Все тесты пройдены успешно!")
    else:
        print("\n[FAILURE] Некоторые тесты не прошли. Рекомендации:")
        if not any(ok for name, ok, _ in results if "TripoSR" in name):
            print("  - Установите TripoSR: клонируйте репозиторий в services/ai_worker/TripoSR и установите зависимости")
            print("  - Установите omegaconf: pip install omegaconf")
        if not any(ok for name, ok, _ in results if "xatlas" in name):
            print("  - Установите xatlas: pip install xatlas")
        if not any(ok for name, ok, _ in results if "Hunyuan3D-2" in name):
            print("  - Установите Hunyuan3D-2 (опционально): клонируйте репозиторий Tencent/Hunyuan3D-2")
        print("  - Убедитесь, что установлены все зависимости из services/ai_worker/requirements.txt")
    
    # Возвращаем код выхода
    sys.exit(0 if all_ok else 1)

if __name__ == "__main__":
    main()