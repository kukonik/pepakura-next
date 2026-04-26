// index.js - Точка входа для веб-приложения
console.log('🚀 Запуск Pepakura Next веб-приложения...');

// Динамический импорт WASM модуля
async function loadWasm() {
    try {
        console.log('🔧 Загрузка WASM модуля...');
        
        // Импортируем init функцию и другие экспорты
        const { default: init, greet, version } = await import('../core/pkg/pepakura_next_core.js');
        
        console.log('⚙️ Инициализация WASM...');
        await init();
        
        console.log('✅ WASM успешно инициализирован!');
        
        // Пример использования функций из WASM
        const result = greet("Pepakura Next Developer");
        console.log('💬 Результат из WASM:', result);
        
        // Показываем информацию в UI
        const output = document.getElementById('output');
        if (output) {
            output.innerHTML = 
                <div style="font-size: 1.5rem; color: #4ecdc4; margin-bottom: 1rem;">✅ WASM УСПЕШНО ЗАГРУЖЕН!</div>
                <div style="margin: 1rem 0; padding: 1rem; background: rgba(78, 205, 196, 0.1); border-radius: 8px;">
                    <strong>Сообщение из Rust:</strong> 
                </div>
                <div style="margin: 1rem 0; padding: 1rem; background: rgba(78, 205, 196, 0.1); border-radius: 8px;">
                    <strong>Версия:</strong> 
                </div>
                <div style="margin-top: 1rem; font-size: 0.9rem; color: #aaa;">
                    💡 Теперь вы можете разрабатывать веб-интерфейс для вашего бумажного моделирования!
                </div>
            ;
        }
        
        // Небольшая анимация успеха
        document.querySelector('.status').className = 'status connected';
        document.querySelector('.status').textContent = '🟢 WASM готов!';
        
        return true;
    } catch (error) {
        console.error('❌ Критическая ошибка загрузки WASM:', error);
        return false;
    }
}

// Запускаем загрузку WASM
loadWasm().then(success => {
    if (!success) {
        // Повторная попытка через 3 секунды
        setTimeout(() => {
            console.log('🔄 Повторная попытка загрузки WASM...');
            loadWasm();
        }, 3000);
    }
});

// Горячая перезагрузка при изменении файлов (простая реализация)
if (import.meta.hot) {
    import.meta.hot.accept();
    console.log('🔥 Горячая перезагрузка активна!');
}
