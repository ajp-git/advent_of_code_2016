use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
#[derive(Debug, Clone)]
enum Command {
    RotateRight(usize),
    RotateLeft(usize),
    SwapLetter(char,char),
    SwapPosition(usize,usize),
    Move(usize,usize),
    ReversePositions(usize,usize),
    RotatePositionLetter(char), //rotate based on position of letter f
}

#[derive(Debug, Clone)]
struct Scrambler {
    password: Vec<char>,
    commands:Vec<Command>,
}

impl Scrambler {
    fn new (initial:&str) -> Self {
        Scrambler{
            password:initial.to_string().chars().collect(),
            commands:Vec::new()}
    }
    fn run (&mut self) {
        let commands=self.commands.clone();
        for c in commands {
            println!("From : {} with command {:?}", self.password.iter().collect::<String>(),c );
            self.execute_command(&c);
        }
    }

    fn execute_command(&mut self, c:&Command ){
        match c.clone() {
            Command::Move(a, b) =>{
                let c = *self.password.get(a).unwrap();
                self.password.remove(a);
                self.password.insert(b, c);

            },
            Command::ReversePositions(a, b) =>{
                let size=b-a;

                for i in 0..(size/2+1) {
                    let c=self.password[a+i];
                    self.password[a+i]=self.password[b-i];
                    self.password[b-i]=c;
                }
            },
            Command::RotateLeft(a) =>{
                for _ in 0..a {
                    let c=self.password[0];
                    self.password.remove(0);
                    self.password.push(c);
                }
            },
            Command::RotateRight(a) =>{
                let l = self.password.len();
                for _ in 0..a {
                    let c=self.password[l-1];
                    self.password.remove(l-1);
                    self.password.insert(0, c);
                }
            },
            Command::RotatePositionLetter(a) =>{
                let mut index=0;
                for i in 0..self.password.len() {
                    if a==self.password[i] {
                        index=i;
                        break;
                    }
                }

                let rot=1+index+if index>=4 { 1 } else {0};
                for _ in 0..rot {
                    let c =self.password[self.password.len()-1];
                    self.password.remove(self.password.len()-1);
                    self.password.insert(0, c);    
                }
            },
            Command::SwapLetter(a, b) => {
                let mut index_a=0;
                for i in 0..self.password.len() {
                    if a==self.password[i] {
                        index_a=i;
                        break;
                    }
                }
                let mut index_b=0;
                for i in 0..self.password.len() {
                    if b==self.password[i] {
                        index_b=i;
                        break;
                    }
                }
                let c=self.password[index_a];
                self.password[index_a]=self.password[index_b];
                self.password[index_b]=c;

            },
            Command::SwapPosition(a, b) => {
                let c = self.password[a];
                self.password[a]=self.password[b];
                self.password[b]=c;
            },
        }

    }
    fn reverse(&mut self){
        for c in self.commands.clone().into_iter().rev(){
            match c {
                Command::Move(a, b) => {self.execute_command(&Command::Move(b, a))},
                Command::ReversePositions(_, _) => self.execute_command(&c),
                Command::RotateLeft(a) => self.execute_command(&Command::RotateRight(a)),
                Command::RotateRight(a) => self.execute_command(&Command::RotateLeft(a)),
                Command::SwapLetter(_, _) => self.execute_command(&c),
                Command::SwapPosition(_,_) => self.execute_command(&c),
                Command::RotatePositionLetter(a) => {
                    let len = self.password.len();
                    for original_index in 0..len {
                        let mut rot = original_index + 1; 
                        if original_index >= 4 {
                            rot += 1; 
                        }
                        rot %= len; 
                
                        let new_index = (original_index + rot) % len;
                
                        if self.password[new_index] == a {
                            self.execute_command(&Command::RotateLeft(rot));
                            break;
                        }
                    }
                }
            }
        }
    }
}

#[aoc_generator(day21)]
fn input_generator(input: &str) -> Scrambler {
    let mut s = Scrambler::new("abcdefgh");
/*    let mut s = Scrambler::new("abcde");
    s.commands.push(Command::SwapPosition(4, 0));
    s.commands.push(Command::SwapLetter('d', 'b'));
    s.commands.push(Command::ReversePositions(0, 4));
    s.commands.push(Command::RotateLeft(1));
    s.commands.push(Command::Move(1, 4));
    s.commands.push(Command::Move(3, 0));
    s.commands.push(Command::RotatePositionLetter('b'));
    s.commands.push(Command::RotatePositionLetter('d'));

    return s;
 */
    let re_rr=Regex::new(r"rotate right (\d*)").unwrap();
    let re_rl=Regex::new(r"rotate left (\d*)").unwrap();
    let re_sl=Regex::new(r"swap letter ([a-z]) with letter ([a-z])").unwrap();
    let re_sp=Regex::new(r"swap position (\d*) with position (\d*)").unwrap();
    let re_revp=Regex::new(r"reverse positions (\d*) through (\d*)").unwrap();
    let re_rotp=Regex::new(r"rotate based on position of letter ([a-z])").unwrap();
    let re_move=Regex::new(r"move position (\d*) .* (\d*)").unwrap();

    for line in input.lines(){
        if let Some(caps) =re_rr.captures(line)  {
            s.commands.push(Command::RotateRight(caps[1].parse::<usize>().unwrap()));            
        } else if let Some(caps)=re_rl.captures(line) {
            s.commands.push(Command::RotateLeft(caps[1].parse::<usize>().unwrap()));            
        } else if let Some(caps)=re_sl.captures(line) {
            s.commands.push(Command::SwapLetter(caps[1].to_string().chars().next().unwrap(),caps[2].to_string().chars().next().unwrap()));            
        } else if let Some(caps)=re_sp.captures(line) {
            s.commands.push(Command::SwapPosition(caps[1].parse::<usize>().unwrap(),caps[2].parse::<usize>().unwrap()));            
        } else if let Some(caps)=re_revp.captures(line) {
            s.commands.push(Command::ReversePositions(caps[1].parse::<usize>().unwrap(),caps[2].parse::<usize>().unwrap()));            
        } else if let Some(caps)=re_rotp.captures(line) {
            s.commands.push(Command::RotatePositionLetter(caps[1].to_string().chars().next().unwrap()));            
        } else if let Some(caps)=re_move.captures(line) {
            s.commands.push(Command::Move(caps[1].parse::<usize>().unwrap(),caps[2].parse::<usize>().unwrap()));            
        } else  {
            panic!("Unknown command {}", line);            
        }
    }
    s
}

#[aoc(day21, part1)]
fn solve_part1(s: &Scrambler) -> String {
    let mut s = s.clone();
    s.run();
    s.password.iter().collect::<String>()
}
#[aoc(day21, part2)]
fn solve_part2(s: &Scrambler) -> String {
    let mut s = s.clone();
    s.password="fbgdceah".chars().collect::<Vec<char>>();
    s.reverse();
    s.password.iter().collect::<String>()
}
