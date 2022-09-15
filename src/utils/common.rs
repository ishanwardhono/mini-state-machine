pub fn StringToBool(bool_str: String) -> Option<bool> {
    match bool_str.to_lowercase().as_str() {
        "true" | "t" | "1" => Some(true),
        "false" | "f" | "0" => Some(false),
        _ => None,
    }
}
