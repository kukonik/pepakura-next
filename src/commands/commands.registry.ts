/**
 * Реестр команд для Command Palette
 * Содержит все доступные команды приложения
 */

export interface Command {
  id: string;
  title: string;
  description: string;
  icon?: string;
  category: 'file' | 'edit' | 'view' | 'tools' | 'export' | 'settings' | 'help';
  hotkey?: string;
  action: () => void | Promise<void>;
  keywords?: string[]; // Для fuzzy search
}

// Глобальный реестр команд
class CommandRegistry {
  private commands: Map<string, Command> = new Map();
  private categories: Set<string> = new Set();

  register(command: Command) {
    this.commands.set(command.id, command);
    this.categories.add(command.category);
  }

  unregister(id: string) {
    this.commands.delete(id);
  }

  getCommand(id: string): Command | undefined {
    return this.commands.get(id);
  }

  getAllCommands(): Command[] {
    return Array.from(this.commands.values());
  }

  getCommandsByCategory(category: string): Command[] {
    return this.getAllCommands().filter(cmd => cmd.category === category);
  }

  getCategories(): string[] {
    return Array.from(this.categories);
  }

  search(query: string): Command[] {
    const q = query.toLowerCase().trim();
    if (!q) return this.getAllCommands();

    return this.getAllCommands().filter(cmd => {
      // Поиск по заголовку, описанию и ключевым словам
      const searchText = [
        cmd.title.toLowerCase(),
        cmd.description.toLowerCase(),
        ...(cmd.keywords || []).map(k => k.toLowerCase())
      ].join(' ');

      return searchText.includes(q) || 
             cmd.title.toLowerCase().includes(q) ||
             cmd.description.toLowerCase().includes(q);
    });
  }
}

// Создаем глобальный экземпляр реестра
export const commandRegistry = new CommandRegistry();

// Базовые команды приложения
export const baseCommands: Command[] = [
  // Файловые операции
  {
    id: 'file.new',
    title: 'New Project',
    description: 'Создать новый проект',
    icon: '📄',
    category: 'file',
    hotkey: 'Ctrl+N',
    action: () => {
      console.log('Создание нового проекта');
      // TODO: Реализовать создание нового проекта
    },
    keywords: ['новый', 'проект', 'create']
  },
  {
    id: 'file.open',
    title: 'Open Project',
    description: 'Открыть существующий проект',
    icon: '📂',
    category: 'file',
    hotkey: 'Ctrl+O',
    action: () => {
      console.log('Открытие проекта');
      // TODO: Реализовать открытие проекта
    },
    keywords: ['открыть', 'загрузить', 'load']
  },
  {
    id: 'file.save',
    title: 'Save Project',
    description: 'Сохранить текущий проект',
    icon: '💾',
    category: 'file',
    hotkey: 'Ctrl+S',
    action: () => {
      console.log('Сохранение проекта');
      // TODO: Реализовать сохранение проекта
    },
    keywords: ['сохранить', 'save']
  },
  {
    id: 'file.export.pdf',
    title: 'Export PDF',
    description: 'Экспортировать развертки в PDF',
    icon: '📊',
    category: 'export',
    hotkey: 'Ctrl+Shift+P',
    action: () => {
      console.log('Экспорт в PDF');
      // TODO: Реализовать экспорт в PDF
    },
    keywords: ['экспорт', 'pdf', 'развертки', 'export']
  },
  {
    id: 'file.export.svg',
    title: 'Export SVG',
    description: 'Экспортировать развертки в SVG',
    icon: '🖼️',
    category: 'export',
    hotkey: 'Ctrl+Shift+S',
    action: () => {
      console.log('Экспорт в SVG');
      // TODO: Реализовать экспорт в SVG
    },
    keywords: ['экспорт', 'svg', 'вектор', 'vector']
  },

  // Редактирование
  {
    id: 'edit.undo',
    title: 'Undo',
    description: 'Отменить последнее действие',
    icon: '↩️',
    category: 'edit',
    hotkey: 'Ctrl+Z',
    action: () => {
      console.log('Отмена действия');
      // TODO: Реализовать отмену действия
    },
    keywords: ['отмена', 'undo', 'назад']
  },
  {
    id: 'edit.redo',
    title: 'Redo',
    description: 'Повторить отмененное действие',
    icon: '↪️',
    category: 'edit',
    hotkey: 'Ctrl+Y',
    action: () => {
      console.log('Повтор действия');
      // TODO: Реализовать повтор действия
    },
    keywords: ['повтор', 'redo', 'вперед']
  },
  {
    id: 'edit.flip',
    title: 'Flip Part',
    description: 'Перевернуть выбранную деталь',
    icon: '🔄',
    category: 'edit',
    hotkey: 'F',
    action: () => {
      console.log('Переворот детали');
      // TODO: Реализовать переворот детали
    },
    keywords: ['перевернуть', 'flip', 'деталь', 'part']
  },
  {
    id: 'edit.rotate',
    title: 'Rotate Part',
    description: 'Повернуть выбранную деталь',
    icon: '🔄',
    category: 'edit',
    hotkey: 'R',
    action: () => {
      console.log('Поворот детали');
      // TODO: Реализовать поворот детали
    },
    keywords: ['повернуть', 'rotate', 'вращение']
  },

  // Вид
  {
    id: 'view.wireframe',
    title: 'Toggle Wireframe',
    description: 'Переключить режим каркаса',
    icon: '🔲',
    category: 'view',
    hotkey: 'W',
    action: () => {
      console.log('Переключение режима каркаса');
      // TODO: Реализовать переключение режима
    },
    keywords: ['каркас', 'wireframe', 'режим', 'toggle']
  },
  {
    id: 'view.textures',
    title: 'Toggle Textures',
    description: 'Включить/выключить текстуры',
    icon: '🎨',
    category: 'view',
    hotkey: 'T',
    action: () => {
      console.log('Переключение текстур');
      // TODO: Реализовать переключение текстур
    },
    keywords: ['текстуры', 'textures', 'материалы']
  },

  // Инструменты
  {
    id: 'tools.unfold',
    title: 'Unfold Model',
    description: 'Развернуть 3D модель в 2D',
    icon: '✂️',
    category: 'tools',
    hotkey: 'U',
    action: () => {
      console.log('Развертка модели');
      // TODO: Реализовать развертку модели
    },
    keywords: ['развертка', 'unfold', '2d', 'раскрой']
  },
  {
    id: 'tools.optimize',
    title: 'Optimize Layout',
    description: 'Оптимизировать расположение деталей на листе',
    icon: '📐',
    category: 'tools',
    hotkey: 'O',
    action: () => {
      console.log('Оптимизация расположения');
      // TODO: Реализовать оптимизацию
    },
    keywords: ['оптимизация', 'layout', 'расположение', 'nesting']
  },
  {
    id: 'tools.ai.generate',
    title: 'Generate AI Model',
    description: 'Сгенерировать 3D модель с помощью ИИ',
    icon: '🤖',
    category: 'tools',
    hotkey: 'Ctrl+G',
    action: () => {
      console.log('Генерация модели ИИ');
      // TODO: Реализовать генерацию модели
    },
    keywords: ['ai', 'ии', 'генерация', 'generate', 'модель']
  },

  // Настройки
  {
    id: 'settings.language',
    title: 'Change Language',
    description: 'Сменить язык интерфейса',
    icon: '🌐',
    category: 'settings',
    action: () => {
      console.log('Смена языка');
      // TODO: Реализовать смену языка
    },
    keywords: ['язык', 'language', 'локализация']
  },
  {
    id: 'settings.theme',
    title: 'Toggle Theme',
    description: 'Переключить светлую/темную тему',
    icon: '🌓',
    category: 'settings',
    hotkey: 'Ctrl+T',
    action: () => {
      console.log('Переключение темы');
      // TODO: Реализовать переключение темы
    },
    keywords: ['тема', 'theme', 'темная', 'светлая']
  },
  {
    id: 'settings.shortcuts',
    title: 'Keyboard Shortcuts',
    description: 'Показать список горячих клавиш',
    icon: '⌨️',
    category: 'settings',
    hotkey: 'Ctrl+Shift+K',
    action: () => {
      console.log('Показать горячие клавиши');
      // TODO: Реализовать отображение горячих клавиш
    },
    keywords: ['горячие клавиши', 'shortcuts', 'клавиатура']
  },

  // Помощь
  {
    id: 'help.docs',
    title: 'Open Documentation',
    description: 'Открыть документацию',
    icon: '📚',
    category: 'help',
    hotkey: 'F1',
    action: () => {
      console.log('Открытие документации');
      // TODO: Реализовать открытие документации
    },
    keywords: ['документация', 'docs', 'help', 'справка']
  },
  {
    id: 'help.about',
    title: 'About Pepakura Next',
    description: 'Информация о приложении',
    icon: 'ℹ️',
    category: 'help',
    action: () => {
      console.log('О программе');
      // TODO: Реализовать окно "О программе"
    },
    keywords: ['о программе', 'about', 'информация']
  }
];

// Регистрируем все базовые команды
baseCommands.forEach(cmd => commandRegistry.register(cmd));

// Вспомогательные функции
export function registerCommand(command: Command) {
  commandRegistry.register(command);
}

export function unregisterCommand(id: string) {
  commandRegistry.unregister(id);
}

export function executeCommand(id: string) {
  const cmd = commandRegistry.getCommand(id);
  if (cmd) {
    cmd.action();
  } else {
    console.warn(`Команда с ID "${id}" не найдена`);
  }
}

export function searchCommands(query: string): Command[] {
  return commandRegistry.search(query);
}

export function getCommandsByCategory(category: string): Command[] {
  return commandRegistry.getCommandsByCategory(category);
}

export function getAllCategories(): string[] {
  return commandRegistry.getCategories();
}