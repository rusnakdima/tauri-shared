use crate::rbac::{AppUser, Session};
use crate::AppError;
use nosql_orm::provider::DatabaseProvider;

pub fn hash_password(password: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    format!("zenith_hash_{}", hasher.finish())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    hash_password(password) == hash
}

fn generate_token(user_id: &str, username: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    let timestamp = chrono::Utc::now().timestamp();
    user_id.hash(&mut hasher);
    username.hash(&mut hasher);
    timestamp.hash(&mut hasher);
    format!("zenith_token_{}", hasher.finish())
}

pub async fn login(
    db: &impl DatabaseProvider,
    username: String,
    password: String,
) -> Result<Session, AppError> {
    let items = db.find_all("app_users").await.map_err(AppError::from)?;
    let users: Vec<AppUser> = items
        .into_iter()
        .map(|data| {
            serde_json::from_value(data).map_err(|e| AppError::ValidationError(e.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let user = users
        .into_iter()
        .find(|u| u.username == username)
        .ok_or_else(|| AppError::Unauthorized)?;

    if !verify_password(&password, &user.password_hash) {
        return Err(AppError::Unauthorized);
    }

    let token = generate_token(&user.id, &user.username);
    let expires_at = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .to_rfc3339();
    let session_id = uuid::Uuid::new_v4().to_string();

    let session = Session {
        id: session_id.clone(),
        token,
        user_id: user.id,
        expires_at,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    let data =
        serde_json::to_value(&session).map_err(|e| AppError::ValidationError(e.to_string()))?;
    db.insert("sessions", data).await.map_err(AppError::from)?;

    Ok(session)
}

pub async fn logout(db: &impl DatabaseProvider, session_token: String) -> Result<(), AppError> {
    let items = db.find_all("sessions").await.map_err(AppError::from)?;
    let sessions: Vec<Session> = items
        .into_iter()
        .map(|data| {
            serde_json::from_value(data).map_err(|e| AppError::ValidationError(e.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    for session in sessions {
        if session.token == session_token {
            db.delete("sessions", &session.id)
                .await
                .map_err(AppError::from)?;
            return Ok(());
        }
    }
    Err(AppError::NotFound("Session not found".to_string()))
}

pub async fn register(
    db: &impl DatabaseProvider,
    username: String,
    password: String,
    email: String,
) -> Result<AppUser, AppError> {
    if username.is_empty() || email.is_empty() || password.is_empty() {
        return Err(AppError::ValidationError(
            "Username, email and password are required".to_string(),
        ));
    }

    let items = db.find_all("app_users").await.map_err(AppError::from)?;
    let existing: Vec<AppUser> = items
        .into_iter()
        .map(|data| {
            serde_json::from_value(data).map_err(|e| AppError::ValidationError(e.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    if existing.iter().any(|u| u.username == username) {
        return Err(AppError::Duplicate("User".to_string()));
    }
    if existing.iter().any(|u| u.email == email) {
        return Err(AppError::Duplicate("User".to_string()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let password_hash = hash_password(&password);

    let user = AppUser {
        id: id.clone(),
        username: username.clone(),
        email: email.clone(),
        password_hash,
        roles: vec!["viewer".to_string()],
        created_at: now.clone(),
        updated_at: now.clone(),
    };

    let data = serde_json::to_value(&user).map_err(|e| AppError::ValidationError(e.to_string()))?;
    db.insert("app_users", data).await.map_err(AppError::from)?;

    Ok(user)
}

pub async fn get_current_user(
    db: &impl DatabaseProvider,
    session_token: String,
) -> Result<AppUser, AppError> {
    let session_items = db.find_all("sessions").await.map_err(AppError::from)?;
    let sessions: Vec<Session> = session_items
        .into_iter()
        .map(|data| {
            serde_json::from_value(data).map_err(|e| AppError::ValidationError(e.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let session = sessions
        .into_iter()
        .find(|s| s.token == session_token)
        .ok_or_else(|| AppError::Unauthorized)?;

    let user_items = db.find_all("app_users").await.map_err(AppError::from)?;
    let users: Vec<AppUser> = user_items
        .into_iter()
        .map(|data| {
            serde_json::from_value(data).map_err(|e| AppError::ValidationError(e.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    users
        .into_iter()
        .find(|u| u.id == session.user_id)
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))
}
