//! Lesson 03: ownership — move, Copy, drop, RAII.

/// HW-RS-03-FIX-MOVE: returns the length of the string.
pub fn describe(s: String) -> usize {
    let _ = s;
    todo!("HW-RS-03-FIX-MOVE: describe")
}

/// HW-RS-03-FIX-MOVE, second fix: returns the length and hands ownership of the string back.
pub fn describe_and_return(s: String) -> (usize, String) {
    let _ = s;
    todo!("HW-RS-03-FIX-MOVE: describe_and_return")
}
