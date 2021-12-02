use std::{fs::read_to_string, str::FromStr};

fn main() {
    let instructions = get_instructions();

    let part1 = Location::new(0, 0).travel(&instructions);
    let part2 = AimedLocation::new(0, 0, 0).travel(&instructions);

    println!("{}", part1.get_distance() * part1.get_depth());
    println!("{}", part2.get_distance() * part2.get_depth());
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
    fn apply_instruction(&self, ins: &Instruction) -> Self;
    fn travel(&self, instructions: &Vec<Instruction>) -> Self;
}

impl AimedLocation {
    pub fn new(depth: i32, distance: i32, aim: i32) -> Self {
        Self::new_from_location(Location::new(depth, distance), aim)
    }

    pub fn new_from_location(location: Location, aim: i32) -> Self {
        Self { location, aim }
    }
}

impl Submarine for AimedLocation {
    fn get_depth(&self) -> i32 {
        self.location.get_depth()
    }

    fn get_distance(&self) -> i32 {
        self.location.get_distance()
    }

    fn travel(&self, instructions: &Vec<Instruction>) -> AimedLocation {
        instructions
            .iter()
            .fold(self.clone(), |location, instruction| {
                location.apply_instruction(instruction)
            })
    }

    fn apply_instruction(&self, ins: &Instruction) -> AimedLocation {
        let Location { depth, distance } = self.location.clone();
        match ins.direction {
            Direction::Up => {
                AimedLocation::new_from_location(self.location.clone(), self.aim - ins.distance)
            }
            Direction::Down => {
                AimedLocation::new_from_location(self.location.clone(), self.aim + ins.distance)
            }
            Direction::Forward => AimedLocation::new(
                depth + self.aim * ins.distance,
                distance + ins.distance,
                self.aim,
            ),
        }
    }
}

impl Location {
    pub fn new(depth: i32, distance: i32) -> Location {
        Location { depth, distance }
    }
}

impl Submarine for Location {
    fn get_depth(&self) -> i32 {
        self.depth
    }

    fn get_distance(&self) -> i32 {
        self.distance
    }

    fn apply_instruction(&self, ins: &Instruction) -> Location {
        match ins.direction {
            Direction::Up => Location::new(self.depth - ins.distance, self.distance),
            Direction::Down => Location::new(self.depth + ins.distance, self.distance),
            Direction::Forward => Location::new(self.depth, self.distance + ins.distance),
        }
    }

    fn travel(&self, instructions: &Vec<Instruction>) -> Location {
        instructions
            .iter()
            .fold(self.clone(), |location, instruction| {
                location.apply_instruction(instruction)
            })
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
