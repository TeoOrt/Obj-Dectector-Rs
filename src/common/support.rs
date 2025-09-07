pub fn decode_dev_number(path: &str) -> Option<i32> {
    let pos = match path.rfind(|c: char| !c.is_ascii_digit()) {
        Some(t) => t,
        None => {
            return None;
        }
    };
    path[pos + 1..].parse().ok()
}
