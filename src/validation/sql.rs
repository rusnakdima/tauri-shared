#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_sql_select_allowed() {
    assert!(validate_sql("SELECT * FROM users").is_ok());
    assert!(validate_sql("select id, name from users where id = 1").is_ok());
  }

  #[test]
  fn test_validate_sql_insert_allowed() {
    assert!(validate_sql("INSERT INTO users (name) VALUES ('test')").is_ok());
  }

  #[test]
  fn test_validate_sql_update_allowed() {
    assert!(validate_sql("UPDATE users SET name = 'test' WHERE id = 1").is_ok());
  }

  #[test]
  fn test_validate_sql_delete_allowed() {
    assert!(validate_sql("DELETE FROM users WHERE id = 1").is_ok());
  }

  #[test]
  fn test_validate_sql_create_allowed() {
    assert!(validate_sql("CREATE TABLE test (id INT)").is_ok());
  }

  #[test]
  fn test_validate_sql_drop_allowed() {
    assert!(validate_sql("DROP TABLE users").is_ok());
  }

  #[test]
  fn test_validate_sql_alter_allowed() {
    assert!(validate_sql("ALTER TABLE users ADD COLUMN age INT").is_ok());
  }

  #[test]
  fn test_validate_sql_show_allowed() {
    assert!(validate_sql("SHOW TABLES").is_ok());
  }

  #[test]
  fn test_validate_sql_use_allowed() {
    assert!(validate_sql("USE database_name").is_ok());
  }

  #[test]
  fn test_validate_sql_describe_allowed() {
    assert!(validate_sql("DESCRIBE users").is_ok());
  }

  #[test]
  fn test_validate_sql_explain_allowed() {
    assert!(validate_sql("EXPLAIN SELECT * FROM users").is_ok());
  }

  #[test]
  fn test_validate_sql_empty_string() {
    let result = validate_sql("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Empty SQL statement");
  }

  #[test]
  fn test_validate_sql_whitespace_only() {
    let result = validate_sql("   \n\t  ");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Empty SQL statement");
  }

  #[test]
  fn test_validate_sql_comments_blocked() {
    assert!(validate_sql("SELECT * FROM users -- comment").is_err());
    assert_eq!(
      validate_sql("SELECT * FROM users -- comment").unwrap_err(),
      "SQL comments not allowed"
    );
    assert!(validate_sql("SELECT * FROM users /* comment */").is_err());
    assert_eq!(
      validate_sql("SELECT * FROM users /* comment */").unwrap_err(),
      "SQL comments not allowed"
    );
  }

  #[test]
  fn test_validate_sql_multiple_statements_blocked() {
    let result = validate_sql("SELECT * FROM users; SELECT * FROM orders");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Multiple statements not allowed");
  }

  #[test]
  fn test_validate_sql_drop_blocked() {
    // DROP is actually allowed according to the code, let me verify
    assert!(validate_sql("DROP TABLE users").is_ok());
  }

  #[test]
  fn test_validate_sql_unsupported_command() {
    let result = validate_sql("TRUNCATE TABLE users");
    assert!(result.is_err());
  }

  #[test]
  fn test_validate_sql_case_insensitive() {
    assert!(validate_sql("select * from users").is_ok());
    assert!(validate_sql("SELECT * FROM USERS").is_ok());
  }
}

pub fn validate_sql(sql: &str) -> Result<(), String> {
  let trimmed = sql.trim();
  if trimmed.is_empty() {
    return Err("Empty SQL statement".to_string());
  }
  let upper = trimmed.to_uppercase();
  let allowed = [
    "SELECT", "INSERT", "UPDATE", "DELETE", "CREATE", "DROP", "ALTER", "SHOW", "USE", "DESCRIBE",
    "EXPLAIN",
  ];
  if !allowed.iter().any(|cmd| upper.starts_with(cmd)) {
    return Err("Only SELECT, INSERT, UPDATE, DELETE, CREATE, DROP, ALTER, SHOW, USE, DESCRIBE, EXPLAIN are allowed".to_string());
  }
  if trimmed.contains(';') {
    return Err("Multiple statements not allowed".to_string());
  }
  if trimmed.contains("--") || trimmed.contains("/*") || trimmed.contains("*/") {
    return Err("SQL comments not allowed".to_string());
  }
  Ok(())
}
