pub struct ValidationAlgorithm;

impl ValidationAlgorithm {
    pub fn validate_input(input: &str, max_length: usize) -> bool {
        input.len() <= max_length && !input.is_empty()
    }

    pub fn validate_email(email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }

    pub fn sanitize_input(input: &str) -> String {
        input.chars()
            .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
            .collect()
    }
}