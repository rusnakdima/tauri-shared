pub fn validate_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    if name.len() > 255 {
        return Err("Name must be 255 characters or less".to_string());
    }
    if name.contains(['/', '\\', '\0', ';', '\'', '"', '`', '(', ')', ',']) {
        return Err("Name contains invalid characters".to_string());
    }
    let lower = name.to_lowercase();
    if lower.contains("drop ")
        || lower.contains("delete ")
        || lower.contains("insert ")
        || lower.contains("update ")
        || lower.contains("select ")
        || lower.contains("--")
        || lower.contains("/*")
    {
        return Err("Name contains invalid patterns".to_string());
    }
    Ok(())
}
