def parse_prompt_to_dict(text):
    result = {}
    text_lower = text.lower()

    # Определение формы
    shape_keywords = {
        "box": ["куб"],
        "sphere": ["шар", "сфера"],
        "cylinder": ["цилиндр"],
        "pyramid": ["пирамида"]
    }

    for shape, keywords in shape_keywords.items():
        if any(keyword in text_lower for keyword in keywords):
            result["type"] = shape
            break

    # Цвет и текстура
    material_map = {
        "красный": {"color": "#FF0000"},
        "синий": {"color": "#0000FF"},
        "зелёный": {"color": "#00FF00"},
        "жёлтый": {"color": "#FFFF00"},
        "белый": {"color": "#FFFFFF"},
        "чёрный": {"color": "#000000"},
        "деревянный": {"texture_url": "/textures/wood.jpg"},
        "золотая": {"texture_url": "/textures/gold.jpg"},
        "резиновая": {"material_type": "rubber"}
    }

    for word, attrs in material_map.items():
        if word in text_lower:
            result.update(attrs)
            break

    # Размеры
    import re

    height_match = re.search(r"высотой\s+([\d\.]+)\s*(см|мм)", text_lower)
    if height_match:
        val = float(height_match.group(1))
        unit = height_match.group(2)
        result.setdefault("dimensions", {})["height"] = val * 10 if unit == "см" else val

    radius_match = re.search(r"радиусом\s+([\d\.]+)\s*(см|мм)", text_lower)
    if radius_match:
        val = float(radius_match.group(1))
        unit = radius_match.group(2)
        result.setdefault("dimensions", {})["radius"] = val * 10 if unit == "см" else val

    return result
