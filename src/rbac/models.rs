#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_user_new() {
    let user = User::new(
      "testuser".to_string(),
      "hashed_password".to_string(),
      "test@example.com".to_string(),
    );
    assert_eq!(user.username, "testuser");
    assert_eq!(user.password_hash, "hashed_password");
    assert_eq!(user.email, "test@example.com");
    assert!(!user.id.is_empty());
    // Verify id is a valid UUID
    assert!(uuid::Uuid::parse_str(&user.id).is_ok());
  }

  #[test]
  fn test_user_new_unique_ids() {
    let user1 = User::new(
      "user1".to_string(),
      "hash1".to_string(),
      "a@b.com".to_string(),
    );
    let user2 = User::new(
      "user2".to_string(),
      "hash2".to_string(),
      "c@d.com".to_string(),
    );
    assert_ne!(user1.id, user2.id);
  }

  #[test]
  fn test_session_new() {
    let session = Session::new("user-123".to_string());
    assert_eq!(session.user_id, "user-123");
    assert!(!session.token.is_empty());
    // Verify token is a valid UUID
    assert!(uuid::Uuid::parse_str(&session.token).is_ok());
  }

  #[test]
  fn test_session_new_unique_tokens() {
    let session1 = Session::new("user-123".to_string());
    let session2 = Session::new("user-123".to_string());
    assert_ne!(session1.token, session2.token);
  }

  #[test]
  fn test_role_new() {
    let role = Role::new("admin".to_string(), "Administrator role".to_string());
    assert_eq!(role.name, "admin");
    assert_eq!(role.description, "Administrator role");
    assert!(!role.id.is_empty());
    assert!(uuid::Uuid::parse_str(&role.id).is_ok());
  }

  #[test]
  fn test_permission_new() {
    let permission = Permission::new(
      "read_users".to_string(),
      "users".to_string(),
      "read".to_string(),
    );
    assert_eq!(permission.name, "read_users");
    assert_eq!(permission.resource, "users");
    assert_eq!(permission.action, "read");
    assert!(!permission.id.is_empty());
    assert!(uuid::Uuid::parse_str(&permission.id).is_ok());
  }

  #[test]
  fn test_user_role_new() {
    let user_role = UserRole::new("user-123".to_string(), "role-456".to_string());
    assert_eq!(user_role.user_id, "user-123");
    assert_eq!(user_role.role_id, "role-456");
    assert!(!user_role.id.is_empty());
    assert!(uuid::Uuid::parse_str(&user_role.id).is_ok());
  }

  #[test]
  fn test_role_permission_new() {
    let role_perm = RolePermission::new("role-123".to_string(), "perm-456".to_string());
    assert_eq!(role_perm.role_id, "role-123");
    assert_eq!(role_perm.permission_id, "perm-456");
    assert!(!role_perm.id.is_empty());
    assert!(uuid::Uuid::parse_str(&role_perm.id).is_ok());
  }

  #[test]
  fn test_all_models_have_created_at() {
    let user = User::new("u".to_string(), "p".to_string(), "e@e.com".to_string());
    let session = Session::new("uid".to_string());
    let role = Role::new("r".to_string(), "d".to_string());
    let permission = Permission::new("n".to_string(), "res".to_string(), "act".to_string());
    let user_role = UserRole::new("uid".to_string(), "rid".to_string());
    let role_perm = RolePermission::new("rid".to_string(), "pid".to_string());

    assert!(user.created_at <= chrono::Utc::now());
    assert!(session.created_at <= chrono::Utc::now());
    assert!(role.created_at <= chrono::Utc::now());
    assert!(permission.created_at <= chrono::Utc::now());
    assert!(user_role.assigned_at <= chrono::Utc::now());
    assert!(role_perm.granted_at <= chrono::Utc::now());
  }

  #[test]
  fn test_user_serialization() {
    let user = User::new(
      "testuser".to_string(),
      "hash".to_string(),
      "test@example.com".to_string(),
    );
    let json = serde_json::to_string(&user).unwrap();
    assert!(json.contains("testuser"));
    assert!(json.contains("test@example.com"));
    assert!(json.contains("id"));
  }

  #[test]
  fn test_session_serialization() {
    let session = Session::new("user-123".to_string());
    let json = serde_json::to_string(&session).unwrap();
    assert!(json.contains("user-123"));
    assert!(json.contains("token"));
  }

  #[test]
  fn test_role_serialization() {
    let role = Role::new("admin".to_string(), "Administrator".to_string());
    let json = serde_json::to_string(&role).unwrap();
    assert!(json.contains("admin"));
    assert!(json.contains("Administrator"));
  }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  pub id: String,
  pub username: String,
  pub password_hash: String,
  pub email: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
  pub fn new(username: String, password_hash: String, email: String) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      username,
      password_hash,
      email,
      created_at: chrono::Utc::now(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
  pub token: String,
  pub user_id: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Session {
  pub fn new(user_id: String) -> Self {
    Self {
      token: Uuid::new_v4().to_string(),
      user_id,
      created_at: chrono::Utc::now(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
  pub id: String,
  pub name: String,
  pub description: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Role {
  pub fn new(name: String, description: String) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      name,
      description,
      created_at: chrono::Utc::now(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
  pub id: String,
  pub name: String,
  pub resource: String,
  pub action: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Permission {
  pub fn new(name: String, resource: String, action: String) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      name,
      resource,
      action,
      created_at: chrono::Utc::now(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
  pub id: String,
  pub user_id: String,
  pub role_id: String,
  pub assigned_at: chrono::DateTime<chrono::Utc>,
}

impl UserRole {
  pub fn new(user_id: String, role_id: String) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      user_id,
      role_id,
      assigned_at: chrono::Utc::now(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePermission {
  pub id: String,
  pub role_id: String,
  pub permission_id: String,
  pub granted_at: chrono::DateTime<chrono::Utc>,
}

impl RolePermission {
  pub fn new(role_id: String, permission_id: String) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      role_id,
      permission_id,
      granted_at: chrono::Utc::now(),
    }
  }
}
