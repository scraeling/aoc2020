use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let mut last: u128;
    let mut now: u128;
    let time = Instant::now();

    let input = parse_input();
    now = time.elapsed().as_micros();
    println!("[{0}µ/{0}µ] Finished processing input", now);
    last = now;

    part_1();
    now = time.elapsed().as_micros();
    println!("[{}µ/{}µ] Finished Part 1", now - last, now);
    last = now;

    part_2();
    now = time.elapsed().as_micros();
    println!("[{}µ/{}µ] Finished Part 2", now - last, now);
}

fn parse_input() -> String {
    read_to_string(&env::args().collect::<Vec<String>>()[1]).unwrap()
}

fn part_1() {

}

fn part_2() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        unimplemented!();
    }
}
