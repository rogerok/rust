struct TempFile {
    path: String,
}

impl Drop for TempFile {
    fn drop(&mut self) {
        println!("Dropping TempFile: {}", self.path);
    }
}

// second.md, third.md, first.md
fn main() {
    let _first = TempFile {
        path: String::from("first.md"),
    };

    {
        let _second = TempFile {
            path: String::from("second.md"),
        };
    }

    let _third = TempFile {
        path: String::from("third.md"),
    };
}
