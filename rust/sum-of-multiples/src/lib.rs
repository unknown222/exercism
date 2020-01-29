pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {

    let mut multiples: Vec<u32> = vec![];

    for factor in factors.iter() {
        if factor == &(0 as u32) {
            continue;
        }
        let mut index = 0;
        while index < limit {
            if index % factor.clone() == 0 {
                multiples.push(index);
            }
            index = index + 1;
        }
    }

    multiples.sort();
    multiples.dedup();

    return multiples.into_iter().sum::<u32>();
}
