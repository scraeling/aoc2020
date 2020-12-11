use std::fs::read_to_string;
use timer::time;

fn main() {
    let input = time!(parse_input);
    let input_ref = input.as_ref();
    let p1_answer = time!(part_1, input_ref);
    let p2_answer = time!(part_2, input_ref);

    println!("{}", p1_answer);
    println!("{}", p2_answer);
}

fn parse_input() -> String {
    read_to_string("input.txt").unwrap()
}

fn part_1(input: &str) -> String {
    "".to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        unimplemented!();
    }
}
