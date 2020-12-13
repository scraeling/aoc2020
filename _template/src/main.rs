use std::fs::read_to_string;
use timer::time;

fn parse_input(input: String) {

}

fn part_1() -> usize {
    0
}

// fn part_2() {

// }

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let parsed_input = time!(parse_input(input));
    let p1_answer = time!(part_1());
    // let p2_answer = time!(part_2());

    println!("{}", p1_answer);
    //println!("{}", p2_answer);
}