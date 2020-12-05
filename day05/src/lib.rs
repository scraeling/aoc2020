mod input;

type Seat = (u32, u32, u32); // (row, col, id)

trait StringToNumber {
    fn binary_to_u32(&self) -> u32;
}

impl StringToNumber for str {
    fn binary_to_u32(&self) -> u32 {
        u32::from_str_radix(self, 2).unwrap()
    }
}

pub fn parse_input(input: &str) -> Vec<Seat> {
    input
    .split('\n')
    .map(|seat| {
        let (row, col) = seat.split_at(7);
        let row = row.chars()
            .map(|c| match c {
                'F' => '0',
                _ => '1'
            })
            .collect::<String>()
            .binary_to_u32();
        let col = col.chars()
            .map(|c| match c {
                'R' => '1',
                _ => '0'
            })
            .collect::<String>()
            .binary_to_u32();
        (row, col, row * 8 + col)
    })
    .collect::<Vec<Seat>>()
}

#[cfg(test)]
mod tests {
    use super::{input::INPUT, parse_input};
    
    #[test]
    fn test_parsing() {
        assert_eq!(parse_input("BFFFBBFRRR")[0], (70, 7, 567));
    }

    #[test]
    fn test_find_highest_seat_id() { // Part 1
        let highest = parse_input(INPUT)
            .iter()
            .map(|seat| seat.2)
            .max()
            .unwrap();
        println!("Highest Seat ID: {}", highest);
    }

    #[test]
    fn test_find_my_seat() { // Part 2
        let all_ids = parse_input(INPUT)
            .iter()
            .map(|seat| seat.2)
            .collect::<Vec<_>>();
        let mut my_seat = 0;
        for id in 13u32..806 {
            if !all_ids.contains(&id)
            && all_ids.contains(&(id-1))
            && all_ids.contains(&(id+1)) {my_seat = id; break}
        }
        println!("My Seat ID: {}", my_seat);
    }
}