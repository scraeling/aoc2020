use std::fs::read_to_string;
use timer::time;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let adapters = time!(parse_input(&input));
    let p1 = time!(part_1(&adapters));
    let p2 = time!(part_2(&adapters));

    println!("Part 1: 1 jolt multiplied by 3 jolt differences when all adapters are connected: {}", p1);
    println!("Part 2: Total number of distinct ways you can arrange adapters: {}", p2);
}

fn parse_input(input: &str) -> Vec<u32> {
    let mut adapters = input
        .split("\n")
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);
    adapters
}

fn part_1(adapters: &Vec<u32>) -> u32 {
    let mut jolt_diff = vec![0u32;4];
    for pair in adapters.windows(2) {
        let diff = pair[1] - pair[0];
        jolt_diff[diff as usize] += 1;
    }
    jolt_diff[1] * jolt_diff[3]
}

fn get_num_branches(adapter: &u32, set: &HashSet<u32>, branch_count: &mut HashMap<u32, u64>) -> u64 {
    (adapter+1..=adapter+3u32)
    .map(|a| {
        if set.contains(&a) {
            if branch_count.contains_key(&a) {
                branch_count[&a]
            }
            else {
                let count = get_num_branches(&a, set, branch_count);
                branch_count.insert(a, count);
                count
            }
        }
        else { 0 }
    })
    .sum()
}

fn part_2(adapters: &Vec<u32>) -> u64 {
    let target_joltage = adapters.last().unwrap().clone();
    let set = adapters.iter().map(|a| a.clone()).collect::<HashSet<u32>>();
    let mut branch_count: HashMap<u32, u64> = HashMap::new();
    branch_count.insert(target_joltage, 1);
    for a in adapters.iter().rev() {
        if !branch_count.contains_key(a) {
            let count = get_num_branches(a, &set, &mut branch_count);
            branch_count.insert(a.clone(), count);
        }
    }
    branch_count[&0u32]
}