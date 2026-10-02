//! # Pepakura Debug CLI
//!
//! CLI-утилита для отладки и анализа 3D моделей Pepakura.
//!
//! ## Команды
//!
//! - `llm-status` - проверка статуса LLM-бэкенда
//! - `analyze <file>` - анализ модели с рекомендациями
//! - `unfold <file>` - подготовка к развёртке с LLM-помощью
//!
//! ## Примеры
//!
//! ```bash
//! # Проверить LLM
//! pepakura-debug llm-status
//!
//! # Анализировать модель
//! pepakura-debug analyze model.obj
//!
//! # Анализ с объяснением проблем
//! pepakura-debug analyze model.obj --explain
//!
//! # Развёртка с LLM-помощью
//! pepakura-debug unfold model.obj --explain
//! ```

mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pepakura-debug")]
#[command(author = "Pepakura Next Team")]
#[command(version = "0.1.0")]
#[command(about = "CLI-утилита для отладки и анализа 3D моделей Pepakura", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Уровень логирования (debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Проверка статуса LLM-бэкенда
    LlmStatus,

    /// Анализ модели с рекомендациями
    Analyze {
        /// Путь к файлу модели (.obj)
        #[arg(index = 1)]
        file: String,

        /// Показать объяснения проблем
        #[arg(short, long)]
        explain: bool,
    },

    /// Развёртка с LLM-объяснением проблем
    Unfold {
        /// Путь к файлу модели (.obj)
        #[arg(index = 1)]
        file: String,

        /// Показать объяснения проблем
        #[arg(short, long)]
        explain: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // Инициализируем логирование
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(&cli.log)
    ).init();

    // Выполняем команду
    let result = match cli.command {
        Commands::LlmStatus => {
            commands::cmd_llm_status()
        }
        Commands::Analyze { file, explain } => {
            commands::cmd_analyze(&file, explain)
        }
        Commands::Unfold { file, explain } => {
            commands::cmd_unfold(&file, explain)
        }
    };

    // Обработка ошибок
    if let Err(e) = result {
        eprintln!("\n❌ Ошибка: {}", e);
        std::process::exit(1);
    }
}
