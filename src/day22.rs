use std::{collections::{HashSet, HashMap}, cmp::max};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    size:u32,
    used:u32,
    avail:u32,
}
#[derive(Debug, PartialEq, Eq)]
struct Grid {
    nodes: HashMap<(u32,u32), Node>,
    max_x:u32,
    max_y:u32,
    largest:u32,
}

#[aoc_generator(day22)]
fn input_generator(input: &str) -> Grid {
    
    let mut grid=Grid{nodes: HashMap::new(), max_x:0,max_y: 0, largest:0};
    let re_disk=Regex::new(r".*node-x(\d*)-y(\d*)\s*(\d*)T\s*(\d*)T\s*(\d*)T").unwrap();
    
    for line in input.lines(){
        if let Some(caps) = re_disk.captures(line) {
            grid.nodes.insert(
                (caps[1].parse::<u32>().unwrap(),caps[2].parse::<u32>().unwrap()),
             Node {
                size: caps[3].parse::<u32>().unwrap(),
                used: caps[4].parse::<u32>().unwrap(),
                avail: caps[5].parse::<u32>().unwrap()
            });

            grid.max_x=max(grid.max_x, caps[1].parse::<u32>().unwrap());
            grid.max_y=max(grid.max_y, caps[2].parse::<u32>().unwrap());
            grid.largest=max(grid.largest, caps[5].parse::<u32>().unwrap());
        }
    }
    grid
}


#[aoc(day22, part1)]
fn solve_part1(g: &Grid) -> u32 {

    println!("Nodes :{}", g.nodes.len());

    let count=g.nodes.iter().filter(|a| a.1.used!=0).fold(0, |sum, a| 
    {
        sum + g.nodes
        .iter()
        .filter(|b| a.0 != b.0 && a.1.used <= b.1.avail)
        .count()
    });
    count as u32
}

#[aoc(day22, part2)]
fn solve_part2(g: &Grid) -> u32 {

    println!("Nodes :{}", g.nodes.len());
    for y in 0..=g.max_y{
        for x in 0..=g.max_x{
            let used = g.nodes.get(&(x,y)).unwrap().used;
            if y==0&&x==g.max_x{
                print!("G");
            }else if used ==0{
                print!("_");
            } else if used<g.largest{
                print!(".");                
            } else {
                print!("#");
            }
        }
        println!();
    }
    0
}