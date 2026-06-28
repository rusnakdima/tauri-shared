pub fn validate_sql(sql: &str) -> Result<(), String> {
    let trimmed = sql.trim();
    if trimmed.is_empty() {
        return Err("Empty SQL statement".to_string());
    }
    let upper = trimmed.to_uppercase();
    let allowed = [
        "SELECT", "INSERT", "UPDATE", "DELETE", "CREATE", "DROP", "ALTER", "SHOW", "USE",
        "DESCRIBE", "EXPLAIN",
    ];
    if !allowed.iter().any(|cmd| upper.starts_with(cmd)) {
        return Err("Only SELECT, INSERT, UPDATE, DELETE, CREATE, DROP, ALTER, SHOW, USE, DESCRIBE, EXPLAIN are allowed".to_string());
    }
    if trimmed.contains(';') {
        return Err("Multiple statements not allowed".to_string());
    }
    if trimmed.contains("--") || trimmed.contains("/*") || trimmed.contains("*/") {
        return Err("SQL comments not allowed".to_string());
    }
    Ok(())
}
