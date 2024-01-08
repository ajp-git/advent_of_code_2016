use std::collections::{HashMap};

use regex::Regex;

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Vec<String> {

    let mut v:Vec<String>=Vec::new();
    for line in input.lines(){
        v.push(line.to_string());
    }
    v
}

struct Register {
    name:char,
    val:i32,
}

#[derive(Debug)]
enum Instruction {
    CpyReg(char,char),
    CpyVal(i32,char),
    JnzReg(char,i32),
    JnzVal(u32,i32),
    Inc(char),
    Dec(char),
}

#[aoc(day12, part1)]
fn solve_part1(input: &Vec<String>) -> u32 {
    let mut v_instructions:Vec<Instruction>=Vec::new();
    let mut h_registers:HashMap<char, i32>=[
        ('a', 0),
        ('b', 0),
        ('c', 0),
        ('d', 0) ]
        .iter()
        .cloned()
        .collect();

    let re_cpyreg=Regex::new(r"cpy ([a-d]) ([a-d])").unwrap();
    let re_cpyval=Regex::new(r"cpy (\d*) ([a-d])").unwrap();
    let re_jnzreg=Regex::new(r"jnz ([a-d]) ([-]?\d*)").unwrap();
    let re_jnzval=Regex::new(r"jnz (\d*) ([-]?\d*)").unwrap();
    let re_dec=Regex::new(r"dec ([a-d])").unwrap();
    let re_inc=Regex::new(r"inc ([a-d])").unwrap();

    for line in input{
        if let Some(caps) = re_cpyval.captures(line){
            v_instructions.push(Instruction::CpyVal(caps[1].parse::<i32>().unwrap(), caps[2].chars().next().unwrap()))
        } else if let Some(caps) = re_cpyreg.captures(line){
            v_instructions.push(Instruction::CpyReg(caps[1].chars().next().unwrap(), caps[2].chars().next().unwrap()))
        } else if let Some(caps) = re_jnzreg.captures(line){
        v_instructions.push(Instruction::JnzReg(caps[1].chars().next().unwrap(), caps[2].parse::<i32>().unwrap()))
        } else if let Some(caps) = re_jnzval.captures(line){
            v_instructions.push(Instruction::JnzVal(caps[1].parse::<u32>().unwrap(), caps[2].parse::<i32>().unwrap()))
        } else if let Some(caps) = re_inc.captures(line){
            v_instructions.push(Instruction::Inc(caps[1].chars().next().unwrap()))
        } else if let Some(caps) = re_dec.captures(line){
            v_instructions.push(Instruction::Dec(caps[1].chars().next().unwrap()))
        } else {
            panic!("Bad instruction {line:?}");
        }
    }

    let mut addr:i32=0;
    while  addr>=0 && addr<v_instructions.len() as i32{
        
//        println!("{:?}", v_instructions[addr as usize]);

        match v_instructions[addr as usize] {
            Instruction::CpyVal(val, reg) => {h_registers.insert(reg, val); addr+=1; },
            Instruction::CpyReg(reg_src, reg_dest) => {
            
                h_registers.insert(reg_dest, *h_registers.get(&reg_src).unwrap());
                addr+=1; 
            },
            Instruction::Dec(reg) => { 
                if let Some(v) = h_registers.get_mut(&reg) {
                    *v-=1;
                    addr+=1;
                }
            },
            Instruction::Inc(reg)=> { 
                if let Some(v) = h_registers.get_mut(&reg) {
                    *v+=1;
                    addr+=1;
                }
            },
            Instruction::JnzReg(reg, val ) => {
                if let Some(&v) = h_registers.get(&reg) {
                    if v !=0 as i32 {
                        addr+=val;
                    } else {
                        addr+=1;
                    }
                }
            },
            Instruction::JnzVal(direct_val, val ) => {
                if direct_val !=0 {
                    addr+=val;
                }
            },
        }
    }
    let v=*h_registers.get(&'a').unwrap();
    v as u32
}

#[aoc(day12, part2)]
fn solve_part2(input: &Vec<String>) -> u32 {
    let mut v_instructions:Vec<Instruction>=Vec::new();
    let mut h_registers:HashMap<char, i32>=[
        ('a', 0),
        ('b', 0),
        ('c', 1),
        ('d', 0) ]
        .iter()
        .cloned()
        .collect();

    let re_cpyreg=Regex::new(r"cpy ([a-d]) ([a-d])").unwrap();
    let re_cpyval=Regex::new(r"cpy (\d*) ([a-d])").unwrap();
    let re_jnzreg=Regex::new(r"jnz ([a-d]) ([-]?\d*)").unwrap();
    let re_jnzval=Regex::new(r"jnz (\d*) ([-]?\d*)").unwrap();
    let re_dec=Regex::new(r"dec ([a-d])").unwrap();
    let re_inc=Regex::new(r"inc ([a-d])").unwrap();

    for line in input{
        if let Some(caps) = re_cpyval.captures(line){
            v_instructions.push(Instruction::CpyVal(caps[1].parse::<i32>().unwrap(), caps[2].chars().next().unwrap()))
        } else if let Some(caps) = re_cpyreg.captures(line){
            v_instructions.push(Instruction::CpyReg(caps[1].chars().next().unwrap(), caps[2].chars().next().unwrap()))
        } else if let Some(caps) = re_jnzreg.captures(line){
        v_instructions.push(Instruction::JnzReg(caps[1].chars().next().unwrap(), caps[2].parse::<i32>().unwrap()))
        } else if let Some(caps) = re_jnzval.captures(line){
            v_instructions.push(Instruction::JnzVal(caps[1].parse::<u32>().unwrap(), caps[2].parse::<i32>().unwrap()))
        } else if let Some(caps) = re_inc.captures(line){
            v_instructions.push(Instruction::Inc(caps[1].chars().next().unwrap()))
        } else if let Some(caps) = re_dec.captures(line){
            v_instructions.push(Instruction::Dec(caps[1].chars().next().unwrap()))
        } else {
            panic!("Bad instruction {line:?}");
        }
    }

    let mut addr:i32=0;
    while  addr>=0 && addr<v_instructions.len() as i32{
        
//        println!("{:?}", v_instructions[addr as usize]);

        match v_instructions[addr as usize] {
            Instruction::CpyVal(val, reg) => {h_registers.insert(reg, val); addr+=1; },
            Instruction::CpyReg(reg_src, reg_dest) => {
            
                h_registers.insert(reg_dest, *h_registers.get(&reg_src).unwrap());
                addr+=1; 
            },
            Instruction::Dec(reg) => { 
                if let Some(v) = h_registers.get_mut(&reg) {
                    *v-=1;
                    addr+=1;
                }
            },
            Instruction::Inc(reg)=> { 
                if let Some(v) = h_registers.get_mut(&reg) {
                    *v+=1;
                    addr+=1;
                }
            },
            Instruction::JnzReg(reg, val ) => {
                if let Some(&v) = h_registers.get(&reg) {
                    if v !=0 as i32 {
                        addr+=val;
                    } else {
                        addr+=1;
                    }
                }
            },
            Instruction::JnzVal(direct_val, val ) => {
                if direct_val !=0 {
                    addr+=val;
                }
            },
        }
    }
    let v=*h_registers.get(&'a').unwrap();
    v as u32
}