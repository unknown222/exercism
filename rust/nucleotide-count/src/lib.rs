use std::collections::HashMap;
use std::fmt::Error;
use std::iter::FromIterator;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

fn is_valid(char: &char) -> bool {
    NUCLEOTIDES.contains(char)
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut amount: usize = 0;

    if !is_valid(&nucleotide) {
        return Err(nucleotide);
    }

    for c in dna.chars() {
        if !is_valid(&c) {
            return Err(c);
        }

        if c == nucleotide {
            amount += 1
        }
    }
    Ok(amount)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result: HashMap<char, usize> = HashMap::new();

    for nucleotide in NUCLEOTIDES.iter() {
        match count(*nucleotide, dna) {
            Ok(v) => result.insert(*nucleotide, v),
            Err(err) => return Err(err),
        };
    }
    Ok(result)
}
