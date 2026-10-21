use l_04_borrowing::{first_word, last_word};

#[test]
fn first_word_accepts_string_reference() {
    let line = String::from("hello world");
    assert_eq!(
        first_word(&line),
        "hello",
        "из &String \"hello world\" ожидалось первое слово"
    );
}

#[test]
fn first_word_accepts_string_literal() {
    assert_eq!(
        first_word("borrow checker"),
        "borrow",
        "из литерала ожидалось первое слово"
    );
}

#[test]
fn first_word_accepts_slice_of_another_string() {
    let line = String::from("one two three");
    assert_eq!(
        first_word(&line[4..]),
        "two",
        "срез \"two three\" начинается со слова \"two\""
    );
}

#[test]
fn first_word_handles_cyrillic() {
    assert_eq!(
        first_word("привет мир"),
        "привет",
        "кириллица: ожидалось первое слово целиком, без паники"
    );
}

#[test]
fn first_word_without_space_returns_whole_line() {
    assert_eq!(
        first_word("rust"),
        "rust",
        "строка без пробела — это одно слово"
    );
}

#[test]
fn first_word_of_empty_line_is_empty() {
    assert_eq!(first_word(""), "", "у пустой строки первое слово пустое");
}

#[test]
fn first_word_returns_window_into_input() {
    let line = String::from("hello world");
    let word = first_word(&line);
    assert_eq!(
        word.as_ptr(),
        line.as_ptr(),
        "результат должен смотреть в исходную строку, а не быть новой строкой"
    );
}

#[test]
fn last_word_accepts_string_reference() {
    let line = String::from("hello world");
    assert_eq!(
        last_word(&line),
        "world",
        "из &String \"hello world\" ожидалось последнее слово"
    );
}

#[test]
fn last_word_accepts_string_literal() {
    assert_eq!(
        last_word("one two three"),
        "three",
        "из литерала с тремя словами ожидалось последнее"
    );
}

#[test]
fn last_word_accepts_slice_of_another_string() {
    let line = String::from("one two three");
    assert_eq!(
        last_word(&line[..7]),
        "two",
        "срез \"one two\" заканчивается словом \"two\""
    );
}

#[test]
fn last_word_handles_cyrillic() {
    assert_eq!(
        last_word("привет мир"),
        "мир",
        "кириллица: ожидалось последнее слово целиком, без паники"
    );
}

#[test]
fn last_word_without_space_returns_whole_line() {
    assert_eq!(
        last_word("rust"),
        "rust",
        "строка без пробела — это одно слово"
    );
}

#[test]
fn last_word_of_empty_line_is_empty() {
    assert_eq!(last_word(""), "", "у пустой строки последнее слово пустое");
}

#[test]
fn last_word_returns_window_into_input() {
    let line = String::from("hello world");
    let word = last_word(&line);
    assert_eq!(
        word.as_ptr(),
        line[6..].as_ptr(),
        "результат должен смотреть в исходную строку, а не быть новой строкой"
    );
}
