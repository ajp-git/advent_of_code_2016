use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day18)]
fn input_generator(input: &str) -> String {
    input.lines().next().unwrap().to_string()
}

#[aoc(day18, part1)]
fn solve_part1(input: &String) -> u32 {
//    let input=".^^.^.^^^^".to_string();
    println!("Using input :\t\t\t{}", input.clone());

    let mut v:Vec<String>=Vec::new();
    let mut current_line=input.clone();
    let mut safe_tiles = current_line.chars().filter(|&c|c=='.').count() as u32;


    for i in 1..40 {
        current_line=format!(".{}.",current_line);
        let v_line:Vec<char>=current_line.chars().collect();
        let mut new_line=String::new();

        for j in 1..current_line.len()-1 {
            match (v_line[j-1], v_line[j], v_line[j+1]) {
                ('^','^','.') | 
                ('.','^','^') |
                ('^', '.', '.') |
                ( '.', '.', '^')   => new_line.push('^'),
                _=>new_line.push('.'),
            }
        }
        print!("Iter {i} :\t{}", &new_line);
        current_line=new_line;//(&new_line[1..new_line.len()]).to_string();
        println!("\t{}", &current_line);
        safe_tiles+=current_line.chars().filter(|&c|c=='.').count() as u32;

    }
    safe_tiles
}

#[aoc(day18, part2)]
fn solve_part2(input: &String) -> u32 {
    0
}
