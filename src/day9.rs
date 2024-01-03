use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<String> {

    let mut v:Vec<String>=Vec::new();
    for line in input.lines(){
        v.push(line.to_string());
    }
    v
}
#[derive(Debug)]
enum DecompressState {
    Normal,
    OpenParenthesis,
    X,
    CloseParenthesis,
    Repeat,
}
#[aoc(day9, part1)]
fn solve_part1(input: &Vec<String>) -> u32 {

    let mut current_state=DecompressState::Normal;
    let mut out:String = String::new();
    let mut repeat_string=String::new();
    let mut chars_counter=0;
    let mut repeat_counter:usize=0;

    for c in input[0].chars(){
        match current_state {
            DecompressState::Normal if c == '(' => current_state=DecompressState::OpenParenthesis,
            DecompressState::Normal => out.push(c),
            DecompressState::OpenParenthesis if c !='x' => {
                chars_counter*=10;
                chars_counter+=(c as u32-b'0' as u32);
            },
            DecompressState::OpenParenthesis if c =='x' => current_state=DecompressState::X,
            DecompressState::X if c != ')' => {
                repeat_counter*=10;
                repeat_counter+=c as usize-b'0' as usize;   
            },
            DecompressState::X => current_state=DecompressState::Repeat,
            DecompressState::Repeat => {
                chars_counter-=1;
                repeat_string.push(c); 
                
                if chars_counter == 0 {
                    out.push_str(repeat_string.repeat(repeat_counter).as_str());
                    repeat_counter=0;
                    repeat_string.clear();
                    current_state=DecompressState::Normal;
                }
            }
            _ => {panic!("Uncknow state : {c}, {:?}", current_state);},
        }
    }
    out.len() as u32
}

#[aoc(day9, part2)]
fn solve_part2(input: &Vec<String>) -> u32 {

    let mut current_state=DecompressState::Normal;
    let mut out:String = String::new();
    let mut repeat_string=String::new();
    let mut chars_counter=0;
    let mut repeat_counter:usize=0;
    let mut input = input[0].clone();

    let mut total_chars=0;

    for c in input.chars(){
        match current_state {
            DecompressState::Normal if c == '(' => current_state=DecompressState::OpenParenthesis,
            DecompressState::Normal => {out.push(c); total_chars+=1;},
            DecompressState::OpenParenthesis if c !='x' => {
                chars_counter*=10;
                chars_counter+=(c as u32-b'0' as u32);
            },
            DecompressState::OpenParenthesis if c =='x' => current_state=DecompressState::X,
            DecompressState::X if c != ')' => {
                repeat_counter*=10;
                repeat_counter+=c as usize-b'0' as usize;   
            },
            DecompressState::X => current_state=DecompressState::Repeat,
            DecompressState::Repeat => {
                total_chars+=(repeat_counter as u32*chars_counter);
                current_state=DecompressState::Normal;
            }
            _ => {panic!("Uncknow state : {c}, {:?}", current_state);},
        }
    }

    total_chars as u32
}
