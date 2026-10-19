fn main() {
    let mut s = String::from("привет");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} и {r2}");
    let w = &mut s;
    w.push_str(", мир");
    println!("{s}");
}
