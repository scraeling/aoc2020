mod input;

use regex::Regex;

pub struct Rule {
    bag_name: String,
    contains: Vec<(String, u32)>, // bag_name, quantity
}

pub fn parse_input(input: &str) -> Vec<Rule> {
    let rule_pattern = Regex::new(r#"((:?\d\s)?\S+\s\S+) bag"#).unwrap();
    input
        .split("\n")
        .map(|rule_line| {
            let mut rule_parts = rule_pattern
            .captures_iter(rule_line)
            .filter_map(|m| {
                let part = m.get(1).unwrap().as_str();
                match part {
                    "no other" => None,
                    _ => Some(part)
                }
            });
            Rule {
                bag_name: rule_parts.next().unwrap().to_string(),
                contains: rule_parts.map(|part|{
                    let mut split = part.split(' ');
                    let num = split.next().unwrap().parse::<u32>().unwrap();
                    (split.collect::<Vec<&str>>().join(" "), num)
                })
                .collect::<Vec<_>>()
            }
        })
        .collect::<Vec<Rule>>()
}

pub fn direct_containers_of(bag_name: String, rules: &Vec<Rule>) -> Vec<String> {
    rules
    .into_iter()
    .filter_map(|rule| {
        match rule.contains
            .iter()
            .any(|item| item.0 == bag_name) {
                true => Some(rule.bag_name.clone()),
                false => None
            }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::{input::INPUT, parse_input, direct_containers_of};
    use std::collections::HashMap;

    #[test]
    fn test_how_many_bags_can_contain_shiny_gold() { // Part 1
        let rules = parse_input(INPUT);
        let mut queue = direct_containers_of("shiny gold".to_string(), &rules);
        let mut answer = queue.clone();
        
        while !queue.is_empty() {
            let result = direct_containers_of(queue.pop().unwrap()
            , &rules);
            queue.extend_from_slice(&result);
            answer.extend_from_slice(&result);
        }
        answer.sort();
        answer.dedup();
        println!("Number of bags that can eventually contain `shiny gold`: {}", answer.len());
    }

    #[test]
    fn test_total_bags_inside_shiny_gold() { // Part 2
        let rules = parse_input(INPUT)
            .into_iter()
            .map(|rule| {
                (
                    rule.bag_name,
                    rule.contains
                )
            })
            .collect::<HashMap<_, _>>();
        let mut count = 0u32;
        let mut queue = vec![(1u32, "shiny gold".to_string())];
        
        while !queue.is_empty() {
            let (multiplier, bag_name) = queue.pop().unwrap();
            for bag in &rules[&bag_name] {
                count += bag.1 * multiplier;
                queue.push((bag.1 * multiplier, bag.0.clone()));
            }
        }
        println!("Total number of bags inside `shiny gold`: {}", count);
    }
}