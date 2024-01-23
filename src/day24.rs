use std::collections::VecDeque;
use itertools::Itertools;
#[derive(Debug,Clone)]
struct Coord{
    x:usize,
    y:usize,
    cost:u32,
}
impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[aoc_generator(day24)]
fn input_generator(input: &str) -> Vec<Vec<char>> {

    let mut v:Vec<Vec<char>>=Vec::new();
    let mut l=0;
    for line in input.lines(){
        let mut v_line= line.chars().collect();
        v.push(v_line);
    }
    v
}


#[aoc(day24, part1)]
fn solve_part1(input: &Vec<Vec<char>>) -> u32 {

    let parcours=('1'..='7').collect::<Vec<char>>();
    let coords:Vec<Coord>=('0'..='7').map(|c|find_value_coord(input, c)).collect();
    let start=find_value_coord(input, '0');

    println!("Start :{:?}", start);
    println!("Coords :{:?}", coords);

    let digits = "1234567"; // Digits to permute
    let mut permutations = digits.chars().permutations(digits.len())
        .map(|p| {
            let mut s = String::from("0"); // Start with '0'
            s.extend(p.into_iter()); // Extend the string with the permutation of '1' through '6'
            s
        });

    permutations.map(|p|{
        get_path_len(p, &coords, &input)
    }
        
    ).min().unwrap()
}

fn get_path_len (p: String, coords: &Vec<Coord>, grid:&Vec<Vec<char>>) -> u32{
    
    let mut last_pt=&coords[0];
    let mut total:u32=0;
    for c in p[1..=7].chars() {
//        println!("Looking for char {} so {}", c, c as usize-'0' as usize);
        let goal=&coords[c as usize -'0' as usize];
        total+=bfs_part1(&last_pt, goal, grid);
        last_pt=goal;
    }
    print!("\rExploring {p} found len {total}");

    total
}

fn find_value_coord(v: &Vec<Vec<char>>, val:char) -> Coord {

    for x in 0..v[0].len(){
        for y in 0..v.len(){
            if v[y][x]==val { return Coord{x, y,cost:0};}
        }
    }
    Coord { x: 0, y: 0, cost:0 }
}

fn bfs_part1 (root:&Coord, goal:&Coord, grid:&Vec<Vec<char>>) -> u32 {
    let mut queue:VecDeque<Coord>=VecDeque::new();
    let mut explored:Vec<Coord>=Vec::new();
    let mut parent:Vec<Coord>=Vec::new();
    queue.push_back(root.clone());
    explored.push(root.clone());

    while !queue.is_empty() {
        let v=queue.pop_front().unwrap();
        //println!("Exploring {};{}", v.x, v.y);
        let cost=v.cost;
        if v.x==goal.x && v.y==goal.y {
            return v.cost;
        }

        let adj = adjacents(v, grid, cost+1);
        for a in adj {
            if !explored.contains(&a) {
                explored.push(a.clone());
                queue.push_back(a.clone());
//                queue.retain(|q|q!=&a);
            }
        }
    }
    u32::MAX
}

fn adjacents(root: Coord, grid: &Vec<Vec<char>>, cost:u32) -> Vec<Coord> {
    let mut v = Vec::new();

    if root.x > 0 && grid[root.y][root.x - 1] != '#' {
        v.push(Coord { x: root.x - 1, y: root.y, cost: cost });
    }

    if root.y > 0 && grid[root.y - 1][root.x] != '#' {
        v.push(Coord { x: root.x, y: root.y - 1, cost: cost });
    }

    if root.x < grid[0].len() - 1 && grid[root.y][root.x + 1] != '#' {
        v.push(Coord { x: root.x + 1, y: root.y, cost: cost });
    }

    if root.y < grid.len() && grid[root.y + 1][root.x] != '#' {
        v.push(Coord { x: root.x, y: root.y + 1, cost: cost });
    }

    v
}
