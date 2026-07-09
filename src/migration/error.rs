#[derive(Debug)]
pub enum MigrationError {
  Provider(String),
  Entity(String),
}

impl std::fmt::Display for MigrationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      MigrationError::Provider(msg) => write!(f, "Migration error: {}", msg),
      MigrationError::Entity(msg) => write!(f, "Migration error: {}", msg),
    }
  }
}

impl std::error::Error for MigrationError {}
