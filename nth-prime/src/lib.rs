pub fn nth(n: u32) -> Option<u32> {
    if n == 0 {
        None
   } else {
       let mut primes = vec![2];
       let mut count  = 3;
       while primes.len() != (n as usize) {
        if is_prime(count) {
            primes.push(count); 
        } 
        count += 2; 
       }
       let index = (primes.len() - 1) as usize;
       primes
            .get(index)
            .map(|i| *i)
   }
}

fn is_prime(u: u32) -> bool {
    let root = (u as f32).sqrt() as u32 + 1;
    for n in 2..root{
        if u % n == 0 {
            return false 
        } 
    }
    return true
}
