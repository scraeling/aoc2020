use std::fs::read_to_string;
use timer::time;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Mask {
    mask: String,
    mask_0: usize,
    mask_1: usize
}

impl Mask {
    fn apply(&self, number: &usize) -> usize {
        (number & self.mask_0) | self.mask_1
    }

    fn apply_v2(&self, number: &usize) -> String {
        format!("{:036b}", number | self.mask_0)
        .chars()
        .zip(self.mask.chars())
        .map(|(n, m)| match m {
            'X' => 'X',
            _ => n
        })
        .collect::<String>()
    }

    fn combinations<'a>(mask: String, combs: &'a mut HashSet<String>) -> &'a mut HashSet<String> {
        if mask.contains("X") {
            Mask::combinations(mask.replacen("X", "0", 1), combs);
            Mask::combinations(mask.replacen("X", "1", 1), combs);
        }
        else {
            combs.insert(mask);
        }
        combs
    }
}

impl From<&str> for Mask {
    fn from(mask: &str) -> Self {
        let mask = mask.to_string();
        let mask_0 = usize::from_str_radix(&mask.replace("X", "1"), 2).unwrap();
        let mask_1 = usize::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
        Mask {mask, mask_0, mask_1}
    }
}

enum Instruction {
    Mask(Mask),
    Mem(usize, usize)
}

fn parse_input(input: String) -> Vec<Instruction> {
    input.trim()
    .lines()
    .map(|l| {
        if l.starts_with("mask") {
            Instruction::Mask(l.split(" ").nth(2).unwrap().into())
        }
        else {
            let mut parts = l.split(" = ");
            Instruction::Mem (
                parts.next().unwrap().split_at(4).1
                    .strip_suffix("]").unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap()
            )
        }
    })
    .collect()
}

fn part_1(instructions: &Vec<Instruction>) -> usize {
    let mut memory = HashMap::<usize, usize>::new();
    let mut mask: Mask = "X".into();
    for i in instructions {
        match i {
            Instruction::Mask(m) => mask = m.clone(),
            Instruction::Mem(address, value) => 
            { memory.insert(address.clone(), mask.apply(value)); }
        }
    }
    memory.values().sum()
}

fn part_2(instructions: &Vec<Instruction>) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut mask: Mask = "X".into();

    for i in instructions {
        match i {
            Instruction::Mask(m) => mask = m.clone(),
            Instruction::Mem(address, value) => {
                Mask::combinations(mask.apply_v2(address), &mut HashSet::new())
                .iter()
                .map(|m| usize::from_str_radix(m, 2).unwrap())
                .for_each(|a| {memory.insert(a, value.clone());})
            }
        }
    }
    memory.values().sum()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let instructions = time!(parse_input(input));
    let p1_answer = time!(part_1(&instructions));
    let p2_answer = time!(part_2(&instructions));
    
    println!("Decoder Emulator v1 memory total: {}", p1_answer);
    println!("Decoder Emulator v2 memory total: {}", p2_answer);
}