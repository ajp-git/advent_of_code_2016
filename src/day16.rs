#[derive(Debug, Clone)]
struct BinaryString{
    text:String,
}

impl BinaryString {
    fn step(&mut self) -> usize {

        let a = self.text.clone();
        let mut b:String = a.chars().rev().collect();
        let c:String=b.chars().map(|c|
            match c {
                '1' => '0',
                '0' => '1',
                _ => panic!("Not binary digit"),
                
            }).collect();
        self.text=format!("{a}0{c}");
        self.text.len()
    }

    fn checksum(&self) -> String {
        let mut checksum:String=self.text.clone();
        loop {
            let mut tmp=checksum.clone();
            checksum.clear();

            let mut chars_it=tmp.chars();
            
            while let (Some(c1), Some(c2))=(chars_it.next(), chars_it.next()) {
                match (c1==c2) {
                    true => checksum.push('1'),
                    false => checksum.push('0'),
                }
            }

            if checksum.len()%2==1 {
                break;
            }  

        } 
        checksum
    }

    fn steps (&mut self, lenght:usize){
        while self.text.len()<lenght {
            self.step();
        }
        self.text=self.text[0..lenght].to_string();
    }
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> String {
    let line = input.lines().next().unwrap();
    println!("Input : {}", line);
    line.to_string()
}

#[aoc(day16, part1)]
fn solve_part1(input: &String) -> String {

//    let mut bs=BinaryString{text:"10000".to_string()};
    let mut bs=BinaryString{text:input.clone()};
    bs.steps(272);
//    println!("{bs:?}"); 

    bs.checksum()
}

#[aoc(day16, part2)]
fn solve_part2(input: &String) -> String {

//    let mut bs=BinaryString{text:"10000".to_string()};
    let mut bs=BinaryString{text:input.clone()};
    bs.steps(35651584);
//    println!("{bs:?}"); 

    bs.checksum()
}