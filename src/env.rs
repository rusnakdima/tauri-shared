//! Environment variable loading with compile-time defaults and .env support.

use once_cell::sync::Lazy;

/// Common environment configuration shared across all Tauri projects.
#[derive(Debug, Clone)]
pub struct EnvConfig {
  /// MongoDB connection URI (default: mongodb://localhost:27017)
  pub mongo_uri: String,
  /// MongoDB database name (default: taskflow)
  pub mongo_db_name: String,
  /// Schema database name (default: schemas)
  pub schema_db_name: String,
  /// JSON database name (default: app_data)
  pub jsondb_name: String,
  /// Application home folder (default: auto-detected)
  pub app_home_folder: String,
  /// Log level (default: info)
  pub log_level: String,
  /// Whether logging is enabled (default: true)
  pub log_enabled: bool,
  /// App name (default: from Cargo.toml)
  pub app_name: String,
  /// Environment (default: development)
  pub environment: String,
  /// JWT secret for token signing (no default - must be set)
  pub jwt_secret: String,
  /// SMTP username
  pub smtp_username: String,
  /// SMTP password
  pub smtp_password: String,
  /// SMTP server host
  pub smtp_server: String,
  /// SMTP server port (default: 587)
  pub smtp_port: u16,
  /// Reset token expiry hours (default: 1)
  pub reset_token_expiry_hours: u64,
  /// RP domain for OAuth (default: localhost)
  pub rp_domain: String,
  /// Enable query logging (default: false)
  pub enable_query_logging: bool,
  /// GitHub OAuth client ID
  pub client_id_github: String,
  /// GitHub OAuth client secret
  pub client_secret_github: String,
  /// GitHub OAuth callback URL
  pub callback_url_github: String,
}

impl Default for EnvConfig {
  fn default() -> Self {
    Self {
      mongo_uri: std::env::var("MONGODB_URI")
        .or_else(|_| std::env::var("MONGO_URI"))
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
      mongo_db_name: std::env::var("MONGODB_NAME").unwrap_or_else(|_| "taskflow".to_string()),
      schema_db_name: std::env::var("SCHEMA_DB_NAME").unwrap_or_else(|_| "schemas".to_string()),
      jsondb_name: std::env::var("JSONDB_NAME").unwrap_or_else(|_| "app_data".to_string()),
      app_home_folder: dirs::data_local_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "/tmp".to_string()),
      log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
      log_enabled: std::env::var("LOG_ENABLED")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(true),
      app_name: std::env::var("NAME_APP").unwrap_or_else(|_| "app".to_string()),
      environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
      jwt_secret: std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "DEV_ONLY_SECRET_DO_NOT_USE_IN_PRODUCTION".to_string()),
      smtp_username: std::env::var("SMTP_USERNAME").unwrap_or_else(|_| "".to_string()),
      smtp_password: std::env::var("SMTP_PASSWORD").unwrap_or_else(|_| "".to_string()),
      smtp_server: std::env::var("SMTP_SERVER").unwrap_or_else(|_| "smtp.example.com".to_string()),
      smtp_port: std::env::var("SMTP_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(587),
      reset_token_expiry_hours: std::env::var("RESET_TOKEN_EXPIRY_HOURS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1),
      rp_domain: std::env::var("RP_DOMAIN").unwrap_or_else(|_| "localhost".to_string()),
      enable_query_logging: std::env::var("ENABLE_QUERY_LOGGING")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false),
      client_id_github: std::env::var("CLIENT_ID_GITHUB").unwrap_or_else(|_| "".to_string()),
      client_secret_github: std::env::var("CLIENT_SECRET_GITHUB")
        .unwrap_or_else(|_| "".to_string()),
      callback_url_github: std::env::var("CALLBACK_URL_GITHUB").unwrap_or_else(|_| "".to_string()),
    }
  }
}

impl EnvConfig {
  /// Load environment from std::env vars with compile-time defaults.
  /// Also attempts to load from .env file if dotenv is available.
  pub fn load() -> Self {
    #[cfg(feature = "dotenvy")]
    let _ = dotenvy::dotenv();
    Self::default()
  }

  /// Get a typed environment variable with a default fallback.
  pub fn or(&self, key: &str, default: &'static str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
  }

  /// Get a typed environment variable.
  pub fn get<T: std::str::FromStr>(&self, key: &str) -> Option<T> {
    std::env::var(key).ok()?.parse().ok()
  }

  /// Check if running in a specific environment.
  pub fn is_environment(&self, env: &str) -> bool {
    self.environment == env
  }

  /// Check if running in development mode.
  pub fn is_development(&self) -> bool {
    self.is_environment("development")
  }

  /// Check if running in production mode.
  pub fn is_production(&self) -> bool {
    self.is_environment("production")
  }
}

/// Global environment config — initialized once at startup.
pub static ENV: Lazy<EnvConfig> = Lazy::new(EnvConfig::load);

/// Trait for project-specific environment configurations.
/// Each project implements this to declare its own env vars.
pub trait ProjectEnvConfig: Send + Sync {
  fn prefix(&self) -> &'static str;
  fn to_map(&self) -> std::collections::HashMap<String, String>;
  fn load() -> Self
  where
    Self: Sized;
}

/// Macro to load environment config at compile time for use in const contexts.
#[macro_export]
macro_rules! env_var {
  ($key:expr) => {
    std::env!($key)
  };
  ($key:expr, $default:expr) => {
    std::env::var($key).unwrap_or_else(|_| $default.to_string())
  };
}

#[cfg(feature = "dotenvy")]
pub fn init_env() {
  let _ = dotenvy::dotenv();
}

#[cfg(not(feature = "dotenvy"))]
pub fn init_env() {}
