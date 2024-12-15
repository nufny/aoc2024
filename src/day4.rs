use std::ops::Add;


enum Direction {
    No,
    N,
    E,
    S,
    W,
    NE,
    SE,
    SW,
    NW,
    Vert,
    Hori,
    Right,
    Left,
    Up,
    Down,
    Yes,
}impl Direction{
    fn to_number(&self) -> u32 {
        match self {
            Direction::No => 0,
            Direction::N => 1,
            Direction::E => 2,
            Direction::NE => 3,
            Direction::S => 4,
            Direction::Vert => 5,
            Direction::SE => 6,
            Direction::Right => 7,
            Direction::W => 8,
            Direction::NW => 9,
            Direction::Hori => 10,
            Direction::Up => 11,
            Direction::SW => 12,
            Direction::Left => 13,
            Direction::Down => 14,
            Direction::Yes => 15,
        }
    }

    fn from_number(num: u32) -> Self {
        match num {
            0 => Direction::No,
            1 =>  Direction::N,
            2 =>  Direction::E,
            3 =>  Direction::NE,
            4 =>  Direction::S,
            5 =>  Direction::Vert,
            6 =>  Direction::SE,
            7 =>  Direction::Right,
            8 =>  Direction::W,
            9 =>  Direction::NW,
            10 =>  Direction::Hori,
            11 =>  Direction::Up,
            12 =>  Direction::SW,
            13 =>  Direction::Left,
            14 =>  Direction::Down,
            15 =>  Direction::Yes,
            16.. => panic!(),
        }
    }

    fn contains(&self, other:Self) -> bool {
        self.to_number() & other.to_number() == other.to_number()
    }
} 
impl Add for Direction {
    type Output = Direction;
    fn add(self, rhs: Self) -> Self::Output {
        Direction::from_number(self.to_number() | rhs.to_number())        
    }
}




pub mod p1 {
    use super::Direction;
    struct Wordsearch {
        matrix: Vec<Vec<char>>
    } 
impl Wordsearch{
    fn find_all_occurences(&self, target: &str) -> u32 {
            
            let t_len = target.len();
    
            let y_len = self.matrix.len();
            let x_len = self.matrix[0].len();
    
            let mut counter = 0;
            for y in 0..y_len {
                for x in 0..x_len {
                    let mut direction = Direction::No;
                    if y >= t_len-1 {
                        direction = direction + Direction::N;
                    }
                    if x <= x_len -t_len {
                        direction = direction + Direction::E;
                    }
                    if y <= y_len - t_len {
                        direction = direction + Direction::S;
                    }
                    if x >= t_len-1 {
                        direction = direction + Direction::W;
                    }
                    counter += &self.search_sequence(target.chars().collect(), direction, x,y)
                }
            }
        
            counter
        }
    
    
    fn search_sequence(&self, target:Vec<char>, direction: Direction, x:usize ,y:usize) -> u32 {
        let t_len = target.len();
        let mut counter = 0;
        if direction.contains(Direction::N){
            //North search
            let mut found = true;
            for i in 0..t_len {
                if self.matrix[y-i][x] != target[i]{
                    found = false;
                }
            }
            if found {
                counter += 1;
            }
        }
        if direction.contains(Direction::E){
            //East search
            let mut found = true;
            for i in 0..t_len {
                if self.matrix[y][x+i] != target[i]{
                    found = false;
                }
            }
            if found {
                counter += 1;
            }
        }
        if direction.contains(Direction::S){
            //South search
            let mut found = true;
            for i in 0..t_len {
                if self.matrix[y+i][x] != target[i]{
                    found = false;
                }
            }
            if found {
                counter += 1;
            }
        }
        if direction.contains(Direction::W){
            //West search
            let mut found = true;
            for i in 0..t_len {
                if self.matrix[y][x-i] != target[i]{
                    found = false;
                }
            }
            if found {
                counter += 1;
            }
        }
        if direction.contains(Direction::NE){
            //Northeast search
            let mut found = true;
            for i in 0..t_len {
                if self.matrix[y-i][x+i] != target[i]{
                    found = false;
                }
            }
            if found {
                counter += 1;
            }
        }
        
        if direction.contains(Direction::SE){
            //Southeast search
            let mut found = true;
            for i in 0..t_len {
                if self.matrix[y+i][x+i] != target[i]{
                    found = false;
                }
            }
            if found {
                counter += 1;
            }
        }
        
        if direction.contains(Direction::SW){
            //Southwest search
            let mut found = true;
            for i in 0..t_len {
                if self.matrix[y+i][x-i] != target[i]{
                    found = false;
                }
            }
            if found {
                counter += 1;
            }
        }
        
        if direction.contains(Direction::NW){
            //Northwest search
            let mut found = true;
            for i in 0..t_len {
                if self.matrix[y-i][x-i] != target[i]{
                    found = false;
                }
            }
            if found {
                counter += 1;
            }
        }
        
        
        counter
    }
}

    pub fn run(input: String) -> u32 {
        let matrix: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
        let wordsearch = Wordsearch{matrix};
        wordsearch.find_all_occurences("XMAS")
    }
}

pub mod p2 {

    struct Wordsearch {
        matrix: Vec<Vec<char>>
    } 
    impl Wordsearch{
        fn find_all_occurences(&self, target: &str) -> u32 {
            
            let y_len = self.matrix.len();
            let x_len = self.matrix[0].len();
    
            let mut counter = 0;
            for y in 0..y_len {
                for x in 0..x_len {
                    if 
                    y >= 1 &&
                    x >= 1 &&
                    y < y_len -1 &&
                    x < x_len - 1
                    {
                    counter += &self.search_sequence(target.chars().collect(), x,y)
                    }
                }
            }
        
            counter
        }
    
    
    fn search_sequence(&self, target:Vec<char>, x:usize ,y:usize) -> u32 {
        let t_len = target.len();
        let mut counter = 0;
        {
        //North search
        //1.1
        //.2.
        //3.3
        let mut found = true;
        for i in 0..t_len {
            if 
            self.matrix[y+i-1][x+i-1] != target[i] //NW
            || 
            self.matrix[y+i-1][x+1-i] != target[i] //NE
            {
                found = false;
                break;
            }
        }
        if found {
            counter += 1;
        }
        }
        {
        //East search
        //3.1
        //.2.
        //3.1
        let mut found = true;
        for i in 0..t_len {
            if 
            self.matrix[y+i-1][x+1-i] != target[i] //NE
            || 
            self.matrix[y+1-i][x+1-i] != target[i] //SE
            {
                found = false;
                break;
            }
        }
        if found {
            counter += 1;
        }
        }
        {
        //South search
        //3.3
        //.2.
        //1.1
        let mut found = true;
        for i in 0..t_len {
            if 
            self.matrix[y+1-i][x+1-i] != target[i] //SE
            || 
            self.matrix[y+1-i][x+i-1] != target[i] //SW
            {
                found = false;
                break;
            }
        }
        if found {
            counter += 1;
        }
        }
        {
        //West search
        //1.3
        //.2.
        //1.3
        let mut found = true;
        for i in 0..t_len {
            if 
            self.matrix[y+1-i][x+i-1] != target[i] //SW
            || 
            self.matrix[y+i-1][x+i-1] != target[i] //NW
            {
                found = false;
                break;
            }
        }
        if found {
            counter += 1;
        }
        }
        counter
    }
}

    pub fn run(input: String) -> u32 {
        let matrix: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
        let wordsearch = Wordsearch{matrix};
        wordsearch.find_all_occurences("MAS")
        
    }
}
