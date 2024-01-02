use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<String> {

    let mut v:Vec<String>=Vec::new();
    for line in input.lines(){
        v.push(line.to_string());
    }
    v
}

enum Command {
    Rect{width:usize, height:usize},
    RotateColumn{x:usize, size:usize},
    RotateRow{y:usize, size: usize},
}
struct Screen{
    grid:Vec<Vec<bool>>
}

impl Screen {
    fn new(x: usize, y: usize) -> Screen {
        let mut grid = Vec::with_capacity(y); // Use y for height

        for _ in 0..y {
            let row = vec![false; x]; // Create a row with x elements
            grid.push(row);
        }
        Screen { grid }
    }


    fn rotate_column(self: &mut Self, x:usize, count:usize){
        for i in 0..count {
            self.rotate_column_once(x);            
        }
    }

      // Correct the indices for the rect function
      fn rect(&mut self, width: usize, height: usize) {
        for i in 0..height { // Use height for the outer loop
            for j in 0..width { // Use width for the inner loop
                self.grid[i][j] = true;
            }
        }
    }

    // Correct the rotation logic for columns and rows
    fn rotate_column_once(&mut self, x: usize) {
        let height = self.grid.len();
        let last = self.grid[height - 1][x]; // Store the last element
        for i in (1..height).rev() {
            self.grid[i][x] = self.grid[i - 1][x]; // Shift elements down
        }
        self.grid[0][x] = last; // Place the last element at the start
    }

    fn rotate_row_once(&mut self, y: usize) {
        let width = self.grid[0].len();
        let last = self.grid[y][width - 1]; // Store the last element
        for i in (1..width).rev() {
            self.grid[y][i] = self.grid[y][i - 1]; // Shift elements right
        }
        self.grid[y][0] = last; // Place the last element at the start
    }

    fn rotate_row(self: &mut Self, y:usize, count:usize){
        for i in 0..count {
            self.rotate_row_once(y);            
        }
    }

    fn execute_command(self: &mut Self, command:Command){
        match command {
            Command::Rect { width, height } => {self.rect(width, height)},
            Command::RotateColumn { x, size } => {self.rotate_column(x, size)},
            Command::RotateRow { y, size } => {self.rotate_row(y, size)},
        }
    }

    fn count_lit (self: &Self) -> usize{
        self.grid.iter().flatten().filter(|&&f| f).count()
    }

    fn print(self:&Self){
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                print!("{}", 
                match self.grid[y][x] {
                    true => '#',
                    false => ' ',
                })             
            }
            println!();
        }
    }
}
#[aoc(day8, part1)]
fn solve_part1(input: &Vec<String>) -> u32 {
    
    let mut v_commands:Vec<Command>=Vec::new();
    for line in input.iter(){
        v_commands.push(parse_command(&line));
    }

    let mut screen=Screen::new(50, 6);
    for command in v_commands{
        screen.execute_command(command);
    }
    screen.count_lit() as u32
}

fn parse_command(line: &String) -> Command {
    let re_rect=Regex::new(r"rect (\d*)x(\d*)").unwrap();
    let re_rc=Regex::new(r"rotate column x=(\d*) by (\d*)").unwrap();
    let re_rr=Regex::new(r"rotate row y=(\d*) by (\d*)").unwrap();
    
    if let Some(caps)=re_rect.captures(line.as_str()){
        return Command::Rect { 
            width: caps[1].parse::<usize>().unwrap(),
            height: caps[2].parse::<usize>().unwrap()
        };
    } else if let Some(caps)=re_rc.captures(line.as_str()) {
        return Command::RotateColumn { 
            x: caps[1].parse::<usize>().unwrap(),
            size: caps[2].parse::<usize>().unwrap()
        };        
    } else if let Some(caps)=re_rr.captures(line.as_str()) {
        return Command::RotateRow { 
            y: caps[1].parse::<usize>().unwrap(),
            size: caps[2].parse::<usize>().unwrap()
        };        
    } else {
        panic!("Unknown command");
    }
}

#[aoc(day8, part2)]
fn solve_part2(input: &Vec<String>) -> u32 {
    
    let mut v_commands:Vec<Command>=Vec::new();
    for line in input.iter(){
        v_commands.push(parse_command(&line));
    }

    let mut screen=Screen::new(50, 6);
    for command in v_commands{
        screen.execute_command(command);
    }
    screen.print();
    screen.count_lit() as u32
}
