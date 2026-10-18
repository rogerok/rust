#![allow(unused_variables)]

struct Noisy(&'static str); // тип-обёртка над строкой; синтаксис структур будет позже

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0); // выполняется, когда значение уничтожается
    }
}

fn main() {
    let a = Noisy("a");
    let b = Noisy("b");
    let c = a;
    println!("end of main");
}
