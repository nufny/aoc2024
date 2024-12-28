use std::collections::HashSet;
#[derive(PartialEq,Eq,Hash,Clone,Copy,Debug)]
struct Position {
    x: usize,
    y: usize,
}
impl Position {
    fn step(&self, rhs: &Direction) -> Self {
        
        match rhs {
            Direction::Up => Position {
                x:self.x,
                y:self.y-1,
            },
            Direction::Down => Position {
                x:self.x,
                y:self.y+1,
            },
            Direction::Right => Position {
                x:self.x+1,
                y:self.y,
            },
            Direction::Left => Position {
                x:self.x-1,
                y:self.y,
            }
        }
    }

}
struct Simulation {
    obstacles: HashSet<Position>,
    guard: Npc,
    visited: HashSet<Position>,
    edge: Position,
}

impl Simulation {

    fn run(&mut self) -> u32 {

        while self.check_edge(){
            if self.obstacles.contains(&self.guard.probe()) {
                self.guard.turn();
            } else{
                self.visited.insert(self.guard.position.clone());
                self.guard.step();
            }

        }
        self.visited.len() as u32 + 1
    }

    fn check_edge(&self) -> bool {
        let guard_pos = &self.guard.position;
        
        0 < guard_pos.x && 
        0 < guard_pos.y &&        
        guard_pos.x < self.edge.x && 
        guard_pos.y < self.edge.y
    }
}
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
struct Npc {
    facing: Direction,
    position: Position,
}
impl Npc {
    fn probe(&self) -> Position{
        self.position.step(&self.facing)
    }
    fn step(&mut self) {
        self.position = self.position.step(&self.facing)
    } 
    fn turn(&mut self) {
        self.facing = match self.facing {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,

        };
    }
}



fn parse(input: String) -> Simulation {
    let mut obstacles = HashSet::new();
    let mut guard = None;
    let lines: Vec<(usize, &str)> = input.lines().enumerate().collect();
    let edge = Position {
        x: lines[0].1.len() - 1,
        y: lines.len() - 1,
    };
    for (y,line) in lines {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {obstacles.insert(Position{x,y});},
                '^' => guard = Some(Npc{facing: Direction::Up, position: Position{x,y}}),
                _ => (),
            };
        }
    }
    
    Simulation {
        obstacles,
        guard: guard.expect("No Guard found in Input"),
        visited: HashSet::new(),
        edge, 
    }
}

pub mod p1 {
    use super::parse;
    

    pub fn run(_input: String) -> u32 {
        let mut sim = parse(_input);
        sim.run()
    }
}

pub mod p2 {
    pub fn run(_input: String) -> u32 {
        0
    }
}
