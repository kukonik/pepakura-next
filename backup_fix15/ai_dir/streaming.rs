//! Стриминг AI ответов.
//! 
//! Позволяет получать ответ от AI постепенно, по мере генерации.
//! Использует Server-Sent Events (SSE) подход.
//! 
//! ## Пример использования
//! 
//! ```rust,no_run
//! use pepakura_core::ai::{AiConfig, OllamaClient};
//! use pepakura_core::ai::streaming::chat_stream;
//! use futures::StreamExt;
//! 
//! async fn example() {
//!     let config = AiConfig::default();
//!     let client = OllamaClient::new(&config).unwrap();
//!     
//!     let mut stream = chat_stream(&client, "Привет!").await.unwrap();
//!     
//!     while let Some(token) = stream.next().await {
//!         print!("{}", token);
//!     }
//! }
//! ```

use crate::ai::{OllamaClient, ChatMessage};
use crate::PepakuraError;
use futures::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

/// Потоковый ответ от AI.
pub struct AiStream {
    pub receiver: ReceiverStream<String>,
}

impl Stream for AiStream {
    type Item = String;
    
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_next(cx)
    }
}

/// Отправляет запрос в AI и возвращает стрим токенов.
/// 
/// # Аргументы
/// * `client` - Ollama клиент
/// * `prompt` - промпт для генерации
/// 
/// # Возвращает
/// * `Ok(AiStream)` - стрим токенов
/// * `Err(PepakuraError)` - ошибка
pub async fn chat_stream(
    client: &OllamaClient,
    prompt: &str,
) -> Result<AiStream, PepakuraError> {
    let messages = vec![ChatMessage::user(prompt)];
    messages_stream(client, &messages).await
}

/// Отправляет запрос с историей сообщений и возвращает стрим.
/// 
/// # Аргументы
/// * `client` - Ollama клиент
/// * `messages` - история сообщений
/// 
/// # Возвращает
/// * `Ok(AiStream)` - стрим токенов
/// * `Err(PepakuraError)` - ошибка
pub async fn messages_stream(
    client: &OllamaClient,
    messages: &[ChatMessage],
) -> Result<AiStream, PepakuraError> {
    // Создаём канал для передачи токенов
    let (tx, rx) = mpsc::channel(32);
    
    // Клонируем данные для фонавой задачи
    let client_config = client.config.clone();
    let messages_vec = messages.to_vec();
    
    // Запускаем фоновую задачу для чтения стрима
    tokio::spawn(async move {
        let stream_result = stream_from_ollama(&client_config, &messages_vec, tx).await;
        
        if let Err(e) = stream_result {
            log::error!("Ошибка стриминга: {}", e);
        }
    });
    
    Ok(AiStream {
        receiver: ReceiverStream::new(rx),
    })
}

/// Читает стрим от Ollama и отправляет токены в канал.
async fn stream_from_ollama(
    config: &crate::ai::AiConfig,
    messages: &[ChatMessage],
    tx: mpsc::Sender<String>,
) -> Result<(), PepakuraError> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout_sec))
        .build()
        .map_err(|e| PepakuraError::AiError(format!("Failed to create HTTP client: {}", e)))?;
    
    let url = format!("{}/api/chat", config.ollama_url);
    
    let body = serde_json::json!({
        "model": config.model,
        "messages": messages,
        "stream": true,  // Включаем стриминг
        "options": {
            "temperature": config.temperature,
            "num_predict": config.max_tokens
        }
    });
    
    let mut response = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| PepakuraError::AiError(format!("Request failed: {}", e)))?;
    
    // Читаем стрим построчно
    while let Some(chunk) = response.chunk().await.map_err(|e| {
        PepakuraError::AiError(format!("Failed to read chunk: {}", e))
    })? {
        if chunk.is_empty() {
            continue;
        }
        
        // Парсим JSON ответ
        if let Ok(value) = serde_json::from_slice::<serde_json::Value>(&chunk) {
            if let Some(content) = value["message"]["content"].as_str() {
                // Отправляем токен в канал
                if tx.send(content.to_string()).await.is_err() {
                    // Получатель закрыл канал
                    break;
                }
            }
            
            // Проверяем флаг done
            if value["done"].as_bool() == Some(true) {
                break;
            }
        }
    }
    
    Ok(())
}

/// Собирает стрим в полную строку.
/// 
/// # Пример
/// 
/// ```rust
/// use pepakura_core::ai::streaming::collect_stream;
/// 
/// async fn example(stream: AiStream) {
///     let full_response = collect_stream(stream).await;
///     println!("{}", full_response);
/// }
/// ```
pub async fn collect_stream(stream: AiStream) -> String {
    use futures::StreamExt;
    
    let mut result = String::new();
    let mut stream = stream;
    
    while let Some(token) = stream.next().await {
        result.push_str(&token);
    }
    
    result
}

/// Стриминг с прогрессом.
pub struct ProgressStream {
    inner: AiStream,
    total_tokens: usize,
}

impl ProgressStream {
    /// Создаёт новый прогресс стрим.
    pub fn new(inner: AiStream) -> Self {
        Self {
            inner,
            total_tokens: 0,
        }
    }
    
    /// Возвращает количество полученных токенов.
    pub fn token_count(&self) -> usize {
        self.total_tokens
    }
}

impl Stream for ProgressStream {
    type Item = (String, usize);
    
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.inner).poll_next(cx) {
            Poll::Ready(Some(token)) => {
                self.total_tokens += 1;
                Poll::Ready(Some((token, self.total_tokens)))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Создаёт стрим с прогрессом.
pub fn with_progress(stream: AiStream) -> ProgressStream {
    ProgressStream::new(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    
    #[tokio::test]
    async fn test_collect_stream() {
        // Создаём тестовый стрим
        let (tx, rx) = mpsc::channel(32);
        
        // Отправляем несколько токенов
        tx.send("Hello".to_string()).await.unwrap();
        tx.send(" ".to_string()).await.unwrap();
        tx.send("World".to_string()).await.unwrap();
        drop(tx); // Закрываем канал
        
        let stream = AiStream {
            receiver: ReceiverStream::new(rx),
        };
        
        let result = collect_stream(stream).await;
        assert_eq!(result, "Hello World");
    }
    
    #[tokio::test]
    async fn test_progress_stream() {
        use futures::StreamExt;
        
        let (tx, rx) = mpsc::channel(32);
        
        tx.send("A".to_string()).await.unwrap();
        tx.send("B".to_string()).await.unwrap();
        tx.send("C".to_string()).await.unwrap();
        drop(tx);
        
        let stream = AiStream {
            receiver: ReceiverStream::new(rx),
        };
        
        let mut progress_stream = with_progress(stream);
        
        let mut tokens = Vec::new();
        while let Some((token, count)) = progress_stream.next().await {
            tokens.push((token, count));
        }
        
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], ("A".to_string(), 1));
        assert_eq!(tokens[1], ("B".to_string(), 2));
        assert_eq!(tokens[2], ("C".to_string(), 3));
    }
}
