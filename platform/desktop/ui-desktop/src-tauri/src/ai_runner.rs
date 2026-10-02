//! Запуск локального AI-оркестратора в фоне

use std::thread;

pub fn start_ai_orchestrator() {
    thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime for AI");
        rt.block_on(async {
            crate::orchestrator::run_ai_server().await
        });
    });
}
