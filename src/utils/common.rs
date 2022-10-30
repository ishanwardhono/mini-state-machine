pub fn string_to_bool(bool_str: String) -> Option<bool> {
    match bool_str.to_lowercase().as_str() {
        "true" | "t" | "1" => Some(true),
        "false" | "f" | "0" => Some(false),
        _ => None,
    }
}

pub trait VecExt {
    fn uncontain(&self, data: Vec<String>) -> Vec<String>;
}

impl VecExt for Vec<String> {
    fn uncontain(&self, data: Vec<String>) -> Vec<String> {
        let mut res = vec![];
        for item in self {
            if !data.contains(&item) {
                res.push(item.clone());
            }
        }
        res
    }
}
