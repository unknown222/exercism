pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut current_factor = 2;
    let mut current_value = n;

    while current_factor <= current_value {
        if current_value % current_factor == 0 {
            factors.push(current_factor);
            current_value = current_value / current_factor;
        } else {
            current_factor = current_factor + 1;
        }
    }
    return factors;
}
