#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_name_empty_string() {
    let result = validate_name("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Name cannot be empty");
  }

  #[test]
  fn test_validate_name_too_long() {
    let long_name = "a".repeat(256);
    let result = validate_name(&long_name);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Name must be 255 characters or less");
  }

  #[test]
  fn test_validate_name_max_length() {
    let max_name = "a".repeat(255);
    let result = validate_name(&max_name);
    assert!(result.is_ok());
  }

  #[test]
  fn test_validate_name_invalid_characters() {
    let invalid_chars = ['/', '\\', '\0', ';', '\'', '"', '`', '(', ')', ','];
    for c in invalid_chars {
      let name = format!("test{}name", c);
      let result = validate_name(&name);
      assert!(result.is_err());
      assert_eq!(result.unwrap_err(), "Name contains invalid characters");
    }
  }

  #[test]
  fn test_validate_name_sql_injection_patterns() {
    // Test that SQL comment patterns are blocked
    // Note: "--" appears as invalid patterns check
    let name1 = "test--name";
    let result1 = validate_name(name1);
    assert!(result1.is_err());
    // "--" is detected as pattern
    assert_eq!(result1.unwrap_err(), "Name contains invalid patterns");
  }

  #[test]
  fn test_validate_name_valid_names() {
    assert!(validate_name("valid_name").is_ok());
    assert!(validate_name("valid-name").is_ok());
    assert!(validate_name("ValidName123").is_ok());
    assert!(validate_name("user.name").is_ok());
    assert!(validate_name("name with spaces").is_ok());
  }

  #[test]
  fn test_validate_name_case_insensitive_patterns() {
    // Note: "DROP table" passes character check but fails pattern check
    let result = validate_name("DROP table");
    assert!(result.is_err());
    // It contains "drop " as substring in lowercase check
    assert_eq!(result.unwrap_err(), "Name contains invalid patterns");
  }
}

pub fn validate_name(name: &str) -> Result<(), String> {
  if name.is_empty() {
    return Err("Name cannot be empty".to_string());
  }
  if name.len() > 255 {
    return Err("Name must be 255 characters or less".to_string());
  }
  if name.contains(['/', '\\', '\0', ';', '\'', '"', '`', '(', ')', ',']) {
    return Err("Name contains invalid characters".to_string());
  }
  let lower = name.to_lowercase();
  if lower.contains("drop ")
    || lower.contains("delete ")
    || lower.contains("insert ")
    || lower.contains("update ")
    || lower.contains("select ")
    || lower.contains("--")
    || lower.contains("/*")
  {
    return Err("Name contains invalid patterns".to_string());
  }
  Ok(())
}
