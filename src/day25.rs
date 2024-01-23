use std::{collections::{HashMap, btree_map::Values}, any::Any};

use regex::Regex;

#[aoc_generator(day25)]
fn input_generator(input: &str) -> Vec<String> {

    let mut v:Vec<String>=Vec::new();
    for line in input.lines(){
        v.push(line.to_string());
    }
    v
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Register {
    name:char,
    val:i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Param {
    Reg(char),
    Val(i32),
}

#[derive(Debug,Clone, PartialEq, Eq, Copy)]
enum Instruction {
    Cpy(Param,Param),
    Jnz(Param,Param),
    Inc(Param),
    Dec(Param),
    Tgl(Param),
    Out(Param),
}

#[aoc(day25, part1)]
fn solve_part1(input: &Vec<String>) -> u32 {
    let mut v_instructions:Vec<Instruction>=Vec::new();
    let mut h_registers:HashMap<char, i32>=[
        ('a', 2),
        ('b', 0),
        ('c', 0),
        ('d', 0) ]
        .iter()
        .cloned()
        .collect();

    let re_cpyreg=Regex::new(r"cpy ([a-d]) ([a-d])").unwrap();
    let re_cpyval=Regex::new(r"cpy ([-]?\d+) ([a-d])").unwrap();
    let re_jnzreg=Regex::new(r"jnz ([a-d]) ([-]?\d+)").unwrap();
    let re_jnzval=Regex::new(r"jnz (\d+) ([-]?\d+)").unwrap();
    let re_jnzvalreg=Regex::new(r"jnz (\d+) ([a-d])").unwrap();
    let re_tglreg=Regex::new(r"tgl ([a-d])").unwrap();
    let re_tglval=Regex::new(r"tgl (\d+)").unwrap(); 
    let re_dec=Regex::new(r"dec ([a-d])").unwrap();
    let re_inc=Regex::new(r"inc ([a-d])").unwrap();
    let re_incval=Regex::new(r"inc (\d+)").unwrap();
    let re_outreg=Regex::new(r"out ([a-d])").unwrap();
    let re_outval=Regex::new(r"out (\d+)").unwrap();

    for line in input{
        if let Some(caps) = re_cpyval.captures(line){
            v_instructions.push(Instruction::Cpy(Param::Val(caps[1].parse::<i32>().unwrap()), Param::Reg(caps[2].chars().next().unwrap())))
        } else if let Some(caps) = re_cpyreg.captures(line){
            v_instructions.push(Instruction::Cpy(Param::Reg(caps[1].chars().next().unwrap()), Param::Reg(caps[2].chars().next().unwrap())))
        } else if let Some(caps) = re_jnzreg.captures(line){
            v_instructions.push(Instruction::Jnz(Param::Reg(caps[1].chars().next().unwrap()), Param::Val(caps[2].parse::<i32>().unwrap())))
        } else if let Some(caps) = re_jnzval.captures(line){
            v_instructions.push(Instruction::Jnz(Param::Val(caps[1].parse::<i32>().unwrap()), Param::Val(caps[2].parse::<i32>().unwrap())))
        } else if let Some(caps) = re_jnzvalreg.captures(line){
            v_instructions.push(Instruction::Jnz(Param::Val(caps[1].parse::<i32>().unwrap()), Param::Reg(caps[2].chars().next().unwrap())))
        } else if let Some(caps) = re_tglreg.captures(line){
            v_instructions.push(Instruction::Tgl(Param::Reg(caps[1].chars().next().unwrap())))
        } else if let Some(caps) = re_tglval.captures(line){
            v_instructions.push(Instruction::Tgl(Param::Val(caps[1].parse::<i32>().unwrap())))
        } else if let Some(caps) = re_inc.captures(line){
            v_instructions.push(Instruction::Inc(Param::Reg(caps[1].chars().next().unwrap())))
        } else if let Some(caps) = re_incval.captures(line){
            v_instructions.push(Instruction::Inc(Param::Val(caps[1].parse::<i32>().unwrap())))
        } else if let Some(caps) = re_dec.captures(line){
            v_instructions.push(Instruction::Dec(Param::Reg(caps[1].chars().next().unwrap())))
        } else if let Some(caps) = re_outval.captures(line){
            v_instructions.push(Instruction::Out(Param::Val(caps[1].parse::<i32>().unwrap())))
        } else if let Some(caps) = re_outreg.captures(line){
            v_instructions.push(Instruction::Out(Param::Reg(caps[1].chars().next().unwrap())))
        } else {
            panic!("Bad instruction {line:?}");
        }
    }

    let attended="0101010101010101".to_string();
    let mut index:u32=0;

    loop {
        h_registers.insert('a', index as i32);
        let mut s_out:String=String::new();
        let mut addr:i32=0;
        let mut out_count=0;

        while  addr>=0 && addr<v_instructions.len() as i32 && out_count<16 {
            //print!("\r{:?}\t\t{} {} {} {}", v_instructions[addr as usize], h_registers.get(&'a').unwrap(),h_registers.get(&'b').unwrap(),h_registers.get(&'c').unwrap(),h_registers.get(&'d').unwrap());

            match v_instructions[addr as usize] {
                Instruction::Cpy(Param::Val(a), Param::Reg(b)) => {
                    h_registers.insert( b,a );
                    addr+=1; 
                }, 

                Instruction::Cpy(Param::Reg(a), Param::Reg(b)) => {
                    h_registers.insert(b, *h_registers.get(&a).unwrap());
                    addr+=1; 
                }, 

                Instruction::Dec(Param::Reg(reg)) => { 
                    if let Some(v) = h_registers.get_mut(&reg) {
                        *v-=1;
                        addr+=1;
                    }
                },
                Instruction::Inc(Param::Reg(reg))=> { 
                    if let Some(v) = h_registers.get_mut(&reg) {
                        *v+=1;
                        addr+=1;
                    }
                },
                Instruction::Jnz(Param::Reg(reg), Param::Val(val) ) => {
                    if let Some(&v) = h_registers.get(&reg) {
                        if v !=0 {
                            addr+=val;
                        } else {
                            addr+=1;
                        }
                    }
                },
                Instruction::Jnz(Param::Val(val), Param::Reg(reg) ) => {
                    if let Some(&v) = h_registers.get(&reg) {
                        if val !=0 {
                            if addr+v >0 && addr+v<v_instructions.len() as i32   {
                                addr+=v;
                            }                
                        }else {
                            addr+=1;
                        }
                    }
                },

                Instruction::Jnz(Param::Val(val1), Param::Val(val2) ) => {
                    if val1 !=0 {
                        if addr+val2 >0 && addr+val2<v_instructions.len() as i32   {
                            addr+=val2;
                        }                
                    }else {
                        addr+=1;
                    }
                },

                Instruction::Tgl(Param::Reg(reg))  => {

                    if let Some(&val) = h_registers.get(&reg) {
                        if addr+val >=0 && addr+val<v_instructions.len() as i32   {
                            v_instructions[(addr+val) as usize]=tgl_instruction(v_instructions[(addr+val) as usize], h_registers.clone());
                        }            
                    }
                    addr+=1;
                
                },
                
                Instruction::Tgl(Param::Val(val)) => {
                    if addr+val >=0 && addr+val<v_instructions.len() as i32   {
                        v_instructions[(addr+val) as usize]=tgl_instruction(v_instructions[(addr+val) as usize], h_registers.clone());
                    }            
                },

                Instruction::Out(Param::Reg(reg)) => {
                    if let Some(&val) = h_registers.get(&reg) {
                        out_count+=1;
                        s_out.push_str(format!("{val}").as_str());
                    }
                    addr+=1;

                },

                Instruction::Out(Param::Val(val)) => {
                    s_out.push_str(format!("{val}").as_str());
                    addr+=1;
                    out_count+=1;

                },
                _ => panic!("Instruction :{:?} not covered", v_instructions[addr as usize]),
            }
        }
        if s_out==attended {
            return index;
        }
        print!("\rCurrent out: {} \tfor index : {}", s_out, index);
        index+=1;

    }
    let v=*h_registers.get(&'a').unwrap();
    v as u32
}

fn tgl_instruction(ins: Instruction, h_registers:HashMap<char, i32>) -> Instruction {

    match ins {
        Instruction::Dec(c) => Instruction::Inc(c),
        Instruction::Inc(c) => Instruction::Dec(c),
        Instruction::Cpy(a,b ) => {
            Instruction::Jnz(a, b)
        },
        Instruction::Jnz(a,b ) => Instruction::Cpy(a, b),
        Instruction::Tgl(a) => Instruction::Inc(a),
        Instruction::Out(a) => Instruction::Out(a),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_registers () -> HashMap<char, i32>{

        let mut h_registers:HashMap<char, i32>=[
            ('a', 0),
            ('b', 0),
            ('c', 0),
            ('d', 0) ]
            .iter()
            .cloned()
            .collect();
        h_registers
    }
    #[test]
    fn test_tgl_instruction_dec_to_inc() {
        let ins = Instruction::Dec(Param::Reg('a'));
        let h_registers = create_registers(); // Assuming your function doesn't modify the HashMap
        let result = tgl_instruction(ins, h_registers);
        assert_eq!(result, Instruction::Inc(Param::Reg('a')));
    }

    #[test]
    fn test_tgl_instruction_inc_to_dec() {
        let ins = Instruction::Inc(Param::Reg('a'));
        let h_registers = HashMap::new(); // Assuming your function doesn't modify the HashMap
        let result = tgl_instruction(ins, h_registers);
        assert_eq!(result, Instruction::Dec(Param::Reg('a')));
    }


    #[test]
    fn test_tgl_instruction_cpy_to_jnz() {
        let ins = Instruction::Cpy(Param::Reg('a'),Param::Reg('b'));
        let mut h_registers = create_registers();
        let mut t = h_registers.get_mut(&'a').unwrap();
        *t=4;
        let result = tgl_instruction(ins, h_registers);
        assert_eq!(result, Instruction::Jnz(Param::Reg('a'), Param::Reg('b')));
    }

    // Add more tests for other instruction types...
}

