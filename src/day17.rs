use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet, VecDeque};

#[aoc_generator(day17)]
fn input_generator(input: &str) -> String {
input.lines().next().unwrap().to_string()
}

#[aoc(day17, part1)]
fn solve_part1(input: &String) -> String {
    //let root=Coord{init:input.clone(), x:0, y:0, path:"".to_string()};
    println!("Using input :{}", input.clone());
//    bfs_day17_part1("hijkl".to_string())
    bfs_day17_part1(input.clone())
}

#[aoc(day17, part2)]
fn solve_part2(input: &String) -> u32 {
    0
}
#[derive(Clone, PartialEq, Debug)]
struct Coord {
    init:String,
    x:u32,
    y:u32,
    path:String,
}

impl Coord {
    fn adjacents(&self) -> Vec<Coord> {
        let md_doors=md5::compute(format!("{}{}",self.init,self.path));
        let mut v:Vec<Coord>=Vec::new();
        let digest=format!("{:x}", md_doors);
        let mut doors=digest.chars();


        if matches!(doors.next().unwrap(), 'b'..='f') && self.y>0 {
            v.push(Coord { init: self.init.clone(), x: self.x, y: self.y-1, path: format!("{}U", self.path) });
        }
        if matches!(doors.next().unwrap(), 'b'..='f') && self.y<3 {
            v.push(Coord { init: self.init.clone(), x: self.x, y: self.y+1, path: format!("{}D", self.path) });
        }
        if matches!(doors.next().unwrap(), 'b'..='f') && self.x>0 {
            v.push(Coord { init: self.init.clone(), x: self.x-1, y: self.y, path: format!("{}L", self.path) });
        }
        if matches!(doors.next().unwrap(), 'b'..='f') && self.x<3 {
            v.push(Coord { init: self.init.clone(), x: self.x+1, y: self.y, path: format!("{}R", self.path) });
        }
        println!("Exploring {};{} : {} doors with hash :\t{} and path :\t{}", self.x,self.y, v.len(),digest.clone(), self.path);
        v
    }
}

fn bfs_day17_part1 (init:String) -> String {
    let mut queue:VecDeque<Coord>=VecDeque::new();
    let mut explored:Vec<Coord>=Vec::new();

    let root=Coord{x:0, y:0, init, path:"".to_string()};
    queue.push_back(root.clone());
    explored.push(root);

    while !queue.is_empty() {
        let v=queue.pop_front().unwrap();
        println!("Exploring {};{}", v.x, v.y);

        if v.x==3 && v.y==3 {
            return v.path;
        }

        let adj = v.adjacents();
        for a in adj {
            if !explored.contains(&a){
                explored.push(a.clone());
                queue.push_back(a.clone());
            }
        }
    }
    "".to_string()
}
/*
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_valid_empty_floor() {
        assert!(Floor::default().is_valid())

    }
}
 */