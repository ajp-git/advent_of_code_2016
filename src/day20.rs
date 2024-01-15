use std::cmp::{Ordering, min};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
#[derive(Debug, PartialEq, Eq, Clone)]
struct Plage{
    low:u32,
    high:u32,
}
 impl PartialOrd for Plage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
 }
impl Ord for Plage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.low < other.low{return  Ordering::Less;}
        Ordering::Greater
    }
    
}
#[aoc_generator(day20)]
fn input_generator(input: &str) -> Vec<Plage> {
    let mut v:Vec<Plage>=Vec::new();
    let re=Regex::new(r"(\d*)-(\d*)").unwrap();
    for line in input.lines(){
        let caps=re.captures(line).unwrap();
        v.push(Plage{
            low:caps[1].parse::<u32>().unwrap(),
            high:caps[2].parse::<u32>().unwrap() }
        );
    }
    v.sort();
    v
}

#[aoc(day20, part1)]
fn solve_part1(v: &Vec<Plage>) -> u32 {

    let mut min_found=u32::MAX;

    for p in v.iter(){
        let possible=min(p.high as u64+1 as u64, u32::MAX as u64) as u32;
        let mut bfound=false;
        for p2 in v.iter(){
            if possible>=p2.low && possible <=p2.high{
                bfound=true;
                break;
            }
        }
        if !bfound{
            if possible<min_found{
                min_found=possible;
            }
        }
    }
    min_found
}

 #[aoc(day20, part2)]
fn solve_part2(v: &Vec<Plage>) -> u32 {
    let mut sv:Vec<Plage>=v.clone();
    let mut opened:u32=0;
    sv.sort();
    let mut curr_low:u32 =0;
    let mut curr_high:u32=0;
    for p in sv {
        if p.low <= curr_high  {
            if p.high>curr_high {
                curr_high=p.high;
            }
        } else {
            opened+=p.low-curr_high-1;
            curr_low=p.low;
            curr_high=p.high;
        }
    }
    opened
}