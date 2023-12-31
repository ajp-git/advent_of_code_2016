use std::collections::{HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<String> {

    let mut v:Vec<String>=Vec::new();
    for line in input.lines(){
        v.push(line.to_string());
    }
    v
}

#[aoc(day7, part1)]
fn solve_part1(input: &Vec<String>) -> u32 {
    //let input = vec!["abba[mnop]qrst","abcd[bddb]xyyx","aaaa[qwer]tyui","ioxxoj[asdfgh]zxcvbn"];
    let mut count=0;
    for line in input.iter(){
        if is_tls(line.to_string()){
            count+=1;
        }
    }
    count
}

fn is_tls(ip: String) -> bool {

    let mut in_sb=false;
    let mut bfound=false;

    let mut abba:[char;4]=[' ';4];
    for i in 0..ip.len()-3{
        let c = ip.chars().nth(i).unwrap();
        match c {
            '[' => in_sb=true,
            ']' => {in_sb=false},
            _ => {},
        }
        
        for j in 0..4 {
            abba[j]=ip.chars().nth(i+j).unwrap();
        }
        
        if abba[0]==abba[3]&&abba[1]==abba[2] && abba[1]!=abba[0]{
            if in_sb {
                //println!("\tFound in sb : {:?}", abba);
                return false;
            }
//            println!("\tFound outside sb : {:?}", abba);
            bfound=true;
        }
    }
    bfound
}


#[aoc(day7, part2)]
fn solve_part2(input: &Vec<String>) -> u32 {
//    let input = vec!["aba[bab]xyz","xyx[xyx]xyx","aaa[kek]eke","zazbz[bzb]cdb"];
    let mut count=0;
    for line in input.iter(){
        if is_ssl(line.to_string()){
            count+=1;
        }
    }
    count
}

fn is_ssl(line: String) -> bool {

    let mut aba:HashSet<(char,char,char)>=HashSet::new();
    let mut bab:HashSet<(char,char,char)>=HashSet::new();
    let mut inside=false;

    for i in 0..line.len()-2{
        let c = line.chars().nth(i).unwrap();

        match c {
            '[' => inside=true,
            ']' => inside=false,
            _ =>{
                let d= line.chars().nth(i+1).unwrap();
                if d=='[' || d==']' { continue; }
                let e= line.chars().nth(i+2).unwrap();
                if e=='[' || e==']' { continue; }
                if c==e && c!=d {
                    match inside {
                        false => aba.insert((c,d,e)),
                        true => bab.insert((c,d,e)),
                    };
                }
            }
        }
    }
    for (a,b,_) in aba.iter(){
        if bab.contains(&(*b,*a,*b)) {
            return true;
        }
    }
    false
}