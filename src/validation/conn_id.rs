use uuid::Uuid;

pub fn validate_conn_id(conn_id: &str) -> Result<String, String> {
    Uuid::parse_str(conn_id)
        .map(|_| conn_id.to_string())
        .map_err(|_| format!("Invalid connection ID: {}", conn_id))
}
