import re
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass, field
from enum import Enum

@dataclass
class ParsedPrompt:
    main_subject: str
    style: Optional[str] = None
    colors: List[str] = field(default_factory=list)
    materials: List[str] = field(default_factory=list)
    details: List[str] = field(default_factory=list)
    background: Optional[str] = None
    quality: Optional[str] = None
    lighting: Optional[str] = None
    camera_angle: Optional[str] = None
    dimensions: Dict[str, float] = field(default_factory=dict)
    language: str = "en"  # "en" or "ru"

class Language(Enum):
    ENGLISH = "en"
    RUSSIAN = "ru"

class AdvancedPromptParser:
    def __init__(self):
        # Стили на английском и русском
        self.style_patterns = {
            'realistic': [r'\b(realistic|photorealistic|hyperrealistic)\b', r'\b(реалистичный|фотографический|гиперреалистичный)\b'],
            'cartoon': [r'\b(cartoon|anime|toon|cel-shaded)\b', r'\b(мультфильм|аниме|мультяшный)\b'],
            'lowpoly': [r'\b(low poly|lowpoly|polygonal)\b', r'\b(низкополигональный|лоуполи)\b'],
            'cyberpunk': [r'\b(cyberpunk|futuristic|sci-fi)\b', r'\b(киберпанк|футуристический|научная фантастика)\b'],
            'medieval': [r'\b(medieval|fantasy|knight|castle)\b', r'\b(средневековый|фэнтези|рыцарь|замок)\b'],
            'minimalist': [r'\b(minimalist|simple|clean)\b', r'\b(минималистичный|простой|чистый)\b'],
            'vintage': [r'\b(vintage|retro|old|classic)\b', r'\b(винтажный|ретро|старый|классический)\b'],
        }
        
        # Цвета на английском и русском
        self.color_patterns = [
            r'\b(red|blue|green|yellow|orange|purple|pink|brown|black|white|gray|silver|gold)\b',
            r'\b(красный|синий|зеленый|желтый|оранжевый|фиолетовый|розовый|коричневый|черный|белый|серый|серебристый|золотой)\b'
        ]
        
        # Материалы на английском и русском
        self.material_patterns = {
            'metal': [r'\b(metal|steel|iron|bronze|copper|aluminum)\b', r'\b(металл|сталь|железо|бронза|медь|алюминий)\b'],
            'wood': [r'\b(wood|timber|oak|pine|mahogany)\b', r'\b(дерево|дуб|сосна|махагони)\b'],
            'plastic': [r'\b(plastic|polymer|synthetic)\b', r'\b(пластик|полимер|синтетический)\b'],
            'glass': [r'\b(glass|crystal|transparent)\b', r'\b(стекло|хрусталь|прозрачный)\b'],
            'fabric': [r'\b(fabric|cloth|textile|cotton|silk)\b', r'\b(ткань|хлопок|шелк)\b'],
            'stone': [r'\b(stone|rock|marble|granite)\b', r'\b(камень|скала|мрамор|гранит)\b'],
            'rubber': [r'\b(rubber|latex|elastic)\b', r'\b(резина|латекс|эластичный)\b'],
        }
        
        # Качество на английском и русском
        self.quality_patterns = {
            'high': [r'\b(high quality|detailed|intricate|high resolution)\b', r'\b(высокое качество|детализированный|сложный|высокое разрешение)\b'],
            'low': [r'\b(low quality|simple|basic)\b', r'\b(низкое качество|простой|базовый)\b'],
        }
        
        # Фон на английском и русском
        self.background_patterns = {
            'indoor': [r'\b(indoor|room|house|building)\b', r'\b(внутри помещения|комната|дом|здание)\b'],
            'outdoor': [r'\b(outdoor|landscape|nature|forest|mountain)\b', r'\b(на улице|пейзаж|природа|лес|горы)\b'],
            'studio': [r'\b(studio|white background|plain background)\b', r'\b(студия|белый фон|чистый фон)\b'],
            'space': [r'\b(space|cosmic|galaxy)\b', r'\b(космос|космический|галактика)\b'],
        }
        
        # Освещение на английском и русском
        self.lighting_patterns = {
            'bright': [r'\b(bright|well-lit|illuminated)\b', r'\b(яркий|хорошо освещенный|освещенный)\b'],
            'dark': [r'\b(dark|dim|shadowy)\b', r'\b(темный|тусклый|тенистый)\b'],
            'dramatic': [r'\b(dramatic lighting|high contrast)\b', r'\b(драматическое освещение|высокий контраст)\b'],
            'soft': [r'\b(soft lighting|diffused)\b', r'\b(мягкое освещение|рассеянный)\b'],
        }
        
        # Углы камеры на английском и русском
        self.camera_angle_patterns = {
            'front': [r'\b(front view|frontal)\b', r'\b(вид спереди|фронтальный)\b'],
            'side': [r'\b(side view|profile)\b', r'\b(вид сбоку|профиль)\b'],
            'top': [r'\b(top view|overhead)\b', r'\b(вид сверху|сверху)\b'],
            'bottom': [r'\b(bottom view|underneath)\b', r'\b(вид снизу|снизу)\b'],
            'isometric': [r'\b(isometric view|isometric)\b', r'\b(изометрический вид|изометрия)\b'],
        }
        
        # Размеры на английском и русском
        self.size_patterns = {
            'large': [r'\b(large|big|huge|massive|enormous)\b', r'\b(большой|огромный|массивный|громадный)\b'],
            'small': [r'\b(small|tiny|miniature|little)\b', r'\b(маленький|крошечный|миниатюрный|небольшой)\b'],
            'medium': [r'\b(medium|average|normal size)\b', r'\b(средний|обычный|нормальный размер)\b'],
        }
        
        # Состояния на английском и русском
        self.condition_patterns = {
            'old': [r'\b(old|ancient|vintage|worn)\b', r'\b(старый|древний|винтажный|поношенный)\b'],
            'new': [r'\b(new|modern|fresh|brand new)\b', r'\b(новый|современный|свежий|совершенно новый)\b'],
            'damaged': [r'\b(damaged|broken|cracked|worn)\b', r'\b(поврежденный|сломанный|треснувший|поношенный)\b'],
            'shiny': [r'\b(shiny|glossy|polished)\b', r'\b(блестящий|глянцевый|полированный)\b'],
        }

    def detect_language(self, prompt: str) -> Language:
        """Определяет язык промпта"""
        russian_chars = re.findall(r'[а-яА-ЯёЁ]', prompt)
        english_chars = re.findall(r'[a-zA-Z]', prompt)
        
        if len(russian_chars) > len(english_chars):
            return Language.RUSSIAN
        else:
            return Language.ENGLISH

    def parse(self, prompt: str) -> ParsedPrompt:
        # Определяем язык
        language = self.detect_language(prompt)
        lang_code = language.value
        
        # Извлекаем основной субъект
        main_subject = self._extract_main_subject(prompt, lang_code)
        
        # Извлекаем стиль
        style = self._extract_style(prompt, lang_code)
        
        # Извлекаем цвета
        colors = self._extract_colors(prompt, lang_code)
        
        # Извлекаем материалы
        materials = self._extract_materials(prompt, lang_code)
        
        # Извлекаем детали
        details = self._extract_details(prompt, lang_code)
        
        # Извлекаем фон
        background = self._extract_background(prompt, lang_code)
        
        # Извлекаем качество
        quality = self._extract_quality(prompt, lang_code)
        
        # Извлекаем освещение
        lighting = self._extract_lighting(prompt, lang_code)
        
        # Извлекаем угол камеры
        camera_angle = self._extract_camera_angle(prompt, lang_code)
        
        # Извлекаем размеры
        dimensions = self._extract_dimensions(prompt, lang_code)
        
        return ParsedPrompt(
            main_subject=main_subject,
            style=style,
            colors=colors,
            materials=materials,
            details=details,
            background=background,
            quality=quality,
            lighting=lighting,
            camera_angle=camera_angle,
            dimensions=dimensions,
            language=lang_code
        )

    def _extract_main_subject(self, prompt: str, lang_code: str) -> str:
        """Извлекает основной субъект из промпта"""
        # Простой подход: берем первые несколько слов как основной субъект
        words = prompt.split()
        if len(words) <= 3:
            return prompt
        return ' '.join(words[:3])

    def _extract_style(self, prompt: str, lang_code: str) -> Optional[str]:
        """Извлекает стиль из промпта"""
        lang_index = 1 if lang_code == "ru" else 0
        for style, patterns in self.style_patterns.items():
            if re.search(patterns[lang_index], prompt, re.IGNORECASE):
                return style
        return None

    def _extract_colors(self, prompt: str, lang_code: str) -> List[str]:
        """Извлекает цвета из промпта"""
        colors = []
        for pattern in self.color_patterns:
            matches = re.findall(pattern, prompt, re.IGNORECASE)
            colors.extend([m.lower() for m in matches])
        return list(set(colors))  # Удаляем дубликаты

    def _extract_materials(self, prompt: str, lang_code: str) -> List[str]:
        """Извлекает материалы из промпта"""
        lang_index = 1 if lang_code == "ru" else 0
        materials = []
        for material, patterns in self.material_patterns.items():
            if re.search(patterns[lang_index], prompt, re.IGNORECASE):
                materials.append(material)
        return materials

    def _extract_details(self, prompt: str, lang_code: str) -> List[str]:
        """Извлекает дополнительные детали из промпта"""
        lang_index = 1 if lang_code == "ru" else 0
        details = []
        
        # Ищем описания размера
        for size, patterns in self.size_patterns.items():
            if re.search(patterns[lang_index], prompt, re.IGNORECASE):
                details.append(f"size:{size}")
                
        # Ищем описания состояния
        for condition, patterns in self.condition_patterns.items():
            if re.search(patterns[lang_index], prompt, re.IGNORECASE):
                details.append(f"condition:{condition}")
                
        return list(set(details))  # Удаляем дубликаты

    def _extract_background(self, prompt: str, lang_code: str) -> Optional[str]:
        """Извлекает фон из промпта"""
        lang_index = 1 if lang_code == "ru" else 0
        for background, patterns in self.background_patterns.items():
            if re.search(patterns[lang_index], prompt, re.IGNORECASE):
                return background
        return None

    def _extract_quality(self, prompt: str, lang_code: str) -> Optional[str]:
        """Извлекает качество из промпта"""
        lang_index = 1 if lang_code == "ru" else 0
        for quality, patterns in self.quality_patterns.items():
            if re.search(patterns[lang_index], prompt, re.IGNORECASE):
                return quality
        return None

    def _extract_lighting(self, prompt: str, lang_code: str) -> Optional[str]:
        """Извлекает освещение из промпта"""
        lang_index = 1 if lang_code == "ru" else 0
        for lighting, patterns in self.lighting_patterns.items():
            if re.search(patterns[lang_index], prompt, re.IGNORECASE):
                return lighting
        return None

    def _extract_camera_angle(self, prompt: str, lang_code: str) -> Optional[str]:
        """Извлекает угол камеры из промпта"""
        lang_index = 1 if lang_code == "ru" else 0
        for angle, patterns in self.camera_angle_patterns.items():
            if re.search(patterns[lang_index], prompt, re.IGNORECASE):
                return angle
        return None

    def _extract_dimensions(self, prompt: str, lang_code: str) -> Dict[str, float]:
        """Извлекает размеры из промпта"""
        dimensions = {}
        
        # Ищем числовые значения с единицами измерения
        if lang_code == "ru":
            # Русские единицы измерения
            height_match = re.search(r'высотой\s+([\d\.]+)\s*(см|мм)', prompt, re.IGNORECASE)
            width_match = re.search(r'шириной\s+([\d\.]+)\s*(см|мм)', prompt, re.IGNORECASE)
            depth_match = re.search(r'глубиной\s+([\d\.]+)\s*(см|мм)', prompt, re.IGNORECASE)
        else:
            # Английские единицы измерения
            height_match = re.search(r'height\s+([\d\.]+)\s*(cm|mm)', prompt, re.IGNORECASE)
            width_match = re.search(r'width\s+([\d\.]+)\s*(cm|mm)', prompt, re.IGNORECASE)
            depth_match = re.search(r'depth\s+([\d\.]+)\s*(cm|mm)', prompt, re.IGNORECASE)
        
        if height_match:
            val = float(height_match.group(1))
            unit = height_match.group(2)
            dimensions["height"] = val * 10 if unit.lower() in ["cm", "см"] else val
            
        if width_match:
            val = float(width_match.group(1))
            unit = width_match.group(2)
            dimensions["width"] = val * 10 if unit.lower() in ["cm", "см"] else val
            
        if depth_match:
            val = float(depth_match.group(1))
            unit = depth_match.group(2)
            dimensions["depth"] = val * 10 if unit.lower() in ["cm", "см"] else val
            
        return dimensions

# Пример использования
if __name__ == "__main__":
    parser = AdvancedPromptParser()
    
    # Тестовые промпты на английском и русском
    test_prompts = [
        "A red sports car with chrome details, realistic style, on a white background",
        "A medieval knight with armor, fantasy style, in a forest landscape",
        "A cyberpunk robot with glowing blue eyes, futuristic style, detailed design",
        "Красная спортивная машина с хромированными деталями, реалистичный стиль, на белом фоне",
        "Средневековый рыцарь в доспехах, фэнтези стиль, в лесном пейзаже",
        "Киберпанк робот с светящимися синими глазами, футуристический стиль, детализированный дизайн"
    ]
    
    for prompt in test_prompts:
        parsed = parser.parse(prompt)
        print(f"Prompt: {prompt}")
        print(f"Parsed: {parsed}")
        print("---")