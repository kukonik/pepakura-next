/**
 * Тесты для реестра команд
 */

import { commandRegistry, baseCommands, searchCommands, executeCommand } from '../commands.registry'

describe('Command Registry', () => {
  beforeEach(() => {
    // Очищаем реестр перед каждым тестом
    // В реальном приложении нужно пересоздать экземпляр
    baseCommands.forEach(cmd => commandRegistry.register(cmd))
  })

  test('should register and retrieve commands', () => {
    const command = commandRegistry.getCommand('file.new')
    expect(command).toBeDefined()
    expect(command?.title).toBe('New Project')
  })

  test('should search commands by query', () => {
    const results = searchCommands('export')
    expect(results.length).toBeGreaterThan(0)
    expect(results.some(cmd => cmd.title.includes('Export'))).toBe(true)
  })

  test('should get commands by category', () => {
    const fileCommands = commandRegistry.getCommandsByCategory('file')
    expect(fileCommands.length).toBeGreaterThan(0)
    expect(fileCommands.every(cmd => cmd.category === 'file')).toBe(true)
  })

  test('should execute command', () => {
    const consoleSpy = jest.spyOn(console, 'log').mockImplementation()
    
    executeCommand('file.new')
    
    expect(consoleSpy).toHaveBeenCalledWith('Создание нового проекта')
    
    consoleSpy.mockRestore()
  })

  test('should handle non-existent command', () => {
    const consoleSpy = jest.spyOn(console, 'warn').mockImplementation()
    
    executeCommand('non.existent')
    
    expect(consoleSpy).toHaveBeenCalledWith('Команда с ID "non.existent" не найдена')
    
    consoleSpy.mockRestore()
  })
})