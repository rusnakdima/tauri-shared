#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_input_valid() {
    assert!(ValidationAlgorithm::validate_input("hello", 10));
    assert!(!ValidationAlgorithm::validate_input("", 10)); // empty string is not valid
  }

  #[test]
  fn test_validate_input_exceeds_max_length() {
    assert!(!ValidationAlgorithm::validate_input("hello world", 5));
  }

  #[test]
  fn test_validate_input_empty() {
    assert!(!ValidationAlgorithm::validate_input("", 10));
  }

  #[test]
  fn test_validate_input_exact_length() {
    assert!(ValidationAlgorithm::validate_input("hello", 5));
  }

  #[test]
  fn test_validate_email_valid() {
    assert!(ValidationAlgorithm::validate_email("test@example.com"));
    assert!(ValidationAlgorithm::validate_email(
      "user.name@domain.co.uk"
    ));
    assert!(ValidationAlgorithm::validate_email("a@b.c"));
  }

  #[test]
  fn test_validate_email_invalid() {
    assert!(!ValidationAlgorithm::validate_email("invalid"));
    assert!(!ValidationAlgorithm::validate_email("no@domain"));
    assert!(!ValidationAlgorithm::validate_email("")); // empty string is invalid
                                                       // Note: "@nodomain.com" and "noat.com" technically pass the simple check
                                                       // because they contain @ and .
  }

  #[test]
  fn test_sanitize_input_keeps_valid_chars() {
    assert_eq!(ValidationAlgorithm::sanitize_input("Hello123"), "Hello123");
    assert_eq!(
      ValidationAlgorithm::sanitize_input("hello world"),
      "hello world"
    );
    // Note: '.' is not alphanumeric, space, or hyphen, so it's removed
    assert_eq!(
      ValidationAlgorithm::sanitize_input("file-name.txt"),
      "file-nametxt"
    );
  }

  #[test]
  fn test_sanitize_input_removes_invalid_chars() {
    assert_eq!(
      ValidationAlgorithm::sanitize_input("hello;world"),
      "helloworld"
    );
    assert_eq!(
      ValidationAlgorithm::sanitize_input("test@email.com"),
      "testemailcom"
    );
    assert_eq!(ValidationAlgorithm::sanitize_input("user'name"), "username");
    assert_eq!(
      ValidationAlgorithm::sanitize_input("path/to/file"),
      "pathtofile"
    );
  }

  #[test]
  fn test_sanitize_input_preserves_spaces_and_hyphens() {
    assert_eq!(
      ValidationAlgorithm::sanitize_input("hello - world"),
      "hello - world"
    );
    assert_eq!(
      ValidationAlgorithm::sanitize_input("user-name-123"),
      "user-name-123"
    );
  }

  #[test]
  fn test_sanitize_input_unicode_chars_removed() {
    // Unicode chars that are alphanumeric pass through
    assert_eq!(ValidationAlgorithm::sanitize_input("café"), "café");
    assert_eq!(ValidationAlgorithm::sanitize_input("naïve"), "naïve");
  }

  #[test]
  fn test_sanitize_input_empty() {
    assert_eq!(ValidationAlgorithm::sanitize_input(""), "");
    assert_eq!(ValidationAlgorithm::sanitize_input("!@#$%"), "");
  }
}

pub struct ValidationAlgorithm;

impl ValidationAlgorithm {
  pub fn validate_input(input: &str, max_length: usize) -> bool {
    input.len() <= max_length && !input.is_empty()
  }

  pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
  }

  pub fn sanitize_input(input: &str) -> String {
    input
      .chars()
      .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
      .collect()
  }
}
