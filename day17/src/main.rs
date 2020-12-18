use std::fs::read_to_string;
use timer::time;

#[derive(Copy, Clone)]
enum Cell {
    Active, Inactive
}


impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Active,
            '.' => Cell::Inactive,
            _ => panic!("Invalid Cell")
        }
    }
}

struct Matrix3D {
    size: usize,
    space: Vec<Vec<Vec<Cell>>>
}

impl Matrix3D {
    fn new(size: usize) -> Self {
        Self {
            size,
            space: vec![vec![vec![Cell::Inactive; size]; size]; size]
        }
    }

    fn get(&self, x:usize, y:usize, z:usize) -> &Cell {
        &self.space[x][y][z]
    }

    fn active_neighbors(&self, x: usize, y: usize, z: usize) -> usize {
        let mut count = 0;
        let max_index = self.size - 1;
        if x < 1 || x >= max_index
        || y < 1 || y >= max_index
        || z < 1 || z >= max_index
        { return 0 }
        
        for xs in 0..=2usize {
        for ys in 0..=2usize {
        for zs in 0..=2usize {
            if xs == 1 && ys == 1 && zs == 1 { continue; }
            let xi = x+xs-1;
            let yi = y+ys-1;
            let zi = z+zs-1;
            
            match self.get(xi, yi, zi) {
                Cell::Active => count += 1,
                Cell::Inactive => ()
            }
        }}}
        
        count
    }

    fn cycle(&mut self) {
        let mut updates = Vec::<(usize, usize, usize, Cell)>::new();
        for (x, x_slice) in self.space.iter().enumerate() {
            for (y, y_slice) in x_slice.iter().enumerate() {
                for (z, cell) in y_slice.iter().enumerate() {
                    let n = self.active_neighbors(x, y, z);
                    match (cell, n) {
                        (Cell::Active, n) if n!=2 && n!=3 => {
                            updates.push((x,y,z, Cell::Inactive));
                        },
                        (Cell::Inactive, n) if n == 3 => {
                            updates.push((x, y, z, Cell::Active));
                        }
                        _ => ()
                    }
                }
            }
        }
        updates.iter()
        .for_each(|&(x, y,z, new_cell)| self.space[x][y][z] = new_cell);
    }
}

struct Matrix4D {
    size: usize,
    space: Vec<Vec<Vec<Vec<Cell>>>>
}

impl Matrix4D { // Copy pasted and added an extra axis everywhere until the compiler stopped screaming
    fn new(size: usize) -> Self {
        Self {
            size,
            space: vec![vec![vec![vec![Cell::Inactive; size]; size]; size]; size]
        }
    }

    fn get(&self, x:usize, y:usize, z:usize, w:usize) -> &Cell {
        &self.space[x][y][z][w]
    }

    fn active_neighbors(&self, x: usize, y: usize, z: usize, w: usize) -> usize {
        let mut count = 0;
        let max_index = self.size - 1;
        if x < 1 || x >= max_index
        || y < 1 || y >= max_index
        || z < 1 || z >= max_index
        || w < 1 || w >= max_index
        { return count }
        
        for xs in 0..=2usize {
        for ys in 0..=2usize {
        for zs in 0..=2usize {
        for ws in 0..=2usize {
            if xs == 1 && ys == 1 && zs == 1 && ws == 1 { continue; }
            let xi = x+xs-1;
            let yi = y+ys-1;
            let zi = z+zs-1;
            let wi = w+ws-1;
            
            match self.get(xi, yi, zi, wi) {
                Cell::Active => count += 1,
                Cell::Inactive => ()
            }
        }}}}
        
        count
    }

    fn cycle(&mut self) {
        let mut updates = Vec::<(usize, usize, usize, usize, Cell)>::new();
        for (x, x_slice) in self.space.iter().enumerate() {
        for (y, y_slice) in x_slice.iter().enumerate() {
        for (z, z_slice) in y_slice.iter().enumerate() {
        for (w, cell) in z_slice.iter().enumerate() {
            let n = self.active_neighbors(x, y, z, w);
            match (cell, n) {
                (Cell::Active, n) if n!=2 && n!=3 => {
                    updates.push((x, y, z, w, Cell::Inactive));
                },
                (Cell::Inactive, n) if n == 3 => {
                    updates.push((x, y, z, w, Cell::Active));
                }
                _ => ()
            }
        }}}}
        
        updates.iter()
        .for_each(|&(x, y,z, w, new_cell)| self.space[x][y][z][w] = new_cell);
    }
}

fn parse_input(input: String) -> Vec<Vec<Cell>> {
    input.trim()
    .lines()
    .map(|l|
        l.chars()
        .map(|c| c.into())
        .collect()
    )
    .collect()
}

fn part_1(starting_state: &Vec<Vec<Cell>>) -> usize {
    let num_cycles: usize = 6;
    let center = starting_state.len() + (num_cycles*2);
    let mut conway_cube = Matrix3D::new(center*2);
    for (x, y_slice) in starting_state.iter().enumerate() {
        for (y, cell) in y_slice.iter().enumerate() {
            conway_cube.space[center+x][center+y][center] = cell.clone()
        }
    }

    for _ in 0..num_cycles {
        conway_cube.cycle();
    }

    conway_cube.space.iter()
    .flatten()
    .flatten()
    .filter(|&c| match c {
        Cell::Active => true,
        Cell::Inactive => false
    })
    .count()
}

fn part_2(starting_state: &Vec<Vec<Cell>>) -> usize {
    let num_cycles: usize = 6;
    let center = starting_state.len() + (num_cycles*2);
    let mut conway_hypercube = Matrix4D::new(center*2);
    for (x, y_slice) in starting_state.iter().enumerate() {
        for (y, cell) in y_slice.iter().enumerate() {
            conway_hypercube.space[center+x][center+y][center][center] = cell.clone()
        }
    }

    for _ in 0..num_cycles {
        conway_hypercube.cycle();
    }

    conway_hypercube.space.iter()
    .flatten()
    .flatten()
    .flatten()
    .filter(|&c| match c {
        Cell::Active => true,
        Cell::Inactive => false
    })
    .count()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let starting_state = time!(parse_input(input));
    let p1_answer = time!(part_1(&starting_state));
    let p2_answer = time!(part_2(&starting_state));

    println!("Conway Cube: Active cells after 6 cycles: {}", p1_answer);
    println!("Conway Hypercube: Active cells after 6 cycles: {}", p2_answer);
}