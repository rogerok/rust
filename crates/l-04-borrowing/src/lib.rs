pub fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    let a_len = a.len();
    let b_len = b.len();
    if a_len >= b_len { a } else { b }
}

pub fn common_prefix<'a>(a: &'a str, b: &str) -> &'a str {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut end = 0;
    while end < a_bytes.len() && end < b_bytes.len() && a_bytes[end] == b_bytes[end] {
        end += 1;
    }
    &a[..end]
}

pub fn first_word(s: &str) -> &str {
    for (i, byte) in s.bytes().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

pub fn last_word(s: &str) -> &str {
    let len = s.len();
    let bytes = s.as_bytes();

    for i in 0..len {
        let pos = len - i;

        if bytes[pos - 1] == b' ' {
            return &s[pos..];
        }
    }

    s
}

pub fn halves(values: &[i32]) -> (&[i32], &[i32]) {
    let middle = values.len() / 2;

    (&values[..middle], &values[middle..])
}
