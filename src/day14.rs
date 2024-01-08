use std::collections::VecDeque;

#[aoc_generator(day14)]
fn input_generator(input: &str) -> String {
    "yjdafjpo".to_string()
//    "abc".to_string()
}

#[aoc(day14, part1)]
fn solve_part1(input: &String) -> u32 {
    let mut hashes:VecDeque<String>=VecDeque::new();

    let mut found=0;
    for i in 0..1000 {
        hashes.push_back(format!("{:x}", md5::compute(format!("{}{}",input, i))));
    }

    let mut index=1000;

    while found < 64 {
        let line=hashes.pop_front().unwrap();
        hashes.push_back(format!("{:x}", md5::compute(format!("{}{}",input, index))));
        index+=1;

        let mut last_ch='#';
        let mut triple_count=0;
        for ch in line.chars() {

            if ch==last_ch {
                triple_count+=1;
                last_ch=ch;
                if triple_count==3 {
                    break;
                }
            } else {
                triple_count=1;
                last_ch=ch;
            }
        }
        if triple_count==3{
//            println!("Found 3:{} at {}", last_ch, index-1001);
            triple_count=0;

            let mut quinte_count=0;
            'quinte: for hash in &hashes {
                quinte_count=0;
                for ch in hash.chars() {
                    if ch==last_ch {
                        quinte_count+=1;
                        if quinte_count==5 {
//                            println!("Searched {} and found {}",ch, hash);
                            break 'quinte;
                        } 
                    }else {
                        quinte_count=0;
                    }
                }
            }
            if quinte_count==5 {
//                println!("{} found 5 for index {}", found, index-1001);
                found+=1;
            }
        }
    }
    index-1001
}

fn multiple_hashes(input: String) -> String {
    
    let mut out=input;
    for i in 0..2016{
        out=format!("{:x}",md5::compute(out));
    }
    out
}

#[aoc(day14, part2)]
fn solve_part2(input: &String) -> u32 {
    let mut hashes:VecDeque<String>=VecDeque::new();

    let mut found=0;
    for i in 0..1000 {
        hashes.push_back(multiple_hashes(format!("{:x}", md5::compute(format!("{}{}",input, i)))));
    }

    let mut index=1000;

    while found < 64 {
        let line=hashes.pop_front().unwrap();
        hashes.push_back(multiple_hashes(format!("{:x}", md5::compute(format!("{}{}",input, index)))));
        index+=1;

        let mut last_ch='#';
        let mut triple_count=0;
        for ch in line.chars() {

            if ch==last_ch {
                triple_count+=1;
                last_ch=ch;
                if triple_count==3 {
                    break;
                }
            } else {
                triple_count=1;
                last_ch=ch;
            }
        }
        if triple_count==3{
//            println!("Found 3:{} at {}", last_ch, index-1001);
            triple_count=0;

            let mut quinte_count=0;
            'quinte: for hash in &hashes {
                quinte_count=0;
                for ch in hash.chars() {
                    if ch==last_ch {
                        quinte_count+=1;
                        if quinte_count==5 {
//                            println!("Searched {} and found {}",ch, hash);
                            break 'quinte;
                        } 
                    }else {
                        quinte_count=0;
                    }
                }
            }
            if quinte_count==5 {
//                println!("{} found 5 for index {}", found, index-1001);
                found+=1;
            }
        }
    }
    index-1001
}

