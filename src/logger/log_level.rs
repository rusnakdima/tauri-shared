use serde::Serialize;

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
