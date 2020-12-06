mod input;

type Seat = (u32, u32, u32); // (row, col, id)

pub fn parse_input(input: &str) -> Vec<Seat> {
    let mut seats = input
        .split('\n')
        .map(|seat| {
            let binary_coded_seat = u32::from_str_radix(
                &seat.chars()
                .map(|c| match c {
                    'B' => '1',
                    'F' => '0',
                    'R' => '1',
                    'L' => '0',
                    _ => panic!("Invalid input: {}", c)
                })
                .collect::<String>(),
                2
            ).unwrap();
            let row = binary_coded_seat >> 3;
            let col = binary_coded_seat & 0b0000000111;
            (row, col, row * 8 + col)
        })
        .collect::<Vec<Seat>>();

    seats.sort_by(|a, b| a.2.cmp(&b.2));
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