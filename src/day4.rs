use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|f| f.to_string()).collect()
}
#[derive(Debug,Clone)]
struct Name {
    name: String,
    number:u32,
    letters:String,
}

impl Name {

    fn get_checksum(&self) -> String {

        let mut h_freq:HashMap<char, u32>=HashMap::new();

        for c in self.name.chars(){
            match c {
                'a'..='z' => {
                    if let Some(f) = h_freq.get_mut(&c) {
                        *f+=1;
                    } else {
                        h_freq.insert(c, 1);
                    }
                }
                _ =>{},
                
            }
        }
        let mut v_freq:Vec<(char, u32)>=h_freq.into_iter().collect();

        v_freq.sort_by(|&(c1,n1), &(c2,n2)| n2.cmp(&n1).then_with(||c1.cmp(&c2)));
        let checksum:String=v_freq.into_iter().take(5).map(|(c,_)|c).collect();
        //println!("Checksum of {} is {} and letters are : {}", self.name, checksum, self.letters);
        checksum
    }

    fn is_real(&self) -> bool {
        let checksum = self.get_checksum();
        //println!("Letters: {:?}", self.letters);
        //println!("Checksum: {:?}", checksum);
        self.letters == checksum
    }
    fn decrypt(self) -> String {
        let mut decrypted_name=String::new();

        for c in self.name.chars() {
            match c {
                'a'..='z' => {
                    let v=((c as u8 - 'a' as u8 + (self.number%26) as u8)%26 +'a' as u8) as char;
                    decrypted_name.push(v);
                },
                '-' => decrypted_name.push(' '), 
                _ => panic!("Impossible char"),
            }
        }
        decrypted_name
    }
    
}

#[aoc(day4, part1)]
fn solve_part1(input: &Vec<String>) -> u32 {
 let mut v_names:Vec<Name>=Vec::new();

 let re=Regex::new(r"([a-z-]+)-(\d+)\[([a-z]{5})\]").unwrap();

 for line in input {

    //println!("{line}");
    let caps=re.captures(&line).unwrap();

    let na=caps.get(1).unwrap().as_str();
    let nu = caps.get(2).unwrap().as_str();
    let le = caps.get(3).unwrap().as_str();

    let na=na.to_string();
    let nu=nu.parse::<u32>().unwrap();
    let le=le.to_string();

    v_names.push(Name { name: na, number: nu, letters: le });
 }

 let mut total:u32=0;
 for name in v_names {
//    println!("{:?} : {}", name, name.is_real())
    if name.is_real(){
        total+=name.number;
    }
 }
 total
}

#[aoc(day4, part2)]
fn solve_part2(input: &Vec<String>) -> u32 {
 let mut v_names:Vec<Name>=Vec::new();

 let re=Regex::new(r"([a-z-]+)-(\d+)\[([a-z]{5})\]").unwrap();

 for line in input {

    //println!("{line}");
    let caps=re.captures(&line).unwrap();

    let na=caps.get(1).unwrap().as_str();
    let nu = caps.get(2).unwrap().as_str();
    let le = caps.get(3).unwrap().as_str();

    let na=na.to_string();
    let nu=nu.parse::<u32>().unwrap();
    let le=le.to_string();

    v_names.push(Name { name: na, number: nu, letters: le });
 }

 for name in v_names {
    if name.clone().decrypt()=="northpole object storage" {
        return name.number
    }
    //println!("{} : {}", name.number.clone(), name.decrypt())
 }
0
}
