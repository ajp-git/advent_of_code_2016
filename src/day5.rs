use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn input_generator(input: &str) -> String {
    "ffykfhsq".to_string()
}

#[aoc(day5, part1)]
fn solve_part1(input: &String) -> String {
//    return "c6697b55".to_string();

    let mut index:u32=0;
    let mut count:u32=0;
    let mut out:String=String::new();

    while count <8 {
        let key=format!("{}{}",input,index);
        let md = md5::compute(key);
        let md_str=format!("{:x}", md);
        print!("\r{}", md_str);
        if md_str.starts_with("00000") {
            println!("Found {} : {}", count, md_str);
            if let Some(c)=md_str.chars().nth(5) {
                out.push(c);
                count+=1;
            }
        }
        index+=1;
    }
    out
}

#[aoc(day5, part2)]
fn solve_part2(input: &String) -> String {
    
    let mut index:u32=0;
    let mut count:u32=0;
    let mut out:String=String::new();
    let mut v_out:Vec<char>=vec![' ';8];

    while v_out.iter().any(|&c| c==' ' ) {
        let key=format!("{}{}",input,index);
        let md = md5::compute(key);
        let md_str=format!("{:x}", md);

        print!("\r{}", md_str);
        if md_str.starts_with("00000") {
            println!("Found {} : {}", count, md_str);
            if let Some(c)=md_str.chars().nth(5) {
                if c>='0' && c<='7' {
                    let c=(c as u8- '0' as u8) as usize;
                    if v_out[c] ==' ' {
                        if let Some(k)=md_str.chars().nth(6) {
                            v_out[c]=k;
                            count+=1;    
                        }
                    }
                }
            }
        }
        index+=1;
    }
    v_out.iter().collect::<String>()
}
