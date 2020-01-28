pub fn square_of_sum(n: u32) -> u32 {
    let sum: u32 = (1..n + 1).sum();
    return sum.pow(2);
}

pub fn sum_of_squares(n: u32) -> u32 {
    let squares = (1..n + 1).map(|x| x.pow(2));
    return squares.sum();
}

pub fn difference(n: u32) -> u32 {
    let square_of_sum = square_of_sum(n);
    let sum_of_squares = sum_of_squares(n);
    println!("{}, {}", square_of_sum, sum_of_squares);
    return square_of_sum - sum_of_squares;
}
