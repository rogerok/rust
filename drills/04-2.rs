fn main() {
    let mut text = String::from("hello world");
    let first = &text[0..5];
    text.clear();
    println!("первое слово: {first}");
}
