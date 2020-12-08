mod input;
use regex::Regex;
use std::collections::HashMap;

type RuleMap = HashMap<String, Vec<(u32, String)>>;

pub fn parse_input(input: &str) -> RuleMap {
    let rule_pattern = Regex::new(r#"((\d\s)?\S+\s\S+) bag"#).unwrap();
    input
        .split("\n")
        .map(|rule_line| {
            let mut rule_parts = rule_pattern.captures_iter(rule_line)
                .filter_map(|m| {
                    let part = m.get(1).unwrap().as_str();
                    match part {
                        "no other" => None,
                        _ => Some(part),
                    }
                });
            (
                rule_parts.next().unwrap().to_string(),
                rule_parts
                    .map(|part| {
                        let mut split = part.split(' ');
                        let num = split.next().unwrap().parse::<u32>().unwrap();
                        (num, split.collect::<Vec<&str>>().join(" "))
                    })
                    .collect::<Vec<(u32, String)>>(),
            )
        })
        .collect::<RuleMap>()
}

pub fn direct_containers_of(bag_name: String, rules: &RuleMap) -> Vec<String> {
    rules
        .iter()
        .filter_map(|rule| match rule.1.iter().any(|item| item.1 == bag_name) {
            true => Some(rule.0.clone()),
            false => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{direct_containers_of, input::INPUT, parse_input};

    #[test]
    fn test_how_many_bags_can_contain_shiny_gold() { // Part 1
        let rules = parse_input(INPUT);
        let mut queue = direct_containers_of("shiny gold".to_string(), &rules);
        let mut answer = queue.clone();

        while !queue.is_empty() {
            let result = direct_containers_of(queue.pop().unwrap(), &rules);
            queue.extend_from_slice(&result);
            answer.extend_from_slice(&result);
        }

        answer.sort();
        answer.dedup();
        println!(
            "Number of bags that can eventually contain `shiny gold`: {}",
            answer.len()
        );
    }

    #[test]
    fn test_total_bags_inside_shiny_gold() { // Part 2
        let rules = parse_input(INPUT);
        let mut count = 0u32;
        let mut queue = vec![(1u32, "shiny gold".to_string())];

        while !queue.is_empty() {
            let (multiplier, bag_name) = queue.pop().unwrap();
            for bag in &rules[&bag_name] {
                count += bag.0 * multiplier;
                queue.push((bag.0 * multiplier, bag.1.clone()));
            }
        }

        println!("Total number of bags inside `shiny gold`: {}", count);
    }
}
