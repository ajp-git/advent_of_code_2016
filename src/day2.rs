
#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<String> {
    let mut v:Vec<String>=Vec::new();

    for line in input.lines() {
        v.push(line.to_string());        
    }
     v
}


#[aoc(day2, part1)]
fn solve_part1(input: &Vec<String>) -> String {

    let mut x=1;
    let mut y=1;

    let mut code:Vec<char>=Vec::new();
    for line in input {
        for c in line.chars(){
            match c {
                'U' => {
                    if y!=0 {y-=1;}
                },
                'R' =>  {
                    if x!=2 { x+=1;}
                },
                'L' => {
                    if x!=0 { x-=1;}
                },
                'D' => {
                    if y!=2 { y+=1;}
                },

                _ => panic!("invalid char"),
            }
        }
        code.push(get_digit(x, y));
    }
    code.into_iter().collect()
}

fn get_digit(x:u8, y:u8) ->char {
    ('1' as u8 + x + 3*y) as char
}
#[aoc(day2, part2)]
fn solve_part2(input: &Vec<String>) -> String {
    let mut code:String=String::new();
/*
    1
  2 3 4
5 6 7 8 9
  A B C
    D
 */

    let mut x:u8=0;
    let mut y:u8=2;

    for line in input {
        for c in line.chars(){
            match c {
                'U' => {
                    if is_possible_move(x as i8, y as i8-1 ){y-=1;}
                },
                'R' =>  {
                    if is_possible_move(x as i8+1, y as i8){x+=1;}
                },
                'L' => {
                    if is_possible_move(x as i8-1, y as i8){x-=1;}
                },
                'D' => {
                    if is_possible_move(x as i8, y as i8+1){y+=1;}
                },

                _ => panic!("invalid char"),
            }
        }
        code.push(get_keypad_key(x, y));
    }
code
}

fn get_keypad_key(x:u8, y:u8)-> char{
    let keypad:Vec<char>=vec![
        '0','0','1','0','0',
        '0','2','3','4','0',
        '5','6','7','8','9',
        '0','A','B','C','0',
        '0','0','D','0','0'];
    keypad[x as usize+5*y as usize]
}

fn is_possible_move(x:i8, y:i8)-> bool{
    if x <0 || y<0 || x>4||y>4 {return false;}
    let keypad:Vec<char>=vec![
        '0','0','1','0','0',
        '0','2','3','4','0',
        '5','6','7','8','9',
        '0','A','B','C','0',
        '0','0','D','0','0'];
    keypad[x as usize+5*y as usize] !='0'
}
