use l_03_ownership::{describe, describe_and_return};

#[test]
fn describe_returns_length_of_ascii_string() {
    assert_eq!(describe(String::from("hello")), 5);
}

#[test]
fn describe_returns_zero_for_empty_string() {
    assert_eq!(describe(String::new()), 0);
}

#[test]
fn describe_counts_cyrillic_string_like_string_len_does() {
    assert_eq!(
        describe(String::from("привет")),
        12,
        "6 букв, а ожидается 12 — разберись, что именно считает String::len"
    );
}

#[test]
fn describe_and_return_gives_length_and_the_same_string_back() {
    let original = String::from("итоги");
    let (len, back) = describe_and_return(original);

    assert_eq!(
        len,
        back.len(),
        "длина должна совпадать с длиной возвращённой строки"
    );
    assert_eq!(
        back, "итоги",
        "строка должна вернуться вызывающему без изменений"
    );
}
