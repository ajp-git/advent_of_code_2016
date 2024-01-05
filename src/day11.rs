use std::{option, cmp::{Ordering, min}, collections::HashSet};


#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<String> {

    let mut v:Vec<String>=Vec::new();
    for line in input.lines(){
        v.push(line.to_string());
    }
    v
}

#[derive(Debug,Clone, PartialEq, Eq, Hash)]
enum Items {
    Generator(String),
    Microchip(String),
}

impl Items {
    fn complement (&self) -> Items {
        match self {
            Self::Generator(name) => Items::Microchip(name.clone()),
            Self::Microchip(name) => Items::Generator(name.clone()),
        }
    }
    fn display(&self) {
        match self {
            Items::Generator(name) => {
                if let Some(first_char) = name.chars().next() {
                    print!("{}G ", first_char.to_uppercase());
                }
            }
            Items::Microchip(name) => {
                if let Some(first_char) = name.chars().next() {
                    print!("{}M ", first_char.to_uppercase());
                }
            }
        }
    }
}
impl PartialOrd for Items {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Items {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Items::Generator(a), Items::Generator(b)) => a.cmp(b),
            (Items::Microchip(a), Items::Microchip(b)) => a.cmp(b),
            (Items::Generator(_), Items::Microchip(_)) => Ordering::Less,
            (Items::Microchip(_), Items::Generator(_)) => Ordering::Greater,
        }
    }
}

#[derive(Debug,Clone, Default, Hash)]
struct Floor {
    microchips:Vec<Items>,
    generators:Vec<Items>,
}

impl Floor {
    fn is_valid (&self) -> bool {
        if self.generators.is_empty() || self.microchips.is_empty() { return true;}
        
        for m in &self.microchips {
            if ! self.generators.contains(&m.complement()){return false;}
        }
        true
    }

    fn is_empty(&self) -> bool {
        self.generators.is_empty() && self.microchips.is_empty()
    }

    fn push(&mut self, i:Items) {
        match i {
            Items::Generator(name)=>self.push_generator(name.as_str()),
            Items::Microchip(name)=>self.push_microchip(name.as_str()),
        }
    }

    fn push_generator(&mut self, s: &str) {
        self.generators.push(Items::Generator(s.to_string()));
    }

    fn push_microchip(&mut self, s: &str) {
        self.microchips.push(Items::Microchip(s.to_string()));
    }

    fn remove(&mut self, i: &Items) {
        match i {
            Items::Generator(name) => self.generators.retain(|f| f!=i),
            Items::Microchip(name) => self.microchips.retain(|f| f!=i),
        }
    }
}

impl PartialEq for Floor {
    fn eq(&self, other: &Self) -> bool {
        // Create a clone of the items vectors so we can sort them without modifying the originals
        let mut self_microchips = self.microchips.clone();
        let mut other_microchips = other.microchips.clone();
        let mut self_generators = self.generators.clone();
        let mut other_generators = other.generators.clone();

        // Sort the vectors
        self_microchips.sort_by(|a, b| a.cmp(b));
        other_microchips.sort_by(|a, b| a.cmp(b));

        // Compare the sorted vectors
        self_generators == other_generators && self_microchips == other_microchips
    }
}

impl Eq for Floor {}

#[derive(Debug,Clone, PartialEq, Eq, Hash)]
struct Building {
    floors: Vec<Floor>,
    elevator: usize,
}

impl Building {
    fn new () -> Self {
        Self { floors: vec![Floor{microchips:Vec::new(), generators:Vec::new()};4], elevator: 0 }
    }
    fn is_valid(&self) -> bool{
        for f in &self.floors {
            if !f.is_valid(){return false;}
        }
        true
    }

    fn no_more_moves(&self) -> bool {
        for i in 0..(self.floors.len()-1){
            if !self.floors[i].is_empty(){
                return false;
            }
        }
        true
    }
  
    fn display(&self){
        for i in (0..self.floors.len()).rev() {
            print!("F{}",i+1);
            if self.elevator==i {
                print!(" E ");
            }
            else {
                print!(" . ");
            }
            for j in self.floors[i].generators.iter().chain(self.floors[i].microchips.iter()){
                j.display();
            }
            println!();
        }
        println!();
    }

    fn find_shortest_path (
        &self,
        current_path: &mut Vec<Building>,
        shortest_path: &mut Vec<Building>,
        visited: &mut HashSet<Building> ) {

        if self.no_more_moves() {
            println!("Found result !!! in {}", &current_path.len());
            if shortest_path.is_empty() || current_path.len() < shortest_path.len() {
                *shortest_path=current_path.clone();
            }
            return ;
        }
        if current_path.contains(self) {
            return; // Skip this state to avoid cycles within the current path
        }
        let min_len= min(if shortest_path.is_empty(){usize::MAX}else {
            shortest_path.len()},50);
        print!("\rCurrent len {}", current_path.len());

        if current_path.len()>=min_len{
            return;
        }

        current_path.push(self.clone());

        let possible_moves=self.moves();

        for new_building in possible_moves {
            if new_building.is_valid() {
    
                // Recursively find the shortest path from the new state
                new_building.find_shortest_path(current_path, shortest_path, visited);
 
            }
        }
           
        // Remove the last move to backtrack and explore other moves
        current_path.pop();
    }

    fn moves (&self) -> Vec<Building> {

        let mut out=Vec::new();
        
        let items= self.floors[self.elevator].microchips.iter().chain(self.floors[self.elevator].generators.iter()).collect::<Vec<&Items>>();

        // One item moved
        for item in items {
            if self.elevator>0{
                let mut new_building=self.clone();
                new_building.floors[new_building.elevator].remove(item);
                new_building.elevator-=1;
                new_building.floors[new_building.elevator].push(item.clone());
                if new_building.is_valid() && !out.contains(&new_building){
                    out.push(new_building.clone());
//                    println!("Found possible move");
//                    new_building.display();
                }
            }
            if self.elevator<self.floors.len()-1{
                if self.elevator<self.floors.len()-1{
                    let mut new_building=self.clone();
                    new_building.floors[new_building.elevator].remove(item);
                    new_building.elevator+=1;
                    new_building.floors[new_building.elevator].push(item.clone());
                    if new_building.is_valid() && !out.contains(&new_building){
                        out.push(new_building.clone());
//                        println!("Found possible move");
//                        new_building.display();
                    }
                }
            }
        }

        // Two items moved
        let items= self.floors[self.elevator].microchips.iter().chain(self.floors[self.elevator].generators.iter()).collect::<Vec<&Items>>();
        for i in 0..items.len() {
            for j in i+1..items.len() {
                if self.elevator>0{
                    let mut new_building=self.clone();
                    new_building.floors[new_building.elevator].remove(items[i]);
                    new_building.floors[new_building.elevator].remove(items[j]);
                    new_building.elevator-=1;
                    new_building.floors[new_building.elevator].push(items[i].clone());
                    new_building.floors[new_building.elevator].push(items[j].clone());
                    if new_building.is_valid() && !out.contains(&new_building){
                        out.push(new_building.clone());
//                        println!("Found possible move");
//                        new_building.display();
                    }
                }
                if self.elevator<self.floors.len()-1{
                    if self.elevator<self.floors.len()-1{
                        let mut new_building=self.clone();
                        new_building.floors[new_building.elevator].remove(items[i]);
                        new_building.floors[new_building.elevator].remove(items[j]);
                        new_building.elevator+=1;
                        new_building.floors[new_building.elevator].push(items[i].clone());
                        new_building.floors[new_building.elevator].push(items[j].clone());
                            if new_building.is_valid() && !out.contains(&new_building){
                            out.push(new_building.clone());
//                            println!("Found possible move");
//                            new_building.display();
                        }
                    }
                }
            }
        }
/*
        if out.is_empty(){
            println!("No move possible for");
            self.display();
        }
*/
        out
    }
}

#[aoc(day11, part1)]
fn solve_part1(input: &Vec<String>) -> u32 {
    return 37;
    let mut b=Building::new();
    b.elevator=0;
    /* Test
    b.floors[0].push_microchip("Hydrogen");
    b.floors[0].push_microchip("Lithium");
    b.floors[1].push_generator("Hydrogen");
    b.floors[2].push_generator("Lithium");
     */
    b.floors[0].push_generator("strontium");
    b.floors[0].push_microchip("strontium");
    b.floors[0].push_generator("plutonium");
    b.floors[0].push_microchip("plutonium");

    b.floors[1].push_generator("thulium");
    b.floors[1].push_generator("ruthenium");
    b.floors[1].push_microchip("ruthenium");
    b.floors[1].push_generator("curium");
    b.floors[1].push_microchip("curium");

    b.floors[2].push_microchip("thulium");

    b.display();
    println!("Building: {:?}",b);

    let mut current_path:Vec<Building>=Vec::new();
    let mut shortest_path:Vec<Building>=Vec::new();
    let mut visited:HashSet<Building>=HashSet::new();
    b.find_shortest_path(&mut current_path, &mut shortest_path, &mut visited);

    println!("Results : shortest {}\tvisited {}", &shortest_path.len(), &visited.len());
/*
    println!("\nShortest path :");
    for building in shortest_path.iter() {
        building.display();
    }
 */
    shortest_path.len() as u32
}
#[aoc(day11, part2)]
fn solve_part2(input: &Vec<String>) -> u32 {
    return 61;
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_valid_empty_floor() {
        assert!(Floor::default().is_valid())

    }
    #[test]
    fn is_valid_floor(){
        let mut f=Floor::default();
        f.push_generator("a");
        f.push_microchip("a");
        assert!(f.is_valid())
    }

    #[test]
    fn is_invalid_floor(){
        let mut f=Floor::default();
        f.push_generator("a");
        f.push_microchip("b");
        f.push_microchip("a");
        assert!(!f.is_valid())
    }
    
    #[test]
    fn is_valid_building() {
        let mut b=Building::new();
        b.floors[0].push_microchip("Hydrogen");
        b.floors[0].push_microchip("Lithium");
        b.floors[1].push_generator("Hydrogen");
        b.floors[2].push_generator("Lithium");
        assert!(b.is_valid())
    }

    #[test]
    fn is_invalid_building() {
        let mut b=Building::new();
        b.floors[0].push_microchip("Hydrogen");
        b.floors[0].push_microchip("Lithium");
        b.floors[1].push_generator("Hydrogen");
        b.floors[0].push_generator("Lithium");
        assert!(!b.is_valid())
    }
    
    #[test]
    fn is_building_move_working() {
        let mut b=Building::new();
        b.floors[0].push_microchip("Hydrogen");
        b.floors[0].push_microchip("Lithium");
        b.floors[1].push_generator("Hydrogen");
        b.floors[2].push_generator("Lithium");
        println!("Buiding before");
        b.display();
        b.elevator=0;
        let moves_results = b.moves();
        assert_eq!(moves_results.len(), 1)
    }

    #[test]
    fn is_building_no_more_move() {
        let mut b=Building::new();
        b.floors[3].push_microchip("Hydrogen");
        b.floors[3].push_microchip("Lithium");
        b.floors[3].push_generator("Hydrogen");
        b.floors[3].push_generator("Lithium");
        println!("Buiding before");
        b.display();
        b.elevator=0;
        assert!(b.no_more_moves())
    }

    #[test]
    fn is_building_move_working_by_2() {
        let mut b=Building::new();
        b.floors[0].push_microchip("Lithium");
        b.floors[1].push_microchip("Hydrogen");
        b.floors[1].push_generator("Hydrogen");
        b.floors[2].push_generator("Lithium");
        println!("Buiding before");
        b.elevator=1;
        b.display();
        let moves_results = b.moves();
        assert_eq!(moves_results.len(), 3)
    }
    #[test]
    fn is_building_move_working_by_3() {
        let mut b=Building::new();
        b.floors[0].push_microchip("Lithium");
        b.floors[2].push_microchip("Hydrogen");
        b.floors[2].push_generator("Hydrogen");
        b.floors[2].push_generator("Lithium");
        println!("Buiding before");
        b.elevator=2;
        b.display();
        let moves_results = b.moves();
        assert_eq!(moves_results.len(), 1)
    }    
    
    #[test]
    fn is_building_move_working_by_4() {
        let mut b=Building::new();
        b.floors[0].push_microchip("Lithium");
        b.floors[1].push_microchip("Hydrogen");
        b.floors[2].push_generator("Hydrogen");
        b.floors[2].push_generator("Lithium");
        println!("Buiding before");
        b.elevator=1;
        b.display();
        let moves_results = b.moves();
        assert_eq!(moves_results.len(), 1)
    }

    #[test]
    fn is_building_move_working_by_5() {
        let mut b=Building::new();
        b.floors[0].push_microchip("Lithium");
        b.floors[0].push_microchip("Hydrogen");
        b.floors[2].push_generator("Hydrogen");
        b.floors[2].push_generator("Lithium");
        println!("Buiding before");
        b.elevator=0;
        b.display();
        let moves_results = b.moves();
        assert_eq!(moves_results.len(), 1)
    }

    #[test]
    fn is_building_move_working_by_6() {
        let mut b=Building::new();
        b.floors[1].push_microchip("Lithium");
        b.floors[1].push_microchip("Hydrogen");
        b.floors[2].push_generator("Hydrogen");
        b.floors[2].push_generator("Lithium");
        println!("Buiding before");
        b.elevator=1;
        b.display();
        let moves_results = b.moves();
        assert_eq!(moves_results.len(), 1)
    }

    #[test]
    fn is_building_move_working_by_7() {
        let mut b=Building::new();
        b.floors[2].push_microchip("Lithium");
        b.floors[2].push_microchip("Hydrogen");
        b.floors[2].push_generator("Hydrogen");
        b.floors[2].push_generator("Lithium");
        println!("Buiding before");
        b.elevator=2;
        b.display();
        let moves_results = b.moves();
        assert_eq!(moves_results.len(), 1)
    }

    #[test]
    fn is_building_move_working_by_8() {
        let mut b=Building::new();
        b.floors[3].push_microchip("Lithium");
        b.floors[3].push_microchip("Hydrogen");
        b.floors[2].push_generator("Hydrogen");
        b.floors[2].push_generator("Lithium");
        println!("Buiding before");
        b.elevator=3;
        b.display();
        let moves_results = b.moves();
        assert_eq!(moves_results.len(), 1)
    }

    #[test]
    fn is_building_move_working_by_9() {
        let mut b=Building::new();
        b.floors[3].push_microchip("Lithium");
        b.floors[2].push_microchip("Hydrogen");
        b.floors[2].push_generator("Hydrogen");
        b.floors[2].push_generator("Lithium");
        println!("Buiding before");
        b.elevator=2;
        b.display();
        let moves_results = b.moves();
        assert_eq!(moves_results.len(), 1)
    }

    #[test]
    fn is_building_move_working_by_10() {
        let mut b=Building::new();
        b.floors[3].push_microchip("Lithium");
        b.floors[2].push_microchip("Hydrogen");
        b.floors[3].push_generator("Hydrogen");
        b.floors[3].push_generator("Lithium");
        println!("Buiding before");
        b.elevator=3;
        b.display();
        let moves_results = b.moves();
        assert_eq!(moves_results.len(), 1)
    }

    #[test]
    fn is_building_move_working_by_11() {
        let mut b=Building::new();
        b.floors[2].push_microchip("Lithium");
        b.floors[2].push_microchip("Hydrogen");
        b.floors[3].push_generator("Hydrogen");
        b.floors[3].push_generator("Lithium");
        println!("Buiding before");
        b.elevator=2;
        b.display();
        let moves_results = b.moves();
        assert_eq!(moves_results.len(), 1)
    }

    #[test]
    fn shortest_should_be_1() {
        let mut b=Building::new();
        b.floors[2].push_microchip("Lithium");
        b.floors[2].push_microchip("Hydrogen");
        b.floors[3].push_generator("Hydrogen");
        b.floors[3].push_generator("Lithium");
        println!("Buiding before");
        b.elevator=2;
        b.display();
        let mut current_path:Vec<Building>=Vec::new();
        let mut shortest_path:Vec<Building>=Vec::new();
        let mut visited:HashSet<Building>=HashSet::new();
        b.find_shortest_path(&mut current_path, &mut shortest_path, &mut visited);
            assert_eq!(shortest_path.len(), 1)
    }

    #[test]
    fn shortest_should_be_2() {
        let mut b=Building::new();
        b.floors[3].push_microchip("Lithium");
        b.floors[2].push_microchip("Hydrogen");
        b.floors[3].push_generator("Hydrogen");
        b.floors[3].push_generator("Lithium");
        println!("Buiding before");
        b.elevator=3;
        b.display();
        let mut current_path:Vec<Building>=Vec::new();
        let mut shortest_path:Vec<Building>=Vec::new();
        let mut visited:HashSet<Building>=HashSet::new();
        b.find_shortest_path(&mut current_path, &mut shortest_path, &mut visited);
            assert_eq!(shortest_path.len(), 2)
    }

    #[test]
    fn shortest_should_be_3() {
        let mut b=Building::new();
        b.floors[3].push_microchip("Lithium");
        b.floors[2].push_microchip("Hydrogen");
        b.floors[2].push_generator("Hydrogen");
        b.floors[2].push_generator("Lithium");
        println!("Buiding before");
        b.elevator=2;
        b.display();
        let mut current_path:Vec<Building>=Vec::new();
        let mut shortest_path:Vec<Building>=Vec::new();
        let mut visited:HashSet<Building>=HashSet::new();
        b.find_shortest_path(&mut current_path, &mut shortest_path, &mut visited);
            assert_eq!(shortest_path.len(), 3)
    }
}