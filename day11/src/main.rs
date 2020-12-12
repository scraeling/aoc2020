use std::fs::read_to_string;
use timer::time;

#[derive(Debug, Copy, Clone)]
enum Tile {
    Floor,
    Seat,
    Occupied
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'L' => Tile::Seat,
            '#' => Tile::Occupied,
            '.' => Tile::Floor,
            _ => panic!("Bad Input")
        }
    }
}

#[derive(Clone, Debug)]
struct Map {
    rows: usize,
    cols: usize,
    tiles: Vec<Tile>
}

impl Map {
    fn index(&self, col: usize, row: usize) -> usize {
        col + row * self.cols
    }

    fn reverse_index(&self, i: usize) -> (usize, usize) {
        let col = i % self.cols;
        let row = i / self.cols;
        (col, row)
    }

    fn adjacent_occupancy(&self, coord: (usize, usize)) -> u32 {
        use Tile::*;
        let (x, y) = coord;
        let mut occ = 0u32;
        
        let range_limiter =
            |c: usize, limit: usize| match c {
                0 => 0..=1,
                n if n == limit-1 => n-1..=n,
                n => n-1..=n+1
            };
        let col_range = range_limiter(x, self.cols);
        let row_range = range_limiter(y, self.rows);
        
        for col in col_range.clone() {
            for row in row_range.clone() {
                if col == x && row == y { continue }
                match self.tiles[self.index(col, row)] {
                    Occupied => occ += 1,
                    Seat | Floor => continue
                }
            }
        }
        occ
    }

    fn visible_seat_occupancy(&self, coord: (usize, usize)) -> u32 {
        use Tile::*;
        let (x, y) = coord;
        let mut occ = 0u32;
        for direction in &[(1i32,0i32), (-1,0), (0,1), (0,-1), (1,1), (-1,-1), (-1,1), (1,-1)] {
            let mut col = x as i32;
            let mut row = y as i32;
            loop {
                col += direction.0;
                row += direction.1;

                if !(row >= 0 && row < (self.rows as i32)
                && col >= 0 && col < (self.cols as i32))
                {break}

                match self.tiles[self.index(col as usize, row as usize)] {
                    Occupied => {
                        occ += 1;
                        break
                    }
                    Seat => break,
                    Floor => continue
                }
            }
        }
        occ
    }
}

impl std::fmt::Display for Map {
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Tile::*;
        let i = self.tiles.iter()
        .map(|tile| match tile {
            Floor => '.',
            Seat => 'L',
            Occupied => '#'
        })
        .collect::<String>();
        for x in 0..self.rows {
            let start = self.index(0, x);
            let _ = write!(f, "{}\n", &i[start..start+self.cols]);
        }
        Ok(())
    }
}

fn parse_input(input: String) -> Map {
    let lines = input.split("\n");
    let rows = lines.clone().count();
    let cols = lines.clone().next().unwrap().chars().count();
    let tiles = lines
        .map(|line| 
            line.chars()
            .map(|c| c.into())
            .collect::<Vec<Tile>>()
        )
        .flatten()
        .collect::<Vec<Tile>>();

    Map {rows, cols, tiles}
}

fn simulate<F>(map: &Map, sim_method: F, threshold: usize) -> (Map, u32) 
    where F: Fn((usize, usize)) -> usize
{
    use Tile::*;
    let mut changes = 0;

    let new_tiles = map.tiles.iter()
    .enumerate()
    .map(|(i, tile)|
        match tile {
            Seat 
            if sim_method(map.reverse_index(i)) == 0 => {
                changes += 1;
                Occupied
            }
            Occupied 
            if sim_method(map.reverse_index(i)) >= threshold => {
                changes += 1;
                Seat
            },
            _ => tile.clone()
    }).collect::<Vec<Tile>>();

    (Map{tiles: new_tiles, ..*map}, changes)
}

fn part_1(initial_map: &Map) -> usize {
    let mut map: Map = initial_map.clone();
    
    loop {
        let sim_method = |x| map.adjacent_occupancy(x) as usize;
        let (m, c) = simulate(&map, sim_method, 4);
        map = m;
        if c == 0 {break}
    }

    map.tiles.iter().filter(|tile| match tile {
        Tile::Occupied => true,
        _ => false
    })
    .count()
}

fn part_2(initial_map: &Map) -> usize {
    let mut map: Map = initial_map.clone();
    
    loop {
        let sim_method = |x| map.visible_seat_occupancy(x) as usize;
        let (m, c) = simulate(&map, sim_method, 5);
        map = m;
        if c == 0 {break}
    }
    map.tiles.iter().filter(|tile| match tile {
        Tile::Occupied => true,
        _ => false
    })
    .count()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let map = time!(parse_input(input));
    let p1_answer = time!(part_1(&map));
    let p2_answer = time!(part_2(&map));

    println!("Number of occupied seats when people consider adjacent seats only: {}", p1_answer);
    println!("Number of occupied seats when people consider visible seats only: {}", p2_answer);
}