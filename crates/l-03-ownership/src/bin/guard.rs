struct ConnectionGuard {
    name: String,
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        println!("connection {} closed ", self.name);
    }
}

fn main() {
    let _f = ConnectionGuard {
        name: String::from("first"),
    };
    let _s = ConnectionGuard {
        name: String::from("second"),
    };
}
