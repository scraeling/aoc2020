mod input;
use itertools::Itertools;

pub fn parse_input(input: &str) -> Vec<u64> {
    input
    .split("\n")
    .map(|num| num.parse::<u64>().unwrap())
    .collect()
}

pub fn is_sum_of_any(num: u64, items: &[u64]) -> bool {
    for (a, b) in items.iter().tuple_combinations() {
        if *a + *b == num { return true }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::{input::*, parse_input, is_sum_of_any};
    
    #[test]
    fn find_invalid_number() { // Part 1: 20874512
        let preamble_len: usize = 25;
        let input = parse_input(INPUT);
        for cur in preamble_len..input.len() {
            if !is_sum_of_any(input[cur], &input[cur - preamble_len..cur]) {
                println!("i{}: No matches for: {}", cur, input[cur]);
            }
        }
    }

    #[test]
    fn find_encryption_weakness() { // Part 2: 3012420
        let encryption_weakness: u64 = 0x13E8510;
        let input = parse_input(INPUT);
        let len = input.len();
        for cur in 0..len {
            for range_end in cur+1..len {
                let sum: u64 = input[cur..=range_end].iter().sum();
                if sum == encryption_weakness {
                    println!("XMAS Encryption Weakness: {}", input[cur] + input[range_end]);
                    return
                }
                if sum > encryption_weakness { break }
            }
        }
    }
}