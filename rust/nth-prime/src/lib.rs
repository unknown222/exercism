pub fn nth(n: u32) -> u32 {
    let mut prime = 0;

    for index in 0..n + 1 {
        prime = next_prime(prime);
    }

    return prime;
}

fn next_prime(n: u32) -> u32 {
    let mut prime: u32 = n;
    let mut found = false;

    while !found {
        prime = prime + 1;
        if is_prime(prime) {
            found = true;
        }
    }

    return prime;
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i = i + 6;
    }

    return true;
}