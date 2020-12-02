mod input;

pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .split("\n")
        .into_iter()
        .map(|x| x.parse().unwrap_or(0))
        .collect::<Vec<u32>>()
}

pub fn find(list: Vec<u32>) -> (u32, u32) {
    for i in &list {
        for j in &list {
            if i + j == 2020 {
                return (i.clone(), j.clone());
            }
        }
    }
    return (0, 0);
}

pub fn find_triple(list: Vec<u32>) -> (u32, u32, u32) {
    for i in &list {
        for j in &list {
            for k in &list {
                if i + j + k == 2020 {
                    return (i.clone(), j.clone(), k.clone());
                }
            }
        }
    }
    return (0, 0, 0);
}

#[cfg(test)]
mod test {
    use super::{input::INPUT, find, find_triple, parse_input};

    #[test]
    fn find_two_numbers_that_sum_2020() { // part 1
        let (x, y) = find(parse_input(INPUT));
        println!("{}*{}={}", x, y, x * y);
    }

    #[test]
    fn find_three_numbers_that_sum_2020() { // part 2
        let (x, y, z) = find_triple(parse_input(INPUT));
        println!("{}*{}*{}={}", x, y, z, x*y*z);
    }
}