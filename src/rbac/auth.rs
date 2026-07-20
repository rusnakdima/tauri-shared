use crate::rbac::models::{Session, User};
use nosql_orm::prelude::*;
use nosql_orm::providers::JsonProvider;

pub async fn login(
  db: &JsonProvider,
  username: String,
  password: String,
) -> Result<Session, String> {
  let users = db.find_all("users").await.map_err(|e| e.to_string())?;
  let user_data = users
    .into_iter()
    .find(|u| u.get("username").and_then(|v| v.as_str()) == Some(&username));

  let user: User = match user_data {
    Some(data) => serde_json::from_value(data).map_err(|e| e.to_string())?,
    None => return Err("User not found".to_string()),
  };

  if !bcrypt::verify(&password, &user.password_hash).map_err(|e| e.to_string())? {
    return Err("Invalid password".to_string());
  }

  let session = Session::new(user.id);
  let session_value = serde_json::to_value(&session).map_err(|e| e.to_string())?;
  db.insert("sessions", session_value)
    .await
    .map_err(|e| e.to_string())?;

  Ok(session)
}

pub async fn logout(db: &JsonProvider, session_token: String) -> Result<(), String> {
  db.delete("sessions", &session_token)
    .await
    .map_err(|e| e.to_string())?;
  Ok(())
}

pub async fn register(
  db: &JsonProvider,
  username: String,
  password: String,
  email: String,
) -> Result<User, String> {
  let users = db.find_all("users").await.map_err(|e| e.to_string())?;
  if users
    .iter()
    .any(|u| u.get("username").and_then(|v| v.as_str()) == Some(&username))
  {
    return Err("Username already exists".to_string());
  }

  let password_hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST).map_err(|e| e.to_string())?;
  let user = User::new(username, password_hash, email);
  let user_value = serde_json::to_value(&user).map_err(|e| e.to_string())?;
  db.insert("users", user_value)
    .await
    .map_err(|e| e.to_string())?;

  Ok(user)
}

pub async fn get_current_user(db: &JsonProvider, session_token: String) -> Result<User, String> {
  let sessions = db.find_all("sessions").await.map_err(|e| e.to_string())?;
  let session_data = sessions
    .into_iter()
    .find(|s| s.get("token").and_then(|v| v.as_str()) == Some(&session_token));

  let session: Session = match session_data {
    Some(data) => serde_json::from_value(data).map_err(|e| e.to_string())?,
    None => return Err("Invalid session".to_string()),
  };

  let users = db.find_all("users").await.map_err(|e| e.to_string())?;
  let user_data = users
    .into_iter()
    .find(|u| u.get("id").and_then(|v| v.as_str()) == Some(&session.user_id));

  match user_data {
    Some(data) => serde_json::from_value(data).map_err(|e| e.to_string()),
    None => Err("User not found".to_string()),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::TempDir;

  #[tokio::test]
  async fn test_login_unknown_user() {
    let temp_dir = TempDir::new().unwrap();
    // JsonProvider::new expects a directory path, it creates users.json, sessions.json etc. inside
    let provider = JsonProvider::new(temp_dir.path().to_str().unwrap())
      .await
      .unwrap();
    let result = login(
      &provider,
      "nobody@example.com".to_string(),
      "wrong".to_string(),
    )
    .await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "User not found");
  }

  #[tokio::test]
  async fn test_register_and_login() {
    let temp_dir = TempDir::new().unwrap();
    let provider = JsonProvider::new(temp_dir.path().to_str().unwrap())
      .await
      .unwrap();
    let reg_result = register(
      &provider,
      "testuser".to_string(),
      "password123".to_string(),
      "test@example.com".to_string(),
    )
    .await;
    assert!(reg_result.is_ok());
    let login_result = login(&provider, "testuser".to_string(), "password123".to_string()).await;
    assert!(login_result.is_ok());
  }
}
