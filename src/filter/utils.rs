pub(crate) fn key_join(s: &str, j: &str) -> String {
    if s.is_empty() {
        return String::from(j);
    }

    if j.is_empty() {
        return String::from(s);
    }

    format!("{}.{}", s, j)
}
