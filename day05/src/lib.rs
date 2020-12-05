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
    let mut seats = input
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
        .collect::<Vec<Seat>>();

    seats.sort_by(|a, b| a.cmp(b));
    seats
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
        println!("Highest Seat ID: {}", parse_input(INPUT).last().unwrap().2);
    }

    #[test]
    fn test_find_my_seat() { // Part 2
        let seats = parse_input(INPUT);
        let mut my_seat = 0;
        for adj_seats in seats.windows(2) {
            if adj_seats[0].2 + 1 != adj_seats[1].2 {
                my_seat = adj_seats[0].2 + 1;
                break;
            }
        }
        println!("My Seat ID: {}", my_seat);
    }
}