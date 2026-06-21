use serde::Serialize;
use std::sync::OnceLock;

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
}

pub struct Logger {
    entries: std::sync::Mutex<Vec<LogEntry>>,
    max_entries: usize,
}

impl Logger {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: std::sync::Mutex::new(Vec::new()),
            max_entries,
        }
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
        };

        let mut entries = self.entries.lock().unwrap();
        entries.push(entry);

        if entries.len() > self.max_entries {
            entries.remove(0);
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
        tauri_shared::Logger::global().debug(&format!($($arg)*), Some(file!()));
    }};
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        tauri_shared::Logger::global().info(&format!($($arg)*), Some(file!()));
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        tauri_shared::Logger::global().warn(&format!($($arg)*), Some(file!()));
    }};
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        tauri_shared::Logger::global().error(&format!($($arg)*), Some(file!()));
    }};
}
