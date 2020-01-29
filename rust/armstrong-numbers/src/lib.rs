pub fn is_armstrong_number(num: u32) -> bool {
    let string_literal = num.to_string();
    let pow = string_literal.len();
    let mut sum = 0;

    for char_number in string_literal.chars() {
        sum += char_number.to_digit(10).unwrap().pow(pow as u32);
    }
    return  sum == num;
}
