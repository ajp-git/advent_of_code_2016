use std::result;

use regex::Regex;
#[derive(Debug, Clone)]
struct Disc{
    num:u32,
    holes:u32,
    initial:u32,
}

impl Disc {
    fn get_pos(&self, turn:u32) -> u32 {
        (self.num+self.initial+turn) % self.holes
    }
    fn new(line:&str) -> Self {
        let re=Regex::new(r"Disc #(\d*) has (\d*) positions; at time=(\d*), it is at position (\d*)").unwrap();
        let caps=re.captures(&line).unwrap();
        Disc{
            num: caps[1].parse::<u32>().unwrap(),
            holes: caps[2].parse::<u32>().unwrap(),
            initial: caps[4].parse::<u32>().unwrap()}
    }
}

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<Disc> {
    let mut v:Vec<Disc>=Vec::new();
    for line in input.lines(){
        v.push(Disc::new(&line));
    }
    v
}

#[aoc(day15, part1)]
fn solve_part1(input: &Vec<Disc>) -> u32 {
    let mut result:u32=0;

    loop {
        let total:u32 =input.iter().map(|f| f.get_pos(result)).sum::<u32>();
        if total == 0 {
            break; 
        }
        result+=1;
    }
    result
}

#[aoc(day15, part2)]
fn solve_part2(input: &Vec<Disc>) -> u32 {

    let mut input2=input.clone();
    input2.push(Disc { num: 7, holes: 11, initial: 0 });

    let mut result:u32=0;

    loop {
        let total:u32 =input2.iter().map(|f| f.get_pos(result)).sum::<u32>();
        if total == 0 {
            break; 
        }
        result+=1;
    }
    result
}