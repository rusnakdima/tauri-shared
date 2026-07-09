use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use nosql_orm::prelude::Entity;
use nosql_orm::EntityMeta;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Role {
  pub id: String,
  pub name: String,
  #[serde(default)]
  pub description: String,
}

impl Entity for Role {
  fn meta() -> EntityMeta {
    EntityMeta::new("roles").with_id_field("id")
  }
  fn get_id(&self) -> Option<String> {
    Some(self.id.clone())
  }
  fn set_id(&mut self, id: String) {
    self.id = id;
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RoleDef {
  pub name: String,
  #[serde(default)]
  pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct UserRole {
  pub id: String,
  pub user_id: String,
  pub role_id: String,
  #[serde(default)]
  pub granted_by: String,
  #[serde(default)]
  pub granted_at: Option<String>,
}

impl Entity for UserRole {
  fn meta() -> EntityMeta {
    EntityMeta::new("user_roles").with_id_field("id")
  }
  fn get_id(&self) -> Option<String> {
    Some(self.id.clone())
  }
  fn set_id(&mut self, id: String) {
    self.id = id;
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Permission {
  pub id: String,
  pub name: String,
  pub resource: String,
  pub action: String,
  #[serde(default)]
  pub fields: Option<Vec<String>>,
  #[serde(default)]
  pub condition: Option<String>,
}

impl Entity for Permission {
  fn meta() -> EntityMeta {
    EntityMeta::new("permissions").with_id_field("id")
  }
  fn get_id(&self) -> Option<String> {
    Some(self.id.clone())
  }
  fn set_id(&mut self, id: String) {
    self.id = id;
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RolePermission {
  pub id: String,
  pub role_id: String,
  pub permission_id: String,
}

impl Entity for RolePermission {
  fn meta() -> EntityMeta {
    EntityMeta::new("role_permissions").with_id_field("id")
  }
  fn get_id(&self) -> Option<String> {
    Some(self.id.clone())
  }
  fn set_id(&mut self, id: String) {
    self.id = id;
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct AppUser {
  pub id: String,
  pub username: String,
  pub email: String,
  pub password_hash: String,
  #[serde(default)]
  pub roles: Vec<String>,
  pub created_at: String,
  pub updated_at: String,
}

impl Entity for AppUser {
  fn meta() -> EntityMeta {
    EntityMeta::new("app_users").with_id_field("id")
  }
  fn get_id(&self) -> Option<String> {
    Some(self.id.clone())
  }
  fn set_id(&mut self, id: String) {
    self.id = id;
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Session {
  pub id: String,
  pub token: String,
  pub user_id: String,
  pub expires_at: String,
  pub created_at: String,
}

impl Entity for Session {
  fn meta() -> EntityMeta {
    EntityMeta::new("sessions").with_id_field("id")
  }
  fn get_id(&self) -> Option<String> {
    Some(self.id.clone())
  }
  fn set_id(&mut self, id: String) {
    self.id = id;
  }
}

impl Role {
  pub fn new(id: &str, name: &str, description: &str) -> Self {
    Self {
      id: id.to_string(),
      name: name.to_string(),
      description: description.to_string(),
    }
  }
}

impl RoleDef {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      description: String::new(),
    }
  }

  pub fn with_description(mut self, description: &str) -> Self {
    self.description = description.to_string();
    self
  }
}

impl Permission {
  pub fn new(id: &str, name: &str, resource: &str, action: &str) -> Self {
    Self {
      id: id.to_string(),
      name: name.to_string(),
      resource: resource.to_string(),
      action: action.to_string(),
      fields: None,
      condition: None,
    }
  }

  pub fn matches(&self, resource: &str, action: &str) -> bool {
    self.resource == resource && self.action == action
  }

  pub fn matches_with_fields(&self, resource: &str, action: &str, fields: &[String]) -> bool {
    if !self.matches(resource, action) {
      return false;
    }
    if let Some(ref allowed) = self.fields {
      fields.iter().all(|f| allowed.contains(f))
    } else {
      true
    }
  }
}
