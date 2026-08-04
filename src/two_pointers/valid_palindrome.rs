pub fn is_palindrome(s: String) -> bool {
    if s.is_empty() {
        return false;
    }

    let s: Vec<char> = s.chars().collect();
    let mut l = 0;
    let mut r = s.len() - 1;

    loop {
        while l < r && !s[l].is_ascii_alphanumeric() {
            l += 1
        }

        while l < r && !s[r].is_ascii_alphanumeric() {
            r -= 1
        }

        if l >= r {
            break;
        }

        if l < r && !s[l].eq_ignore_ascii_case(&s[r]) {
            return false;
        }

        l += 1;
        r -= 1;
    }

    true
}
