use std::collections::VecDeque;

#[aoc_generator(day13)]
fn input_generator(input: &str) -> u32{
 1350
}

#[aoc(day13, part1)]
fn solve_part1(input: &u32) -> u32 {
    print!("   ");
    for x in 0..10{
        print!("{x:02} ");
    }
    for y in 0..10{
        println!();
        for x in 0..10 {
            if x == 0 {
                print!("{y:02} ");
            }
            let coo =Coord{x:x, y:y, cost:0};
            let bval=if coo.is_valid() { " . "} else { " # " };
            print!("{}", bval)
        }
    }
    println!();
    bfs_part1(Coord { x: 31, y: 39, cost: 0 })
    //bfs(Coord { x: 7, y: 4, cost: 0 })
}

#[aoc(day13, part2)]
fn solve_part2(input: &u32) -> u32 {
    print!("   ");
    for x in 0..10{
        print!("{x:02} ");
    }
    for y in 0..10{
        println!();
        for x in 0..10 {
            if x == 0 {
                print!("{y:02} ");
            }
            let coo =Coord{x:x, y:y, cost:0};
            let bval=if coo.is_valid() { " . "} else { " # " };
            print!("{}", bval)
        }
    }
    println!();
    bfs_part2(Coord { x: 200, y: 200, cost: 0 })
    //bfs(Coord { x: 7, y: 4, cost: 0 })
}
#[derive(Debug,Clone)]
struct Coord {
    x:u32,
    y:u32,
    cost:u32,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x== other.x && self.y==other.y
    }
}
impl Eq for Coord {} // If you want to enforce that Coord can be equated (and you've ensured it's a valid operation)

impl Coord {
    fn is_valid(&self) -> bool {
        /*
        Find x*x + 3*x + 2*x*y + y + y*y.
        Add the office designer's favorite number (your puzzle input).
        Find the binary representation of that sum; count the number of bits that are 1.
        If the number of bits that are 1 is even, it's an open space.
        If the number of bits that are 1 is odd, it's a wall.
         */
        let find=self.x*self.x+3*self.x+2*self.x*self.y+self.y+self.y*self.y+1350;
//        let find=self.x*self.x+3*self.x+2*self.x*self.y+self.y+self.y*self.y+10;
        let bfind=format!("{:b}",find);
        let wall=bfind.chars().filter(|c|c==&'1').count();
        let bwall=wall%2 == 0;

        bwall
    }

    fn adjacents (&self) -> Vec<Coord> {

        let mut out:Vec<Coord>=Vec::new();

        if self.x>50||self.y>50 {
            return out;
        }
        if self.x >0 {
            out.push(Coord { x: self.x-1, y: self.y, cost:self.cost+1});
        }
   
        if self.y >0 {
            out.push(Coord { x: self.x, y: self.y-1, cost:self.cost+1});
            
        }
        out.push(Coord { x: self.x+1, y: self.y, cost:self.cost+1});
        out.push(Coord { x: self.x, y: self.y+1, cost:self.cost+1});

        println!("Adjacents of {self:?} : {out:?}");
        out
    }
}

/*
 procedure BFS(G, root) is
 2      let Q be a queue
 3      label root as explored
 4      Q.enqueue(root)
 5      while Q is not empty do
 6          v := Q.dequeue()
 7          if v is the goal then
 8              return v
 9          for all edges from v to w in G.adjacentEdges(v) do
10              if w is not labeled as explored then
11                  label w as explored
12                  w.parent := v
13                  Q.enqueue(w)
*/

// bfs returns max_cost to goal

fn bfs_part1 (goal:Coord) -> u32 {
    let mut queue:VecDeque<Coord>=VecDeque::new();
    let mut explored:Vec<Coord>=Vec::new();
    let mut parent:Vec<Coord>=Vec::new();

    let root=Coord{x:1, y:1, cost:0};
    queue.push_back(root.clone());
    explored.push(root);

    while !queue.is_empty() {
        let v=queue.pop_front().unwrap();
        println!("Exploring {};{}", v.x, v.y);

        if v==goal {
            return v.cost;
        }

        let adj = v.adjacents();
        for a in adj {
            if !explored.contains(&a) && a.is_valid(){
                explored.push(a.clone());
                queue.push_back(a.clone());
//                queue.retain(|q|q!=&a);
            }
        }
    }
    u32::MAX
}


fn bfs_part2 (goal:Coord) -> u32 {
    let mut queue:VecDeque<Coord>=VecDeque::new();
    let mut explored:Vec<Coord>=Vec::new();
    let mut parent:Vec<Coord>=Vec::new();

    let root=Coord{x:1, y:1, cost:0};
    queue.push_back(root.clone());
    explored.push(root);

    let mut total_locations=1;

    while !queue.is_empty() {
        let v=queue.pop_front().unwrap();
        println!("Exploring {};{}", v.x, v.y);

        if v==goal {
            return v.cost;
        }

        let adj = v.adjacents();
        for a in adj {
            if !explored.contains(&a) && a.is_valid(){
                total_locations+=1;
                explored.push(a.clone());
                queue.push_back(a.clone());
//                queue.retain(|q|q!=&a);
            }
            if v.cost==50 {
                return total_locations;
            }
        }
    }
    u32::MAX
}

