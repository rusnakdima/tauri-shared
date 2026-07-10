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