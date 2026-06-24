mod file_writer;

pub use file_writer::FileLogger;

use serde::Serialize;
use std::sync::OnceLock;
use std::sync::{Arc, RwLock};

static LOGGER: OnceLock<Logger> = OnceLock::new();

#[derive(Debug, Clone, Serialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: String,
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}

pub struct Logger {
    entries: std::sync::Mutex<Vec<LogEntry>>,
    max_entries: usize,
    file_logger: Arc<RwLock<Option<FileLogger>>>,
}

impl Logger {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: std::sync::Mutex::new(Vec::new()),
            max_entries,
            file_logger: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_file_logger(self, path: std::path::PathBuf) -> Result<Self, std::io::Error> {
        *self.file_logger.write().unwrap() = Some(FileLogger::new(path)?);
        Ok(self)
    }

    pub fn global() -> &'static Logger {
        LOGGER.get_or_init(|| Logger::new(1000))
    }

    pub fn log(&self, level: LogLevel, message: &str, source: Option<&str>) {
        let entry = LogEntry {
            level,
            message: message.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            source: source.map(String::from),
            metadata: None,
        };

        let mut entries = self.entries.lock().unwrap();
        entries.push(entry.clone());

        if entries.len() > self.max_entries {
            entries.remove(0);
        }

        if let Ok(mut fl) = self.file_logger.write() {
            if let Some(ref mut file_logger) = *fl {
                let _ = file_logger.write(&entry);
            }
        }
    }

    pub fn debug(&self, message: &str, source: Option<&str>) {
        self.log(LogLevel::Debug, message, source);
    }

    pub fn info(&self, message: &str, source: Option<&str>) {
        self.log(LogLevel::Info, message, source);
    }

    pub fn warn(&self, message: &str, source: Option<&str>) {
        self.log(LogLevel::Warn, message, source);
    }

    pub fn error(&self, message: &str, source: Option<&str>) {
        self.log(LogLevel::Error, message, source);
    }

    pub fn get_entries(&self) -> Vec<LogEntry> {
        self.entries.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
    }
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {{
        $crate::Logger::global().debug(&format!($($arg)*), Some(file!()));
    }};
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        $crate::Logger::global().info(&format!($($arg)*), Some(file!()));
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        $crate::Logger::global().warn(&format!($($arg)*), Some(file!()));
    }};
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        $crate::Logger::global().error(&format!($($arg)*), Some(file!()));
    }};
}
