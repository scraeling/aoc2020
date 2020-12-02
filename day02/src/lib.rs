mod input;
use regex::Regex;

pub fn parse_input(input: &str) -> Vec<(u32, u32, char, String)> {
    let pattern = Regex::new(r#"(\d+)-(\d+)\s(\S):\s(\S+)"#).unwrap();
    
    input.split("\n")
    .map(|line| {
        let caps = pattern.captures(line).unwrap();
        (
            caps.get(1).unwrap().as_str().parse::<u32>().unwrap(), //low
            caps.get(2).unwrap().as_str().parse::<u32>().unwrap(), //high
            caps.get(3).unwrap().as_str().chars().next().unwrap(), //character
            caps.get(4).unwrap().as_str().to_string()              //password
        )
    }).collect::<Vec<_>>()
}

pub fn is_password_valid(pass: &(u32, u32, char, String)) -> bool {
    let count = pass.3.matches(pass.2).count() as u32;
    if count >= pass.0 && count <= pass.1 {return true}
    false
}

pub fn is_toboggan_corporate_policy_compliant(pass: &(u32, u32, char, String)) -> bool {
    let chars = pass.3.chars().collect::<Vec<_>>();
    if (chars[pass.0 as usize - 1] == pass.2) 
    ^ (chars[pass.1 as usize - 1] == pass.2) {return true}
    false
}

#[cfg(test)]
mod tests {
    use super::{input::INPUT, parse_input, is_password_valid, is_toboggan_corporate_policy_compliant};

    #[test]
    fn test_for_valid_passwords() { // part1
        let list = parse_input(INPUT);
        let num_valid = list.into_iter()
            .filter(|p| is_password_valid(p))
            .count();
        println!("{}", num_valid);
    }

    #[test]
    fn test_toboggan_corporate_policy_compliance() { // part2
        let list = parse_input(INPUT);
        let valid = list.into_iter()
            .filter(|p| is_toboggan_corporate_policy_compliant(p))
            .collect::<Vec<_>>();
        println!("{}", valid.len());
        //println!("{:?}", valid);
    }
}