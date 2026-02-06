#[allow(dead_code)]
fn skip_a(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();
    let rest = chars.as_str();

    if first == 'a' {
        skip_a(rest)
    } else {
        let mut result = String::from(first);
        result.push_str(&skip_a(rest));
        result
    }
}
