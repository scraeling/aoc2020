mod input;
use regex::Regex;

pub const PassportFields: &'static [&'static str] =
    &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

pub type Passport = std::collections::HashMap<String, String>;

pub fn parse_input(input: &str) -> Vec<Passport> {
    let field_pattern = Regex::new(r#"(\S+?):(\S+)"#).unwrap();
    input
        .split("\n\n")
        .map(|p| {
            field_pattern
                .captures_iter(p)
                .map(|c| (c[1].to_string(), c[2].to_string()))
                .collect::<Passport>()
        })
        .collect::<Vec<Passport>>()
}

pub fn validate_only_fields(passport: &Passport) -> bool {
    PassportFields.iter().all(|field| match *field {
        "cid" => true,
        _ => passport.contains_key(*field),
    })
}

fn number_in_range(input: &str, min: u32, max: u32) -> bool {
    let y = input.parse::<u32>().unwrap_or(0);
    y >= min && y <= max
}

pub fn validate_passport(passport: &Passport) -> bool {
    PassportFields
    .iter()
    .all(|field| match *field {
        "cid" => true,
        "byr" => match passport.get(*field) {
            None => false,
            Some(year) => number_in_range(year, 1920, 2002),
        },
        "iyr" => match passport.get(*field) {
            None => false,
            Some(year) => number_in_range(year, 2010, 2020),
        },
        "eyr" => match passport.get(*field) {
            None => false,
            Some(year) => number_in_range(year, 2020, 2030),
        },
        "hgt" => match passport.get(*field) {
            None => false,
            Some(height) => match height.split_at(height.len() - 2) {
                (value, "in") => number_in_range(value, 59, 76),
                (value, "cm") => number_in_range(value, 150, 193),
                _ => false,
            }
        },
        "hcl" => match passport.get(*field) {
            Some(color)
            if color.len() == 7 
            && color.chars().next().unwrap_or('_') == '#'
            => match u64::from_str_radix(color.strip_prefix('#').unwrap(), 16) {
                Ok(_) => true,
                Err(_) => false,
            }
            _ => false,
        },
        "ecl" => match passport.get(*field) {
            None => false,
            Some(color) =>
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .contains(&color.as_str())
        },
        "pid" => match passport.get(*field) {
            Some(id) if id.len() == 9 => match id.parse::<u64>() {
                Ok(_) => true,
                Err(_) => false,
            },
            _ => false,
        },
        &_ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::{input::INPUT, parse_input, validate_only_fields, validate_passport};

    #[test]
    fn test_validate_only_if_field_exists() {
        // Part 1
        let passports = parse_input(INPUT);
        let valid = passports.iter()
            .filter(|p| validate_only_fields(p))
            .count();
        println!("{}", valid);
    }

    #[test]
    fn test_passport_validation() { // Part 2
        let passports = parse_input(INPUT);
        let valid = passports.iter()
            .filter(|p| validate_passport(p))
            .count();
        println!("{:?}", valid);
    }
}
