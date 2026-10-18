pub fn describe(s: String) -> usize {
    s.len()
}

pub fn describe_and_return(s: String) -> (usize, String) {
    (s.len(), s)
}
