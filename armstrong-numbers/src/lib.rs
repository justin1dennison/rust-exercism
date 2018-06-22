pub fn is_armstrong_number(num: u32) -> bool {
    let mut work_num = num.clone();
    let mut digits = vec![];
    while work_num > 0 {
        let digit = work_num % 10;
        digits.push(digit);
        work_num /= 10;
    }
    let mut result = 0u32;
    let length = digits.len() as u32;
    for digit in digits.iter() {
        result += digit.pow(length);
    }
    result == num
}
