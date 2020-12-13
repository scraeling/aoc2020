use std::fs::read_to_string;
use timer::time;

fn parse_input(input: String) -> (usize, Vec<Option<usize>>) {
    let mut lines = input.trim().split("\n");
    let depart_time = lines.next().unwrap().parse().unwrap();
    let bus_ids: Vec<Option<usize>> = lines.next().unwrap()
        .split(",")
        .map(|n| match n {
            "x" => None,
            _ => Some(n.parse::<usize>().unwrap())
        }).collect();
    
    (depart_time, bus_ids)
}

fn part_1(depart_time: usize, bus_ids: Vec<Option<usize>>) -> usize {
    let mut bus_ids = bus_ids.iter()
        .filter_map(|x| *x)
        .collect::<Vec<usize>>();
    bus_ids.sort();
    let max_time = depart_time + bus_ids.last().unwrap().clone();
    let mut earliest_times = bus_ids.iter()
        .filter_map(|&id| {
            for time in depart_time..=max_time {
                if time % id == 0 {
                    return Some((id, time))
                }
            }
            None
        })
        .collect::<Vec<(usize, usize)>>();
    earliest_times.sort_by(|a, b| a.1.cmp(&b.1));
    (earliest_times[0].1 - depart_time) * earliest_times[0].0
}

mod crt {
    fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            (b, 0, 1)
        } else {
            let (g, x, y) = egcd(b % a, a);
            (g, y - (b / a) * x, x)
        }
    }
     
    fn mod_inv(x: i64, n: i64) -> Option<i64> {
        let (g, x, _) = egcd(x, n);
        if g == 1 {
            Some((x % n + n) % n)
        } else {
            None
        }
    }
     
    pub fn remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
        let prod = modulii.iter().product::<i64>();
     
        let mut sum = 0;
     
        for (&residue, &modulus) in residues.iter().zip(modulii) {
            let p = prod / modulus;
            sum += residue * mod_inv(p, modulus)? * p
        }
     
        Some(sum % prod)
    }
}

fn part_2(bus_ids: Vec<Option<usize>>) -> i64 {
    let bus_ids: Vec<(i64, i64)> = bus_ids.iter()
        .enumerate()
        .filter_map(|(index, id)| match id {
            &Some(i) => Some((index as i64, i as i64)),
            None => None
        }).collect();
    
    let remainders: Vec<i64> = bus_ids.iter().map(|&(index, id)| id - index).collect();
    let ids: Vec<i64> = bus_ids.iter().map(|&(_, id)| id).collect();
    
    crt::remainder(remainders.as_slice(), ids.as_slice()).unwrap()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (time, bus_ids) = time!(parse_input(input));
    let p1_answer = time!(part_1(time, bus_ids.clone()));
    let p2_answer = time!(part_2(bus_ids));

    println!("Earliest departuring bus id x Wait time: {}", p1_answer);
    println!("Earliest timestamp at which each bus departs at it's offset: {}", p2_answer);
}