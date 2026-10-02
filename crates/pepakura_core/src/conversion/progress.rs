//! Отслеживание прогресса конвертации

use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Атомарный трекер прогресса для многопоточной конвертации
#[derive(Debug, Clone)]
pub struct ProgressTracker {
    inner: Arc<ProgressInner>,
}

#[derive(Debug)]
struct ProgressInner {
    current: AtomicUsize,
    total: AtomicUsize,
    cancelled: AtomicBool,
    description: std::sync::RwLock<String>,
}

impl ProgressTracker {
    /// Создает новый трекер прогресса
    pub fn new(total: usize) -> Self {
        Self {
            inner: Arc::new(ProgressInner {
                current: AtomicUsize::new(0),
                total: AtomicUsize::new(total),
                cancelled: AtomicBool::new(false),
                description: std::sync::RwLock::new(String::new()),
            }),
        }
    }

    /// Увеличивает счетчик прогресса
    pub fn increment(&self) {
        self.inner.current.fetch_add(1, Ordering::Relaxed);
    }

    /// Увеличивает счетчик на значение
    pub fn add(&self, value: usize) {
        self.inner.current.fetch_add(value, Ordering::Relaxed);
    }

    /// Устанавливает текущее значение
    pub fn set_current(&self, value: usize) {
        self.inner.current.store(value, Ordering::Relaxed);
    }

    /// Возвращает текущий прогресс
    pub fn current(&self) -> usize {
        self.inner.current.load(Ordering::Relaxed)
    }

    /// Возвращает общее количество
    pub fn total(&self) -> usize {
        self.inner.total.load(Ordering::Relaxed)
    }

    /// Возвращает процент выполнения (0.0 - 100.0)
    pub fn percent(&self) -> f32 {
        let current = self.current();
        let total = self.total();
        if total == 0 {
            0.0
        } else {
            (current as f32 / total as f32) * 100.0
        }
    }

    /// Устанавливает описание текущего этапа
    pub fn set_description(&self, description: impl Into<String>) {
        if let Ok(mut desc) = self.inner.description.write() {
            *desc = description.into();
        }
    }

    /// Возвращает описание
    pub fn description(&self) -> String {
        self.inner.description.read().unwrap().clone()
    }

    /// Отменяет операцию
    pub fn cancel(&self) {
        self.inner.cancelled.store(true, Ordering::Relaxed);
    }

    /// Проверяет, была ли операция отменена
    pub fn is_cancelled(&self) -> bool {
        self.inner.cancelled.load(Ordering::Relaxed)
    }

    /// Проверяет завершение
    pub fn is_complete(&self) -> bool {
        self.current() >= self.total()
    }

    /// Сбрасывает прогресс
    pub fn reset(&self, new_total: Option<usize>) {
        self.inner.current.store(0, Ordering::Relaxed);
        if let Some(total) = new_total {
            self.inner.total.store(total, Ordering::Relaxed);
        }
    }
}

/// Callback для отслеживания прогресса
pub type ProgressCallback = Box<dyn FnMut(f32, &str) + Send + Sync>;

/// Builder для создания ProgressTracker
pub struct ProgressTrackerBuilder {
    total: usize,
    description: String,
    callbacks: Vec<ProgressCallback>,
}

impl ProgressTrackerBuilder {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            description: String::new(),
            callbacks: vec![],
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn with_callback<F>(mut self, callback: F) -> Self
    where
        F: FnMut(f32, &str) + Send + Sync + 'static,
    {
        self.callbacks.push(Box::new(callback));
        self
    }

    pub fn build(self) -> ProgressTrackerWithCallbacks {
        let tracker = ProgressTracker::new(self.total);
        tracker.set_description(&self.description);

        ProgressTrackerWithCallbacks {
            tracker,
            callbacks: Mutex::new(self.callbacks),
            last_reported: AtomicUsize::new(0),
            report_interval: (self.total / 100).max(1), // Report every 1%
        }
    }
}

/// ProgressTracker с callback'ами
pub struct ProgressTrackerWithCallbacks {
    tracker: ProgressTracker,
    callbacks: Mutex<Vec<ProgressCallback>>,
    last_reported: AtomicUsize,
    report_interval: usize,
}

impl ProgressTrackerWithCallbacks {
    pub fn tracker(&self) -> &ProgressTracker {
        &self.tracker
    }

    /// Вызывает callback'и если прогресс изменился достаточно
    pub fn maybe_report(&self) {
        let current = self.tracker.current();
        let last = self.last_reported.load(Ordering::Relaxed);

        if current - last >= self.report_interval || self.tracker.is_complete() {
            self.last_reported.store(current, Ordering::Relaxed);

            let percent = self.tracker.percent();
            let description = self.tracker.description();

            if let Ok(mut callbacks) = self.callbacks.lock() {
                for callback in callbacks.iter_mut() {
                    callback(percent, &description);
                }
            }
        }
    }

    /// Финальный репорт
    pub fn report_final(&self) {
        let percent = 100.0;
        let description = self.tracker.description();

        if let Ok(mut callbacks) = self.callbacks.lock() {
            for callback in callbacks.iter_mut() {
                callback(percent, &description);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU32;
    use std::sync::Arc;

    #[test]
    fn test_progress_tracker_basic() {
        let tracker = ProgressTracker::new(100);
        assert_eq!(tracker.percent(), 0.0);

        for _ in 0..50 {
            tracker.increment();
        }
        assert_eq!(tracker.percent(), 50.0);

        tracker.add(25);
        assert_eq!(tracker.percent(), 75.0);
    }

    #[test]
    fn test_progress_tracker_cancel() {
        let tracker = ProgressTracker::new(100);
        assert!(!tracker.is_cancelled());

        tracker.cancel();
        assert!(tracker.is_cancelled());
    }

    #[test]
    fn test_progress_tracker_callbacks() {
        let call_count = Arc::new(AtomicU32::new(0));
        let call_count_clone = call_count.clone();

        let tracker = ProgressTrackerBuilder::new(100)
            .with_description("Test")
            .with_callback(move |_, _| {
                call_count_clone.fetch_add(1, Ordering::Relaxed);
            })
            .build();

        // Initial state
        assert_eq!(call_count.load(Ordering::Relaxed), 0);

        // Trigger report
        tracker.tracker().add(10);
        tracker.maybe_report();

        assert!(call_count.load(Ordering::Relaxed) > 0);
    }

    #[test]
    fn test_progress_tracker_complete() {
        let tracker = ProgressTracker::new(10);
        assert!(!tracker.is_complete());

        tracker.set_current(10);
        assert!(tracker.is_complete());
    }
}
