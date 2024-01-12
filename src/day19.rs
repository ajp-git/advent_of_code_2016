use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day19)]
fn input_generator(input: &str) -> String {
    input.lines().next().unwrap().to_string();
    3014387.to_string()
//    5.to_string()
}

#[aoc(day19, part1)]
fn solve_part1(input: &String) -> u32 {
    let mut nb_elfes = input.parse::<usize>().unwrap();

    let nb_b=format!("{:b}", nb_elfes);
    println!("elfes {}", nb_b);
    let nb_2=format!("{}{}", &nb_b[1..], nb_b.chars().next().unwrap());
    println!("elfes {}", nb_2);
    u32::from_str_radix(&nb_2, 2).unwrap()
}


#[aoc(day19, part2)]
fn solve_part2(input: &String) -> u32 {
    let mut nb_elfes = input.parse::<usize>().unwrap();
    let mut n=1;
    while n*3 <nb_elfes {
        n*=3;
    }
    nb_elfes as u32-n as u32
}
