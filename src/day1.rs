use std::{cmp::{min, max}, collections::btree_map::Range};

#[derive(Debug)]
enum Direction {
    Left(u32),
    Right(u32),
}
#[derive(Debug)]
enum Heading {
    North,
    East,
    South,
    West,
}

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<Direction> {
    let mut v:Vec<Direction>=Vec::new();

    for line in input.lines() {
        for command in line.split(',') {
            let (dir, num)=command.trim().split_at(1);
            println!("Line : {command}");
            match (dir, num) {
                ("L",val) => { v.push(Direction::Left(val.parse::<u32>().unwrap()))},
                ("R",val) => { v.push(Direction::Right(val.parse::<u32>().unwrap()))},
                _ => panic!("Unknown direction"),
            }    
        }
    }
     v
}

#[aoc(day1, part1)]
fn solve_part1(input: &Vec<Direction>) -> u32 {

    let mut x_pos:i32=0;
    let mut y_pos:i32=0;

    let mut heading=Heading::North;

    for dir in input {
        match dir {
            Direction::Left(val) => {
                match heading {
                    Heading::North => { heading=Heading::West; x_pos -= *val as i32; },
                    Heading::South => { heading=Heading::East; x_pos += *val as i32; },
                    Heading::West => { heading=Heading::South; y_pos += *val as i32; },
                    Heading::East => { heading=Heading::North; y_pos -= *val as i32; },
                }
            }
            Direction::Right(val) => {
                match heading {
                    Heading::North => { heading=Heading::East; x_pos += *val as i32; },
                    Heading::South => { heading=Heading::West; x_pos -= *val as i32; },
                    Heading::West => { heading=Heading::North; y_pos -= *val as i32; },
                    Heading::East => { heading=Heading::South; y_pos += *val as i32; },
                }
            }
        }
    }
    let distance = x_pos.abs()+y_pos.abs();
    distance as u32
}


#[aoc(day1, part2)]
fn solve_part2(input: &Vec<Direction>) -> u32 {

    let mut grid:Vec<Vec<u8>>=vec![vec![0;500];500];

    let mut x_pos:u32=250;
    let mut y_pos:u32=250;

    let mut new_x_pos:u32 = 250;
    let mut new_y_pos:u32 = 250;

    let mut heading=Heading::North;

    for dir in input {
        match dir {
            Direction::Left(val) => {
                match heading {
                    Heading::North => { heading=Heading::West; new_x_pos = x_pos - *val; },
                    Heading::South => { heading=Heading::East; new_x_pos = x_pos + *val; },
                    Heading::West => { heading=Heading::South; new_y_pos = y_pos + *val; },
                    Heading::East => { heading=Heading::North; new_y_pos = y_pos - *val; },
                }
            }
            Direction::Right(val) => {
                match heading {
                    Heading::North => { heading=Heading::East; new_x_pos = x_pos + *val; },
                    Heading::South => { heading=Heading::West; new_x_pos = x_pos - *val; },
                    Heading::West => { heading=Heading::North; new_y_pos = y_pos - *val; },
                    Heading::East => { heading=Heading::South; new_y_pos = y_pos + *val; },
                }
            }
        }
        if new_x_pos != x_pos {
            let range: Box<dyn Iterator<Item = u32>> = if new_x_pos < x_pos {
                Box::new((new_x_pos..x_pos).rev())
            } else {
                Box::new((x_pos + 1)..=new_x_pos)
            };
            
            for x in range {
                if grid[x as usize][y_pos as usize]==1 {
                    x_pos=x;
                    return ((x_pos as i32-250).abs()+(y_pos as i32-250).abs()) as u32;
                }
                grid[x as usize][y_pos as usize]=1;
            }
        }
        
        if new_y_pos != y_pos {
            let range: Box<dyn Iterator<Item = u32>> = if new_y_pos < y_pos {
                Box::new((new_y_pos..y_pos).rev())
            } else {
                Box::new((y_pos + 1)..=new_y_pos)
            };
            
            for y in range {
                if grid[x_pos as usize][y as usize]==1 {
                    y_pos=y;
                    return ((x_pos as i32-250).abs()+(y_pos as i32-250).abs()) as u32;
                }
                grid[x_pos as usize][y as usize]=1;
            }
        }
        x_pos=new_x_pos;
        y_pos=new_y_pos;
    }
    0
}



