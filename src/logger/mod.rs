mod file_writer;

pub use file_writer::FileLogger;

use serde::Serialize;
use std::sync::OnceLock;
use std::sync::{Arc, RwLock};

static LOGGER: OnceLock<Logger> = OnceLock::new();
static LOG_LEVEL: OnceLock<RwLock<LogLevel>> = OnceLock::new();

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "debug" => Some(LogLevel::Debug),
            "info" => Some(LogLevel::Info),
            "warn" | "warning" => Some(LogLevel::Warn),
            "error" => Some(LogLevel::Error),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
    }

    /// Returns true if this level should be logged given the current minimum level
    pub fn is_enabled(&self, min_level: LogLevel) -> bool {
        let self_val = match self {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Warn => 2,
            LogLevel::Error => 3,
        };
        let min_val = match min_level {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Warn => 2,
            LogLevel::Error => 3,
        };
        self_val >= min_val
    }
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

    /// Get the global log level filter
    pub fn global_level() -> &'static RwLock<LogLevel> {
        LOG_LEVEL.get_or_init(|| RwLock::new(LogLevel::Info))
    }

    /// Set the global log level filter
    pub fn set_level(level: LogLevel) {
        if let Ok(mut guard) = Self::global_level().write() {
            *guard = level;
        }
    }

    /// Get the current global log level
    pub fn get_level() -> LogLevel {
        Self::global_level().read().unwrap().clone()
    }

    pub fn log(&self, level: LogLevel, message: &str, source: Option<&str>) {
        // Check if this level should be logged based on current minimum level
        let min_level = Self::get_level();
        if !level.is_enabled(min_level) {
            return; // Skip logging below the minimum level
        }

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
