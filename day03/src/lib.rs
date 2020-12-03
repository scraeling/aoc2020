mod input;

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.split("\n")
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>()
}

pub fn detect_collisions(map: &Vec<Vec<char>>, vertical_vector: usize, horizontal_vector: usize) -> u32 {
    let mut pos: (usize, usize) = (0, 0); // vertical, horizontal
    let mut collisions: u32 = 0;
    let vertical_bound = map.len();
    let horizontal_bound = map[0].len();

    while pos.0 < vertical_bound {
        match map[pos.0][pos.1] {
            '#' => collisions += 1,
            _ => ()
        }
        pos.0 += vertical_vector;
        pos.1 = (pos.1 + horizontal_vector) % horizontal_bound;
    }
    collisions
}

#[cfg(test)]
mod tests {
    use super::{input::INPUT, parse_input, detect_collisions};
    
    #[test]
    fn first_test() { // part 1
        let col = detect_collisions(&parse_input(INPUT), 1, 3);
        println!("{}", col);
    }

    #[test]
    fn second_test() { // part 2
        let i = parse_input(INPUT);
        let vectors = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let answer: u64 = vectors.iter()
            .map(|v| detect_collisions(&i, v.1, v.0) as u64)
            .product();
        println!("{}", answer);
    }
}