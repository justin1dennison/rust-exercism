pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
        (1..limit)
         .filter(|n| factors
                        .iter()
                        .map(|factor| n % factor == 0)
                        .fold(false, |acc, x| acc || x))
         .sum()
}


