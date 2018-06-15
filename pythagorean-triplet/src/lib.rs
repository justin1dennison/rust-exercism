pub fn find() -> Option<u32> {
    for a in 0..500 {
        for b in 0..500 {
            let c = 1000 - a - b;
            if is_triple(a, b, c) {
                return Some(a * b * c);
            }
        }
    }
    None
}

fn is_triple(a: u32, b: u32, c: u32) -> bool {
    a * a + b * b == c * c
}
