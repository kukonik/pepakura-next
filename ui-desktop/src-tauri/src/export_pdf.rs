use crate::unfold::types::{UnfoldResult, UnfoldParams};

pub fn export_to_pdf(_result: &UnfoldResult, _params: &UnfoldParams, _output_dir: &str) -> std::io::Result<()> {
    // TODO: Реализовать экспорт в PDF с помощью правильной библиотеки (например, printpdf)
    eprintln!("Экспорт в PDF временно отключен. Для реализации требуется добавить библиотеку (например, printpdf).");
    Ok(()) // Возвращаем успех, чтобы не ломать поток выполнения
}
