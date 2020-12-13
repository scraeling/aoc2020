// This one turned into something pretty ugly, but I don't want to rewrite it >_<
use std::fs::read_to_string;
use timer::time;

#[derive(Copy, Clone, Debug)]
enum Action {
    N(i64),
    E(i64),
    W(i64),
    S(i64),
    F(i64),
    L(i64),
    R(i64),
}

impl From<&str> for Action {
    fn from(input: &str) -> Self {
        let mut chars = input.chars();
        let action = chars.next().unwrap();
        let value = chars.collect::<String>().parse::<i64>().unwrap();

        use Action::*;
        match action {
            'N' => N(value),
            'E' => E(value),
            'W' => W(value),
            'S' => S(value),
            'F' => F(value),
            'R' => R(value),
            'L' => L(value),
            c => panic!("Invalid input: {}", c)
        }
    }
}

impl From<i64> for Action {
    fn from(angle: i64) -> Self {
        use Action::*;
        let mut limited = angle % 360;
        if limited < 0 {
            limited += 360;
        }

        match limited {
            0 => N(0),
            90 => E(0),
            180 => S(0),
            270 => W(0),
            a => panic!("Invalid angle: {}", a)
        }
    }
}

impl Action {
    fn from_waypoint(pos: (i64, i64)) -> Vec<Action> {
        use Action::*;
        vec![E(pos.0), S(pos.1)]
    }

    fn rotate_waypoint(&self, pos: (i64, i64)) -> (i64, i64) {
        use Action::*;
        match *self {
            L(value)|R(value) if value == 180 => (-pos.0, -pos.1),
            L(90)|R(270) => (pos.1, -pos.0),
            R(90)|L(270) => (-pos.1, pos.0),
            _ => panic!("Bad rotation")
        }
    }

    fn apply_movement(&self, pos: &(i64, i64)) -> (i64, i64) {
        use Action::*;
        match *self {
            N(distance) => (pos.0, pos.1 - distance),
            E(distance) => (pos.0 + distance, pos.1),
            W(distance) => (pos.0 - distance, pos.1),
            S(distance) => (pos.0, pos.1 + distance),
            _ => panic!("Invalid direction")
        }
    }

    fn direction_to_angle(&self) -> i64 {
        use Action::*;
        match *self {
            N(_) => 0,
            E(_) => 90,
            S(_) => 180,
            W(_) => 270,
            _ => panic!("Invalid direction")
        }
    }

    fn apply_rotation(&self, orientation: &Action) -> Action {
        use Action::*;
        match *self {
            L(angle) => (orientation.direction_to_angle() - angle).into(),
            R(angle) => (orientation.direction_to_angle() + angle).into(),
            _ => panic!("Invalid rotation")
        }
    }

    fn forward_to_direction(&self, orientation: &Action) -> Self {
        use Action::*;
        let value = match *self {F(val) => val, _ => panic!()};
        match orientation {
            N(_) => N(value),
            S(_) => S(value),
            E(_) => E(value),
            W(_) => W(value),
            _ => panic!("Invalid Orientation")
        }
    }
}

struct Ship {
    pos: (i64, i64),
    orientation: Action
}

impl Ship {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            orientation: Action::E(0)
        }
    }

    fn navigate(&mut self, action: Action) {
        use Action::*;
        match action {
            N(_)|S(_)|E(_)|W(_) => self.pos = action.apply_movement(&self.pos),
            L(_)|R(_) => self.orientation = action.apply_rotation(&self.orientation),
            F(_) => self.pos = action.forward_to_direction(&self.orientation).apply_movement(&self.pos)
        }
    }
}

fn parse_input(input: String) -> Vec<Action> {
    input.split("\n")
    .map(|a| a.into())
    .collect()
}

fn part_1(actions: &Vec<Action>) -> i64 {
    let mut ship = Ship::new();
    let mut waypoint = Ship::new();
    waypoint.pos = (10, -1);
    for a in actions.clone() {
        ship.navigate(a);
    }
    ship.pos.0.abs() + ship.pos.1.abs()
}

fn part_2(actions: &Vec<Action>) -> i64 {
    let mut ship = Ship::new();
    let mut waypoint = Ship::new();
    waypoint.pos = (10, -1);

    use Action::*;
    for a in actions {
        match *a {
            N(_)|E(_)|W(_)|S(_) => waypoint.navigate(a.clone()),
            F(times) => {
                for a in Action::from_waypoint(waypoint.pos) {
                    for _ in 0..times {
                        ship.navigate(a.clone())
                    }
                }
            }
            L(_)|R(_) => {
                waypoint.pos = a.rotate_waypoint(waypoint.pos);
            }
        }
    }

    ship.pos.0.abs() + ship.pos.1.abs()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let actions = time!(parse_input(input));
    let p1_answer = time!(part_1(&actions));
    let p2_answer = time!(part_2(&actions));

    println!("Manhattan distance when following actions directly: {}", p1_answer);
    println!("Manhattan distance when using waypoint: {}", p2_answer);
}