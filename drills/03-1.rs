fn shout(text: String) -> usize {
    text.len()
}

fn main() {
    let n = 7;
    let m = n;
    let s = String::from("hi");
    let len = shout(s);
    println!("{n} {m} {len}");
    println!("{s}");
}
