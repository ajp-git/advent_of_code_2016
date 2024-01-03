use std::collections::HashMap;

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
#[derive(Clone, Debug)]
struct Bot{
    id:u32,
    low_bot:Option<u32>,
    high_bot:Option<u32>,
    low_output:Option<u32>,
    high_output:Option<u32>,
    low_val:Option<u32>,
    high_val:Option<u32>
}

impl Bot {
    fn new(id: u32) -> Bot{
        return Bot{id, low_bot: None, high_bot:None, low_output: None, high_output:None, low_val:None, high_val:None};
    }

    fn run(&mut self, h_bots: &mut HashMap<u32, Bot>) -> Option<u32> {
        println!("Running bot {}", self.id);
    
        let mut actions = Vec::new();
    
        if self.low_val.is_some() && self.high_val.is_some() {
            if self.low_val.unwrap() == 17 && self.high_val.unwrap() == 61 {
                return Some(self.id);
            }
    
            if let Some(lb) = self.low_bot {
                actions.push((lb, self.low_val.unwrap()));
                self.low_val = None;
            }
    
            if let Some(hb) = self.high_bot {
                actions.push((hb, self.high_val.unwrap()));
                self.high_val = None;
            }
        }
    
        for (bot_id, value) in actions {
            if let Some(bot) = h_bots.get_mut(&bot_id) {
                if let Some(ret) = bot.set_value(value, h_bots) {
                    return Some(ret);
                }
            }
        }
    
        None
    }
    

    fn set_value(&mut self, val: u32, h_bots:&mut HashMap<u32, Bot>) -> Option<u32>{
        if let Some(low) = self.low_val {
            if low<val{
                self.high_val=Some(val);
                println!("Setting value high {} for bot {}", val, self.id);
            } else {
                self.high_val=self.low_val;
                println!("Setting value low {} for bot {}", val, self.id);
                self.low_val=Some(val);
            }
        } else {
            println!("Setting initial value low {} for bot {}", val, self.id);
            self.low_val=Some(val);
        }
        self.run(h_bots)
    }
    
}
#[aoc(day10, part1)]
fn solve_part1(input: &Vec<String>) -> u32 {

    let mut h_bots:HashMap<u32,Bot>=HashMap::new();
    let re_val = Regex::new(r"value (\d*) goes to bot (\d)").unwrap();
    let re_bot_gives = Regex::new(r"bot (\d*) gives low to (bot|output) (\d*) and high to (bot|output) (\d*)").unwrap();
    for line in input{
        println!("\n{line}");

        if let Some(caps) = re_val.captures(&line) {
            let dest_bot = caps[2].parse::<u32>().unwrap();
            let dest_val = caps[1].parse::<u32>().unwrap();

            if let Some(mut bot) = h_bots.remove(&dest_bot) {
                let ret = bot.set_value(dest_val, &mut h_bots);
                h_bots.insert(dest_bot, bot); // Put the bot back after modification
                if ret.is_some() { return ret.unwrap(); }
            } else {
                let mut t_bot = Bot::new(dest_bot);
                let ret = t_bot.set_value(dest_val, &mut h_bots);
                h_bots.insert(dest_bot, t_bot);
                if ret.is_some() { return ret.unwrap(); }
            }
        } else if let Some(caps) = re_bot_gives.captures(&line) {
            let curr_bot_id=caps[1].parse::<u32>().unwrap();
            let mut curr_bot=h_bots.get_mut(&curr_bot_id).unwrap().clone();
            match &caps[2] { // Low value
                "bot" => {
                    let dest_bot_id=caps[3].parse::<u32>().unwrap();
                    let mut dest_bot=h_bots.entry(dest_bot_id).or_insert_with(|| Bot::new(dest_bot_id));
                    curr_bot.low_bot=Some(dest_bot_id);
                    
                },
                "output" => {},
                _ => { panic!("Unknown recipient {}", line);},
            }
            match &caps[4] { // Low value
                "bot" => {
                    let dest_bot_id=caps[5].parse::<u32>().unwrap();
                    
                    let mut dest_bot=h_bots.entry(dest_bot_id).or_insert_with(|| Bot::new(dest_bot_id));
                    curr_bot.high_bot=Some(dest_bot_id);
                    
                },
                "output" => {},
                _ => { panic!("Unknown recipient {}", line);},
            }
            curr_bot.run(&mut h_bots);
        }
    }
    println!("Hash : {:?}", h_bots);
    0
}