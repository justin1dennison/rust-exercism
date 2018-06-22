pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 =>  Some(0),
        2 =>  Some(1),
        4 =>  Some(2),
        _ =>  match n % 2 {
            0 => collatz(n / 2).map(|x| x + 1),
            _ => collatz(3 * n + 1).map(|x| x + 1)
        }
    } 
}
