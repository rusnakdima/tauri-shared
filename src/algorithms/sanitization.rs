use regex::Regex;

/// Sanitize JSON by removing MongoDB operator keys (keys starting with $)
pub fn sanitize_for_mongo(value: &mut serde_json::Value) {
  if let serde_json::Value::Object(obj) = value {
    obj.retain(|k, _| !k.starts_with('$'));
    for v in obj.values_mut() {
      sanitize_for_mongo(v);
    }
  }
}

/// Escape HTML characters for safe display (XSS prevention)
pub fn escape_html(input: &str) -> String {
  input
    .replace('&', "&amp;")
    .replace('<', "&lt;")
    .replace('>', "&gt;")
    .replace('"', "&quot;")
    .replace('\'', "&#39;")
}

/// Strip URLs from text (for chat filtering)
pub fn strip_urls(input: &str) -> String {
  let url_regex = Regex::new(r"https?://\S+").unwrap();
  url_regex.replace_all(input, "[removed]").to_string()
}

/// Cap string to maximum length, preserving UTF-8 character boundaries
pub fn cap_string(s: &str, max_len: usize) -> String {
  if s.len() <= max_len {
    s.to_string()
  } else {
    s.chars().take(max_len).collect()
  }
}

/// Sanitize chat text specifically for overlay rendering.
///
/// Order matters: strip URLs BEFORE escaping HTML to prevent entity reconstruction attacks.
pub fn sanitize_for_overlay(text: &str, max_len: usize) -> String {
  let without_links = strip_urls(text);
  let escaped = escape_html(&without_links);
  let trimmed = escaped.trim();
  cap_string(trimmed, max_len)
}
