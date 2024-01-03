use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<String> {

    let mut v:Vec<String>=Vec::new();
    for line in input.lines(){
        v.push(line.to_string());
    }
    v
}

#[derive(Debug)]
enum BotDestination {
    Bot(u32),
    Bin(u32),
}

impl BotDestination {
    fn new (kind: &str, val: u32) -> BotDestination {
        match kind {
            "bot" => BotDestination::Bot(val),
            "output" => BotDestination::Bin(val),
            _ => panic!("Imppossible kind"),
        }
    }
}

#[derive(Debug)]
struct BotCommand {
    from: u32,
    to_low: BotDestination,
    to_high: BotDestination,
}

#[aoc(day10, part1)]
fn solve_part1(input: &Vec<String>) -> u32 {

    let mut h_bots:HashMap<u32, HashSet<u32>>=HashMap::new();
    let mut h_bins:HashMap<u32, HashSet<u32>>=HashMap::new();
    let mut v_commands:Vec<BotCommand>=Vec::new();

    let re_val = Regex::new(r"value (\d*) goes to bot (\d*)").unwrap();
    let re_bot_gives = Regex::new(r"bot (\d*) gives low to (bot|output) (\d*) and high to (bot|output) (\d*)").unwrap();
    for line in input{
        println!("\n{line}");

        if let Some(caps) = re_val.captures(&line) {
            let dest_bot = caps[2].parse::<u32>().unwrap();
            let dest_val = caps[1].parse::<u32>().unwrap();
            h_bots.entry(dest_bot).or_default().insert(dest_val);

        } else if let Some(caps) = re_bot_gives.captures(&line) {
            let curr_bot_id=caps[1].parse::<u32>().unwrap();

            v_commands.push(BotCommand{
                from:curr_bot_id,
                to_low:BotDestination::new(&caps[2], caps[3].parse::<u32>().unwrap()),
                to_high:BotDestination::new(&caps[4], caps[5].parse::<u32>().unwrap() )});
            

        }
    }
//    println!("Hash : {:?}", h_bots);
//    println!("Commands : {:?}", v_commands);


    let mut found_part1:Option<u32>=None;
    let mut max_cycles=10_000;

    loop {
        print!("\rCycle {}", max_cycles);
        max_cycles-=1;
        if max_cycles==0{break;}

        for command in &v_commands {
            if let Some(vals) = h_bots.get_mut(&command.from) {
                if vals.len()>=2 {
                    let low_val=*vals.iter().min().unwrap();
                    let high_val=*vals.iter().max().unwrap();
                    if found_part1.is_none() && low_val==17 && high_val==61 {
                        found_part1=Some(command.from);
                    }

                    vals.remove(&low_val);
                    vals.remove(&high_val);

                    match command.to_low {
                        BotDestination::Bot(bot) => { 
                            h_bots.entry(bot).or_default().insert(low_val);

                        },
                        BotDestination::Bin(bin) => {
                            h_bins.entry(bin).or_default().insert(low_val);

                        }
                        
                    };
                    match command.to_high{
                        BotDestination::Bot(bot) => { 
                            h_bots.entry(bot).or_default().insert(high_val);

                        },
                        BotDestination::Bin(bin) => {
                            h_bins.entry(bin).or_default().insert(high_val);

                        }
                        
                    };
                }
            }
        }
        if found_part1.is_some(){
            break;
        }
    }
    found_part1.unwrap()
}

#[aoc(day10, part2)]
fn solve_part2(input: &Vec<String>) -> u32 {

    let mut h_bots:HashMap<u32, HashSet<u32>>=HashMap::new();
    let mut h_bins:HashMap<u32, HashSet<u32>>=HashMap::new();
    let mut v_commands:Vec<BotCommand>=Vec::new();

    let re_val = Regex::new(r"value (\d*) goes to bot (\d*)").unwrap();
    let re_bot_gives = Regex::new(r"bot (\d*) gives low to (bot|output) (\d*) and high to (bot|output) (\d*)").unwrap();
    for line in input{
        println!("\n{line}");

        if let Some(caps) = re_val.captures(&line) {
            let dest_bot = caps[2].parse::<u32>().unwrap();
            let dest_val = caps[1].parse::<u32>().unwrap();
            h_bots.entry(dest_bot).or_default().insert(dest_val);

        } else if let Some(caps) = re_bot_gives.captures(&line) {
            let curr_bot_id=caps[1].parse::<u32>().unwrap();

            v_commands.push(BotCommand{
                from:curr_bot_id,
                to_low:BotDestination::new(&caps[2], caps[3].parse::<u32>().unwrap()),
                to_high:BotDestination::new(&caps[4], caps[5].parse::<u32>().unwrap() )});
            

        }
    }
//    println!("Hash : {:?}", h_bots);
//    println!("Commands : {:?}", v_commands);


    let mut found_part1:Option<u32>=None;
    let mut max_cycles=10_000;

    loop {
        print!("\rCycle {}", max_cycles);
        max_cycles-=1;
        if max_cycles==0{break;}

        for command in &v_commands {
            if let Some(vals) = h_bots.get_mut(&command.from) {
                if vals.len()>=2 {
                    let low_val=*vals.iter().min().unwrap();
                    let high_val=*vals.iter().max().unwrap();
                    if found_part1.is_none() && low_val==17 && high_val==61 {
                        found_part1=Some(command.from);
                    }

                    vals.remove(&low_val);
                    vals.remove(&high_val);

                    match command.to_low {
                        BotDestination::Bot(bot) => { 
                            h_bots.entry(bot).or_default().insert(low_val);

                        },
                        BotDestination::Bin(bin) => {
                            h_bins.entry(bin).or_default().insert(low_val);

                        }
                        
                    };
                    match command.to_high{
                        BotDestination::Bot(bot) => { 
                            h_bots.entry(bot).or_default().insert(high_val);

                        },
                        BotDestination::Bin(bin) => {
                            h_bins.entry(bin).or_default().insert(high_val);

                        }
                        
                    };
                }
            }
        }
    }
    println!("Bin 0 :{:?}", h_bins.get(&0));
    println!("Bin 1 :{:?}", h_bins.get(&1));
    println!("Bin 2 :{:?}", h_bins.get(&2));

    // Retrieve the values from bins 0, 1, and 2
    let bin0_value = *h_bins.get(&0).unwrap().iter().next().unwrap();
    let bin1_value = *h_bins.get(&1).unwrap().iter().next().unwrap();
    let bin2_value = *h_bins.get(&2).unwrap().iter().next().unwrap();

    let product = bin0_value * bin1_value * bin2_value;

    product // Return the product
}