import { TextPromptRequest, ModelShapeAttributes } from '../types/TextPromptModel';

const keywordMap: Record<string, string[]> = {
  box: ['куб'],
  sphere: ['шар', 'сфера'],
  cylinder: ['цилиндр'],
  pyramid: ['пирамида']
};

const colorMap: Record<string, string> = {
  красный: '#FF0000',
  синий: '#0000FF',
  зелёный: '#00FF00',
  жёлтый: '#FFFF00',
  белый: '#FFFFFF',
  чёрный: '#000000',
  деревянный: 'texture:wood.jpg'
};

export function parsePrompt(input: string): Partial<ModelShapeAttributes> {
  const lowerCaseInput = input.toLowerCase();
  let attributes: Partial<ModelShapeAttributes> = {};

  // Ищем форму
  for (const [key, keywords] of Object.entries(keywordMap)) {
    if (keywords.some(kw => lowerCaseInput.includes(kw))) {
      attributes.type = key as any;
      break;
    }
  }

  // Ищем цвет
  for (const [key, value] of Object.entries(colorMap)) {
    if (lowerCaseInput.includes(key)) {
      if (value.startsWith('texture:')) {
        attributes.textureUrl = value.replace('texture:', '');
      } else {
        attributes.color = value;
      }
      break;
    }
  }

  // Ищем размеры: высотой/радиусом X [см/мм]
  const heightMatch = lowerCaseInput.match(/высотой\s+([\d\.]+)\s*(см|мм)?/i);
  if (heightMatch) {
    const val = parseFloat(heightMatch[1]);
    attributes.dimensions = attributes.dimensions || {};
    attributes.dimensions.height = heightMatch[2] === 'см' ? val * 10 : val;
  }

  const radiusMatch = lowerCaseInput.match(/радиусом\s+([\d\.]+)\s*(см|мм)?/i);
  if (radiusMatch) {
    const val = parseFloat(radiusMatch[1]);
    attributes.dimensions = attributes.dimensions || {};
    attributes.dimensions.radius = radiusMatch[2] === 'см' ? val * 10 : val;
  }

  return attributes;
}
