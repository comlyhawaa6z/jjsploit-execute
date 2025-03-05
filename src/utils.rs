pub fn validate_script(script: &str) -> bool {
    !script.is_empty() && script.len() < 500
}

pub fn sanitize_script(script: &str) -> String {
    script.replace(";", "").replace("'", "").replace("\"", "")
}