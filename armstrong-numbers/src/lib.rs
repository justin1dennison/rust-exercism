pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let length  = num_string.len() as u32;
    num == num_string
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .map(|c| c.pow(length))
            .collect::<Vec<u32>>()
            .iter()
            .sum()
}
