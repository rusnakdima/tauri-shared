use crate::rbac::models::{Permission, Role, RolePermission, UserRole};
use nosql_orm::prelude::*;
use nosql_orm::providers::JsonProvider;

pub async fn rbac_list_roles(db: &JsonProvider) -> Result<Vec<Role>, String> {
  let roles = db.find_all("roles").await.map_err(|e| e.to_string())?;
  roles
    .into_iter()
    .map(|data| serde_json::from_value(data).map_err(|e| e.to_string()))
    .collect()
}

pub async fn rbac_create_role(
  db: &JsonProvider,
  name: String,
  description: String,
) -> Result<Role, String> {
  let role = Role::new(name, description);
  let role_value = serde_json::to_value(&role).map_err(|e| e.to_string())?;
  db.insert("roles", role_value)
    .await
    .map_err(|e| e.to_string())?;
  Ok(role)
}

pub async fn rbac_delete_role(db: &JsonProvider, role_id: String) -> Result<(), String> {
  db.delete("roles", &role_id)
    .await
    .map_err(|e| e.to_string())?;
  Ok(())
}

pub async fn rbac_list_permissions(db: &JsonProvider) -> Result<Vec<Permission>, String> {
  let permissions = db.find_all("permissions").await.map_err(|e| e.to_string())?;
  permissions
    .into_iter()
    .map(|data| serde_json::from_value(data).map_err(|e| e.to_string()))
    .collect()
}

pub async fn rbac_create_permission(
  db: &JsonProvider,
  name: String,
  resource: String,
  action: String,
) -> Result<Permission, String> {
  let permission = Permission::new(name, resource, action);
  let perm_value = serde_json::to_value(&permission).map_err(|e| e.to_string())?;
  db.insert("permissions", perm_value)
    .await
    .map_err(|e| e.to_string())?;
  Ok(permission)
}

pub async fn rbac_delete_permission(db: &JsonProvider, perm_id: String) -> Result<(), String> {
  db.delete("permissions", &perm_id)
    .await
    .map_err(|e| e.to_string())?;
  Ok(())
}

pub async fn rbac_assign_role_to_user(
  db: &JsonProvider,
  user_id: String,
  role_id: String,
) -> Result<UserRole, String> {
  let user_role = UserRole::new(user_id, role_id);
  let ur_value = serde_json::to_value(&user_role).map_err(|e| e.to_string())?;
  db.insert("user_roles", ur_value)
    .await
    .map_err(|e| e.to_string())?;
  Ok(user_role)
}

pub async fn rbac_remove_role_from_user(
  db: &JsonProvider,
  user_id: String,
  role_id: String,
) -> Result<(), String> {
  let user_roles = db.find_all("user_roles").await.map_err(|e| e.to_string())?;
  let to_delete = user_roles.iter().find(|ur| {
    ur.get("user_id").and_then(|v| v.as_str()) == Some(&user_id)
      && ur.get("role_id").and_then(|v| v.as_str()) == Some(&role_id)
  });

  if let Some(ur) = to_delete {
    if let Some(id) = ur.get("id").and_then(|v| v.as_str()) {
      db.delete("user_roles", id).await.map_err(|e| e.to_string())?;
    }
  }
  Ok(())
}

pub async fn rbac_grant_permission(
  db: &JsonProvider,
  role_id: String,
  perm_id: String,
) -> Result<RolePermission, String> {
  let role_perm = RolePermission::new(role_id, perm_id);
  let rp_value = serde_json::to_value(&role_perm).map_err(|e| e.to_string())?;
  db.insert("role_permissions", rp_value)
    .await
    .map_err(|e| e.to_string())?;
  Ok(role_perm)
}

pub async fn rbac_revoke_permission(
  db: &JsonProvider,
  role_id: String,
  perm_id: String,
) -> Result<(), String> {
  let role_perms = db.find_all("role_permissions").await.map_err(|e| e.to_string())?;
  let to_delete = role_perms.iter().find(|rp| {
    rp.get("role_id").and_then(|v| v.as_str()) == Some(&role_id)
      && rp.get("permission_id").and_then(|v| v.as_str()) == Some(&perm_id)
  });

  if let Some(rp) = to_delete {
    if let Some(id) = rp.get("id").and_then(|v| v.as_str()) {
      db.delete("role_permissions", id)
        .await
        .map_err(|e| e.to_string())?;
    }
  }
  Ok(())
}

pub async fn rbac_get_user_roles(db: &JsonProvider, user_id: String) -> Result<Vec<Role>, String> {
  let user_roles = db.find_all("user_roles").await.map_err(|e| e.to_string())?;
  let role_ids: Vec<String> = user_roles
    .iter()
    .filter(|ur| ur.get("user_id").and_then(|v| v.as_str()) == Some(&user_id))
    .filter_map(|ur| ur.get("role_id").and_then(|v| v.as_str()).map(String::from))
    .collect();

  let all_roles = db.find_all("roles").await.map_err(|e| e.to_string())?;
  all_roles
    .into_iter()
    .filter(|r| {
      r.get("id")
        .and_then(|v| v.as_str())
        .map(|id| role_ids.contains(&id.to_string()))
        .unwrap_or(false)
    })
    .map(|data| serde_json::from_value(data).map_err(|e| e.to_string()))
    .collect()
}

pub async fn rbac_get_role_permissions(
  db: &JsonProvider,
  role_id: String,
) -> Result<Vec<Permission>, String> {
  let role_perms = db.find_all("role_permissions").await.map_err(|e| e.to_string())?;
  let perm_ids: Vec<String> = role_perms
    .iter()
    .filter(|rp| rp.get("role_id").and_then(|v| v.as_str()) == Some(&role_id))
    .filter_map(|rp| {
      rp.get("permission_id")
        .and_then(|v| v.as_str())
        .map(String::from)
    })
    .collect();

  let all_perms = db.find_all("permissions").await.map_err(|e| e.to_string())?;
  all_perms
    .into_iter()
    .filter(|p| {
      p.get("id")
        .and_then(|v| v.as_str())
        .map(|id| perm_ids.contains(&id.to_string()))
        .unwrap_or(false)
    })
    .map(|data| serde_json::from_value(data).map_err(|e| e.to_string()))
    .collect()
}