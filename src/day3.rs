use core::fmt;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Triangle {
    a:u32,
    b:u32,
    c:u32,
}

impl Triangle {
    fn is_valid(&self) -> bool {
        self.a < self.b+self.c && self.b < self.a+self.c && self.c < self.a+self.b 
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Triangle a:{} b:{} c:{} : Valid:{}", self.a, self.b, self.c, self.is_valid())
    }
}
/*
#[aoc_generator(day3, part1)]
fn input_generator(input: &str) -> Vec<Triangle> {
    let mut v:Vec<Triangle>=Vec::new();

    for line in input.lines() {
        let parts:Vec<&str>=line.split_whitespace().collect();
        match parts.as_slice() {
            [a,b,c] => v.push(Triangle{
                    a:a.parse::<u32>().unwrap(),
                    b:b.parse::<u32>().unwrap(),
                    c:c.parse::<u32>().unwrap()}),
            _ => panic!("Bad import format {:?}", parts),
            
        };
    }
    v
}
 */
/* 
#[aoc(day3, part1)]
fn solve_part1(input: &Vec<Triangle>) -> u32 {
    input.iter().filter(|t| t.is_valid()).count() as u32
}
*/
fn get_triangle(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|l| l.parse::<u32>().unwrap())
        .collect()
}

#[aoc_generator(day3, part2)]
fn input_generator(input: &str) -> Vec<Triangle> {
    let mut v:Vec<Triangle>=Vec::new();

    let mut lines = input.lines();
    loop {
        match lines.next() {
            Some(line) => {
                let line1=get_triangle(line);
                let line2=get_triangle(lines.next().unwrap());
                let line3=get_triangle(lines.next().unwrap());
         
                for i in 0..3 {
                    v.push(Triangle{a:line1[i], b:line2[i], c:line3[i]});
                }
            }
            None => {break;}
        }
    }
    v
}

#[aoc(day3, part2)]
fn solve_part2(input: &Vec<Triangle>) -> u32 {
    input.iter().filter(|t| t.is_valid()).count() as u32
}
