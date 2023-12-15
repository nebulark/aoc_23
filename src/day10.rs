use std::isize;

use itertools::Itertools;



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

impl Direction {
    pub fn offset(&self, pos : (isize, isize)) -> (isize, isize) {
        match *self {
            Direction::Top => (pos.0 - 1, pos.1),
            Direction::Bottom => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        }
    }
    pub fn inverse(&self) -> Direction {
        match *self {
            Direction::Top => Direction::Bottom,
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    // TBLR
    Ground = 0b0000,
    Animal = 0b1111,
    TB = 0b1100,
    TL = 0b1010,
    TR = 0b1001,
    BL = 0b0110,
    BR = 0b0101,
    LR = 0b0011,
}

impl Tile {
    pub fn from_char(c : char) -> Self {
        match c {
            '|' => Self::TB,
            '-' => Self::LR,
            'L' => Self::TR,
            'J' => Self::TL,
            '7' => Self::BL,
            'F' => Self::BR,
            '.' => Self::Ground,
            'S' => Self::Animal,
            _ => panic!()
        }
    }


    pub fn display(&self) -> char {
        match *self {
            Tile::Ground => ' ',
            Tile::Animal => 'S',
            Tile::TB => '┃',
            Tile::TL => '┗',
            Tile::TR => '┛',
            Tile::BL => '┓',
            Tile::BR => '┏',
            Tile::LR => '━',
        }
    }

    pub fn connects(&self, dir : Direction) -> bool
    {
        match (dir, *self)
        {
            (Direction::Top, Self::TB | Self::TL | Self::TR) => true,
            (Direction::Bottom, Self::TB | Self::BL | Self::BR) => true,
            (Direction::Left, Self::TL | Self::BL | Self::LR) => true,
            (Direction::Right, Self::TR | Self::BR | Self::LR) => true,
            _ => false,         
        }
    }

    pub fn first_dir(&self) -> Direction
    {
        match *self
        {
            Tile::TB => Direction::Top,
            Tile::TL => Direction::Top,
            Tile::TR => Direction::Top,
            Tile::BL => Direction::Bottom,
            Tile::BR => Direction::Bottom,
            Tile::LR => Direction::Left,
            _ => panic!(),         
        }
    }

    pub fn out_dir(&self, in_dir : Direction) -> Direction {
        match (in_dir, *self) {
            (Direction::Top, Self::TB) => Direction::Bottom,
            (Direction::Top, Self::TL) => Direction::Left,
            (Direction::Top, Self::TR) => Direction::Right,

            (Direction::Bottom, Self::TB) => Direction::Top,
            (Direction::Bottom, Self::BL) => Direction::Left,
            (Direction::Bottom, Self::BR) => Direction::Right,

            (Direction::Left, Self::TL) => Direction::Top,
            (Direction::Left, Self::BL) => Direction::Bottom,
            (Direction::Left, Self::LR) => Direction::Right,

            (Direction::Right, Self::TR) => Direction::Top,
            (Direction::Right, Self::BR) => Direction::Bottom,
            (Direction::Right, Self::LR) => Direction::Left,

            _ => panic!("{:?}  {:?}", in_dir, self)
        }
    }
}

pub struct Grid {
    pub grid_data : Vec<Vec<Tile>>
} 
impl Grid {
    pub fn get(&self, (row_idx, colum_idx) : (isize, isize)) -> Tile {
        if 0 <= row_idx && row_idx < self.grid_data.len() as isize {
            let row = &self.grid_data[row_idx as usize];
            if 0 <= colum_idx && colum_idx < row.len() as isize {
                return row[colum_idx as usize];
            }
        }

        Tile::Ground
    }

    pub fn set(&mut self, t : Tile, (row_idx, colum_idx) : (isize, isize)) {
        if 0 <= row_idx && row_idx < self.grid_data.len() as isize {
            let row = &mut self.grid_data[row_idx as usize];
            if 0 <= colum_idx && colum_idx < row.len() as isize {
                row[colum_idx as usize] = t;
            }
        }
    }
}


pub fn part1() {
    

    let mut grid_data : Vec<Vec<Tile>> = Vec::new();

    let mut animal_pos = (isize::MAX, isize::MAX);

    for (line_idx, line) in std::fs::read_to_string("inputs/input_10.txt")
        .unwrap()
        .lines()
        .enumerate()
    {
        let grid_line : Vec<Tile> = line.chars().map(|c|Tile::from_char(c)).collect();

        if let Some(animal_column) = grid_line.iter().position(|e|*e==Tile::Animal)
        {
            animal_pos = (line_idx as isize, animal_column as isize);
        }

        grid_data.push(grid_line);
    }

    let mut grid = Grid { grid_data};

    let t_con = grid.get(Direction::Top.offset(animal_pos)).connects(Direction::Bottom);
    let b_con = grid.get(Direction::Bottom.offset(animal_pos)).connects(Direction::Top);
    let l_con = grid.get(Direction::Left.offset(animal_pos)).connects(Direction::Right);
    let r_con = grid.get(Direction::Right.offset(animal_pos)).connects(Direction::Left);

    let animal_tile = match (t_con, b_con, l_con, r_con) {
        (true, true, false, false) => Tile::TB,
        (true, false, true, false) => Tile::TL,
        (true, false, false, true) => Tile::TR,
        (false, true, true, false) => Tile::BL,
        (false, true, false, true) => Tile::BR,
        (false, false, true, true) => Tile::LR, 
        _ => panic!(),        
    };

    dbg!(animal_tile);

    grid.set(animal_tile, animal_pos);

    let mut current = animal_pos;
    let mut dir = grid.get(animal_pos).first_dir();

    dbg!(current);
    dbg!(dir);

    let mut steps = 0;
    loop {
        steps += 1;
        current = dir.offset(current);
        let tile = grid.get(current);
        dir = tile.out_dir(dir.inverse());

        if current == animal_pos {
            break;
        }
    }

    let farthest = steps / 2;


    println!("day 10 part 1: {}", farthest);
}

pub fn part2() {
    
    let mut grid_data : Vec<Vec<Tile>> = Vec::new();

    let mut animal_pos = (isize::MAX, isize::MAX);

    for (line_idx, line) in std::fs::read_to_string("inputs/input_10.txt")
        .unwrap()
        .lines()
        .enumerate()
    {
        let grid_line : Vec<Tile> = line.chars().map(|c|Tile::from_char(c)).collect();

        if let Some(animal_column) = grid_line.iter().position(|e|*e==Tile::Animal)
        {
            animal_pos = (line_idx as isize, animal_column as isize);
        }

        grid_data.push(grid_line);
    }

    let mut grid = Grid { grid_data};

    let t_con = grid.get(Direction::Top.offset(animal_pos)).connects(Direction::Bottom);
    let b_con = grid.get(Direction::Bottom.offset(animal_pos)).connects(Direction::Top);
    let l_con = grid.get(Direction::Left.offset(animal_pos)).connects(Direction::Right);
    let r_con = grid.get(Direction::Right.offset(animal_pos)).connects(Direction::Left);

    let animal_tile = match (t_con, b_con, l_con, r_con) {
        (true, true, false, false) => Tile::TB,
        (true, false, true, false) => Tile::TL,
        (true, false, false, true) => Tile::TR,
        (false, true, true, false) => Tile::BL,
        (false, true, false, true) => Tile::BR,
        (false, false, true, true) => Tile::LR, 
        _ => panic!(),        
    };

    dbg!(animal_tile);

    grid.set(animal_tile, animal_pos);

    let mut current = animal_pos;
    let mut dir = grid.get(animal_pos).first_dir();

    dbg!(current);
    dbg!(dir);

    let mut steps = 0;
    loop {
        steps += 1;
        current = dir.offset(current);
        let tile = grid.get(current);
        dir = tile.out_dir(dir.inverse());

        if current == animal_pos {
            break;
        }
    }

    let farthest = steps / 2;


    println!("day 10 part 2: {}", farthest);
}
