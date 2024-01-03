use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<String> {

    let mut v:Vec<String>=Vec::new();
    for line in input.lines(){
        v.push(line.to_string());
    }

    v
}

#[aoc(day6, part1)]
fn solve_part1(input: &Vec<String>) -> String {

    let mut out:String=String::new();
    for i in 0..8 {
        let mut v_occ:HashMap<char, u32>=HashMap::new();
        for j in 0..input.len() {
            let c= input[j].chars().nth(i).unwrap();
            if let Some(v) = v_occ.get_mut(&c) {
                *v+=1;
            } else {
                v_occ.insert(c, 1);
            }
        }
        let c_max = v_occ.iter().max_by_key(|&(_,v)| v).unwrap();
        out.push(*c_max.0);
    }
    out
}
#[aoc(day6, part2)]
fn solve_part2(input: &Vec<String>) -> String {

    let mut out:String=String::new();
    for i in 0..8 {
        let mut v_occ:HashMap<char, u32>=HashMap::new();
        for j in 0..input.len() {
            let c= input[j].chars().nth(i).unwrap();
            if let Some(v) = v_occ.get_mut(&c) {
                *v+=1;
            } else {
                v_occ.insert(c, 1);
            }
        }
        let c_max = v_occ.iter().filter(|&(_,v)|*v>=1).min_by_key(|&(_,v)| v).unwrap();
        out.push(*c_max.0);
    }
    out
}