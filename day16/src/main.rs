use std::fs::read_to_string;
use timer::time;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct FieldRule {
    name: String,
    ranges: Vec<std::ops::RangeInclusive<usize>>,
}

fn parse_input(input: String) -> (Vec<FieldRule>, Vec<usize>, Vec<Vec<usize>>) {
    let mut parts = input.trim().split("\n\n");
    let field_rules = parts.next().unwrap()
        .lines()
        .map(|l| {
            let mut parts = l.split(" ");
            let name = parts.next().unwrap().strip_suffix(":").unwrap().to_string();
            let range1 = parts.next().unwrap().split("-")
                .map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
            let range2 = parts.last().unwrap().split("-")
                .map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
            FieldRule {
                name,
                ranges: vec![range1[0]..=range1[1], range2[0]..=range2[1]]
            }
        })
        .collect::<Vec<FieldRule>>();
    let my_ticket = parts.next().unwrap()
        .lines()
        .last()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let nearby_tickets = parts.next().unwrap()
        .lines()
        .skip(1)
        .map(|l|
            l.split(",").map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
        ).collect::<Vec<Vec<usize>>>();

    (field_rules, my_ticket, nearby_tickets)
}

fn part_1
(
    field_rules: &Vec<FieldRule>,
    nearby_tickets: &Vec<Vec<usize>>
) -> usize 
{
    nearby_tickets.iter()
    .flat_map(|ticket| 
        ticket.iter()
        .filter_map(|n| {
            if !field_rules.iter().any(|f|
                f.ranges.iter().any(|r| r.contains(n))
            ) {Some(n)} else {None}
        })
    )
    .sum()
}

fn part_2
(
    field_rules: &Vec<FieldRule>, 
    my_ticket: &Vec<usize>,
    nearby_tickets: &Vec<Vec<usize>>
) -> usize 
{
    let mut pos_filter = HashMap::<&str, Vec<bool>>::new();
    
    let valid_tickets = nearby_tickets.iter()
        .filter(|&ticket|
            ticket.iter()
            .all(|n|
                field_rules.iter()
                .any(|f|
                    f.ranges.iter()
                    .any(|r| r.contains(n))
                )
            )
        )
        .collect::<Vec<&Vec<usize>>>();

    let num_fields = field_rules.len();

    for f in field_rules {
        for &t in &valid_tickets {
            for (pos, n) in t.iter().enumerate() {
                pos_filter.entry(&f.name)
                .or_insert(vec![true; num_fields])
                [pos] &= f.ranges.iter().any(|r| r.contains(n))
            }
        }
    }
    
    let mut possible_positions: HashSet<Vec<usize>> = HashSet::new();
    gen_positions(&mut possible_positions, &pos_filter, field_rules, vec![]);
    
    let p = possible_positions.iter().next().unwrap();
    (0..=5usize).into_iter()
    .map(|n| 
        my_ticket[p.iter().position(|&x| x == n).unwrap()]
    )
    .product()
}

fn gen_positions // honestly not sure how this even works at this point
(
    set: &mut HashSet<Vec<usize>>,
    filter: &HashMap<&str, Vec<bool>>,
    rules: &Vec<FieldRule>,
    positions: Vec<usize>
)
{
    let index = positions.len();
    if index == filter.len() {
        set.insert(positions);
        return
    }
    for (field_id, field) in rules.iter().enumerate() {
        if !positions.contains(&field_id) && filter[field.name.as_str()][index] {
            let mut new_pos = positions.clone();
            new_pos.push(field_id);
            gen_positions(set, filter, rules, new_pos);
        }
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (field_rules, my_ticket, nearby_tickets) = time!(parse_input(input));
    let p1_answer = time!(part_1(&field_rules, &nearby_tickets));
    let p2_answer = time!(part_2(&field_rules, &my_ticket, &nearby_tickets));

    println!("Ticket scanning error rate: {}", p1_answer);
    println!("Product of the first six ticket fields: {}", p2_answer);
}