pub fn square_of_sum(n: usize) -> usize {
    square((1..(n + 1)).sum())
}

pub fn sum_of_squares(n: usize) -> usize {
    (1..(n + 1)).map(square).sum()
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}

fn square(n: usize) -> usize {
    n * n
}
