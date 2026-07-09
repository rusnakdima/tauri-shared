use crate::rbac::{Permission, Role, RolePermission, UserRole};
use crate::AppError;
use nosql_orm::provider::DatabaseProvider;

pub async fn rbac_list_roles(db: &impl DatabaseProvider) -> Result<Vec<Role>, AppError> {
  let items = db.find_all("roles").await.map_err(AppError::from)?;
  let roles: Vec<Role> = items
    .into_iter()
    .map(|data| serde_json::from_value(data).map_err(|e| AppError::ValidationError(e.to_string())))
    .collect::<Result<Vec<_>, _>>()?;
  Ok(roles)
}

pub async fn rbac_create_role(
  db: &impl DatabaseProvider,
  name: String,
  description: String,
) -> Result<Role, AppError> {
  let id = uuid::Uuid::new_v4().to_string();
  let role = Role {
    id: id.clone(),
    name,
    description,
  };
  let data = serde_json::to_value(&role).map_err(|e| AppError::ValidationError(e.to_string()))?;
  db.insert("roles", data).await.map_err(AppError::from)?;
  Ok(role)
}

pub async fn rbac_delete_role(db: &impl DatabaseProvider, role_id: String) -> Result<(), AppError> {
  db.delete("roles", &role_id).await.map_err(AppError::from)?;
  Ok(())
}

pub async fn rbac_list_permissions(
  db: &impl DatabaseProvider,
) -> Result<Vec<Permission>, AppError> {
  let items = db.find_all("permissions").await.map_err(AppError::from)?;
  let permissions: Vec<Permission> = items
    .into_iter()
    .map(|data| serde_json::from_value(data).map_err(|e| AppError::ValidationError(e.to_string())))
    .collect::<Result<Vec<_>, _>>()?;
  Ok(permissions)
}

pub async fn rbac_create_permission(
  db: &impl DatabaseProvider,
  name: String,
  resource: String,
  action: String,
) -> Result<Permission, AppError> {
  let id = uuid::Uuid::new_v4().to_string();
  let permission = Permission {
    id: id.clone(),
    name,
    resource,
    action,
    fields: None,
    condition: None,
  };
  let data =
    serde_json::to_value(&permission).map_err(|e| AppError::ValidationError(e.to_string()))?;
  db.insert("permissions", data)
    .await
    .map_err(AppError::from)?;
  Ok(permission)
}

pub async fn rbac_delete_permission(
  db: &impl DatabaseProvider,
  perm_id: String,
) -> Result<(), AppError> {
  db.delete("permissions", &perm_id)
    .await
    .map_err(AppError::from)?;
  Ok(())
}

pub async fn rbac_assign_role_to_user(
  db: &impl DatabaseProvider,
  user_id: String,
  role_id: String,
) -> Result<UserRole, AppError> {
  let id = uuid::Uuid::new_v4().to_string();
  let granted_at = chrono::Utc::now().to_rfc3339();
  let user_role = UserRole {
    id: id.clone(),
    user_id,
    role_id,
    granted_by: String::new(),
    granted_at: Some(granted_at),
  };
  let data =
    serde_json::to_value(&user_role).map_err(|e| AppError::ValidationError(e.to_string()))?;
  db.insert("user_roles", data)
    .await
    .map_err(AppError::from)?;
  Ok(user_role)
}

pub async fn rbac_remove_role_from_user(
  db: &impl DatabaseProvider,
  user_id: String,
  role_id: String,
) -> Result<(), AppError> {
  let items = db.find_all("user_roles").await.map_err(AppError::from)?;
  let user_roles: Vec<UserRole> = items
    .into_iter()
    .map(|data| serde_json::from_value(data).map_err(|e| AppError::ValidationError(e.to_string())))
    .collect::<Result<Vec<_>, _>>()?;
  for ur in user_roles {
    if ur.user_id == user_id && ur.role_id == role_id {
      db.delete("user_roles", &ur.id)
        .await
        .map_err(AppError::from)?;
      return Ok(());
    }
  }
  Err(AppError::NotFound("UserRole not found".to_string()))
}

pub async fn rbac_grant_permission(
  db: &impl DatabaseProvider,
  role_id: String,
  perm_id: String,
) -> Result<RolePermission, AppError> {
  let id = uuid::Uuid::new_v4().to_string();
  let rp = RolePermission {
    id: id.clone(),
    role_id,
    permission_id: perm_id,
  };
  let data = serde_json::to_value(&rp).map_err(|e| AppError::ValidationError(e.to_string()))?;
  db.insert("role_permissions", data)
    .await
    .map_err(AppError::from)?;
  Ok(rp)
}

pub async fn rbac_revoke_permission(
  db: &impl DatabaseProvider,
  role_id: String,
  perm_id: String,
) -> Result<(), AppError> {
  let items = db
    .find_all("role_permissions")
    .await
    .map_err(AppError::from)?;
  let rps: Vec<RolePermission> = items
    .into_iter()
    .map(|data| serde_json::from_value(data).map_err(|e| AppError::ValidationError(e.to_string())))
    .collect::<Result<Vec<_>, _>>()?;
  for rp in rps {
    if rp.role_id == role_id && rp.permission_id == perm_id {
      db.delete("role_permissions", &rp.id)
        .await
        .map_err(AppError::from)?;
      return Ok(());
    }
  }
  Err(AppError::NotFound("RolePermission not found".to_string()))
}

pub async fn rbac_get_user_roles(
  db: &impl DatabaseProvider,
  user_id: String,
) -> Result<Vec<Role>, AppError> {
  let user_role_items = db.find_all("user_roles").await.map_err(AppError::from)?;
  let user_roles: Vec<UserRole> = user_role_items
    .into_iter()
    .map(|data| serde_json::from_value(data).map_err(|e| AppError::ValidationError(e.to_string())))
    .collect::<Result<Vec<_>, _>>()?;
  let role_items = db.find_all("roles").await.map_err(AppError::from)?;
  let all_roles: Vec<Role> = role_items
    .into_iter()
    .map(|data| serde_json::from_value(data).map_err(|e| AppError::ValidationError(e.to_string())))
    .collect::<Result<Vec<_>, _>>()?;
  let role_ids: std::collections::HashSet<_> = user_roles
    .iter()
    .filter(|ur| ur.user_id == user_id)
    .map(|ur| ur.role_id.clone())
    .collect();
  Ok(
    all_roles
      .into_iter()
      .filter(|r| role_ids.contains(&r.id))
      .collect(),
  )
}

pub async fn rbac_get_role_permissions(
  db: &impl DatabaseProvider,
  role_id: String,
) -> Result<Vec<Permission>, AppError> {
  let rp_items = db
    .find_all("role_permissions")
    .await
    .map_err(AppError::from)?;
  let role_perms: Vec<RolePermission> = rp_items
    .into_iter()
    .map(|data| serde_json::from_value(data).map_err(|e| AppError::ValidationError(e.to_string())))
    .collect::<Result<Vec<_>, _>>()?;
  let perm_items = db.find_all("permissions").await.map_err(AppError::from)?;
  let all_perms: Vec<Permission> = perm_items
    .into_iter()
    .map(|data| serde_json::from_value(data).map_err(|e| AppError::ValidationError(e.to_string())))
    .collect::<Result<Vec<_>, _>>()?;
  let perm_ids: std::collections::HashSet<_> = role_perms
    .iter()
    .filter(|rp| rp.role_id == role_id)
    .map(|rp| rp.permission_id.clone())
    .collect();
  Ok(
    all_perms
      .into_iter()
      .filter(|p| perm_ids.contains(&p.id))
      .collect(),
  )
}
