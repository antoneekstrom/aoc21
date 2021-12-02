use std::{fs::read_to_string, str::FromStr};

fn main() {
    let instructions = get_instructions();

    let mut part1 = Location::new(0, 0);
    let mut part2 = AimedLocation::new(0, 0, 0);

    let subs: Vec<&mut dyn Submarine> = vec![&mut part1, &mut part2];

    for sub in subs {
        sub.travel(&instructions);
        println!("{}", sub.get_result());
    }
}

fn get_instructions() -> Vec<Instruction> {
    get_input()
        .lines()
        .map(&str::parse::<Instruction>)
        .map(Result::unwrap)
        .collect::<Vec<_>>()
}

fn get_input<'a>() -> String {
    read_to_string("input").expect("Failed to read input")
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i32,
}

#[derive(Clone, Debug)]
struct Location {
    depth: i32,
    distance: i32,
}

#[derive(Clone, Debug)]
struct AimedLocation {
    location: Location,
    aim: i32,
}

trait Submarine {
    fn get_depth(&self) -> i32;
    fn get_distance(&self) -> i32;
    fn execute(&mut self, ins: &Instruction);

    fn travel(&mut self, instructions: &Vec<Instruction>) {
        instructions.iter().for_each(|ins| self.execute(ins))
    }

    fn get_result(&self) -> i32 {
        self.get_distance() * self.get_depth()
    }
}

impl AimedLocation {
    pub fn new(depth: i32, distance: i32, aim: i32) -> Self {
        Self::new_from_location(Location::new(depth, distance), aim)
    }

    pub fn new_from_location(location: Location, aim: i32) -> Self {
        Self { location, aim }
    }

    fn set_from_location(&mut self, location: Location, aim: i32) {
        self.location = location;
        self.aim = aim;
    }

    fn set(&mut self, depth: i32, distance: i32, aim: i32) {
        self.set_from_location(Location::new(depth, distance), aim);
    }
}

impl Submarine for AimedLocation {
    fn get_depth(&self) -> i32 {
        self.location.get_depth()
    }

    fn get_distance(&self) -> i32 {
        self.location.get_distance()
    }

    fn execute(&mut self, ins: &Instruction) {
        let Location { depth, distance } = self.location.clone();
        match ins.direction {
            Direction::Up => self.set_from_location(self.location.clone(), self.aim - ins.distance),
            Direction::Down => {
                self.set_from_location(self.location.clone(), self.aim + ins.distance)
            }
            Direction::Forward => self.set(
                depth + self.aim * ins.distance,
                distance + ins.distance,
                self.aim,
            ),
        };
    }
}

impl Location {
    pub fn new(depth: i32, distance: i32) -> Location {
        Location { depth, distance }
    }

    fn set(&mut self, depth: i32, distance: i32) {
        self.depth = depth;
        self.distance = distance;
    }
}

impl Submarine for Location {
    fn get_depth(&self) -> i32 {
        self.depth
    }

    fn get_distance(&self) -> i32 {
        self.distance
    }

    fn execute(&mut self, ins: &Instruction) {
        match ins.direction {
            Direction::Up => self.set(self.depth - ins.distance, self.distance),
            Direction::Down => self.set(self.depth + ins.distance, self.distance),
            Direction::Forward => self.set(self.depth, self.distance + ins.distance),
        };
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err(format!("Unknown direction: {}", s)),
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = s
            .chars()
            .take_while(|c| c.is_alphabetic())
            .collect::<String>();

        let distance = s
            .chars()
            .skip(dir.len() + 1)
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        Ok(Instruction {
            direction: dir.parse().unwrap(),
            distance,
        })
    }
}
