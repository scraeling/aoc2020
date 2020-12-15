use std::fs::read_to_string;
use timer::time;
use std::collections::HashMap;

fn parse_input(input: String) -> Vec<usize> {
    input
    .split(",")
    .map(|n| n.parse::<usize>().unwrap())
    .collect()
}

struct MemoryGame {
    turn: usize,
    considering: usize,
    last: usize,
    cache: HashMap<usize, usize>
}

impl MemoryGame {
    fn new(starting_numbers: &Vec<usize>) -> Self {
        let mut game = Self {
            turn: 0,
            considering: 0,
            last: 0,
            cache: HashMap::new()
        };
        let l = starting_numbers.len();
        for &n in &starting_numbers[..l-1] {
            game.turn += 1;
            game.cache.insert(n, game.turn);
        }
        
        game.last = starting_numbers[l-2];
        game.considering = starting_numbers[l-1];
        game.turn += 1;
        game
    }

    fn next(&mut self) {
        self.turn += 1;
        //print!("Turn {}: Considering {} ", &self.turn, &self.considering);
        match self.cache.get(&self.considering) {
            Some(&l) => {
                self.last = self.considering;
                self.considering = self.turn - l - 1;
            }
            None => {
                self.last = self.considering;
                self.considering = 0;
            }
        }
        //println!("Spoke {}", self.considering);
        self.cache.insert(self.last, self.turn-1);
    }
}

fn part_1(starting_numbers: &Vec<usize>) -> usize {
    let mut game = MemoryGame::new(starting_numbers);
    for _ in 0..2020-starting_numbers.len() {
        game.next();
    }

    game.considering
}

fn part_2(starting_numbers: &Vec<usize>) -> usize {
    let mut game = MemoryGame::new(starting_numbers);
    for _ in 0..30000000-starting_numbers.len() {
        game.next();
    }

    game.considering
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let starting_numbers = time!(parse_input(input));
    let p1_answer = time!(part_1(&starting_numbers));
    let p2_answer = time!(part_2(&starting_numbers));

    println!("2020th number spoken: {}", p1_answer);
    println!("30000000th number spoken: {}", p2_answer);
}