#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_conn_id_valid_uuid() {
    let valid_uuid = "550e8400-e29b-41d4-a716-446655440000";
    let result = validate_conn_id(valid_uuid);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), valid_uuid);
  }

  #[test]
  fn test_validate_conn_id_valid_uuid_v4() {
    let uuid_v4 = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
    let result = validate_conn_id(uuid_v4);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), uuid_v4);
  }

  #[test]
  fn test_validate_conn_id_invalid_uuid() {
    let invalid_uuids = [
      "not-a-uuid",
      "550e8400-e29b-41d4-a716",
      // Note: "550e8400e29b41d4a716446655440000" is 32 hex chars and parses as valid UUID
      "550e8400-e29b-41d4-a716-44665544000g", // 'g' is not valid hex
      "",
      "12345",
    ];
    for invalid in invalid_uuids {
      let result = validate_conn_id(invalid);
      assert!(result.is_err(), "Expected {} to be invalid", invalid);
      assert!(result.unwrap_err().contains("Invalid connection ID"));
    }
  }

  #[test]
  fn test_validate_conn_id_empty_string() {
    let result = validate_conn_id("");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid connection ID"));
  }
}

use uuid::Uuid;

pub fn validate_conn_id(conn_id: &str) -> Result<String, String> {
  Uuid::parse_str(conn_id)
    .map(|_| conn_id.to_string())
    .map_err(|_| format!("Invalid connection ID: {}", conn_id))
}
