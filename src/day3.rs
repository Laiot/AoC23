use aoc_runner_derive::{aoc, aoc_generator};

pub struct Numbr {
    value: u32,
    row: usize,
    col: (usize, usize)
}

pub struct Symbl {
    value: char,
    row: usize,
    col: usize
}

#[aoc_generator(day3)]
pub fn day2_gen(input: &str) -> (Vec<Symbl>, Vec<Numbr>) {
    let mut schematic: Vec<Vec<char>> = Vec::new();
    let mut symbols: Vec<Symbl> = Vec::new();
    let mut numbers: Vec<Numbr> = Vec::new();
    let mut tmp_numb = String::new();
    let mut tmp_init_pos: usize = usize::MAX;
    let mut tmp_flag = false;

    for line in input.lines(){
        schematic.push(line.chars().collect());
    }

    for (ir, row) in schematic.iter().enumerate() {

        for (ic, el) in row.iter().enumerate() {
            if el.is_digit(10) {
                tmp_numb.push(*el);

                //Check if number just before new line
                if ic == row.len() - 1 {
                    tmp_flag = false;
                    numbers.push(Numbr {value:tmp_numb.parse::<u32>().unwrap(),col:(tmp_init_pos,ic-1), row: ir });
                    tmp_init_pos = usize::MAX;
                    tmp_numb = String::new();
                } else {
                    if tmp_init_pos == usize::MAX {
                        tmp_init_pos = ic;
                    }
                    tmp_flag = true;
                }

            } else {
                if tmp_flag {
                    tmp_flag = false;
                    numbers.push(Numbr {value:tmp_numb.parse::<u32>().unwrap(),col:(tmp_init_pos,ic-1), row: ir });
                    tmp_init_pos = usize::MAX;
                    tmp_numb = String::new();
                }
                
                if *el != '.' {
                    symbols.push(Symbl {col:ic,row:ir, value: *el });
                }
            } 
        }
    }

    (symbols, numbers)
}

#[aoc(day3, part1)]
pub fn part1(schematic: &(Vec<Symbl>, Vec<Numbr>)) -> u32{
    let mut res = 0;

    for sym in &schematic.0 {
        for n in &schematic.1 {

            let new_min = if n.col.0 == 0 {
                n.col.0
            } else {
                n.col.0 - 1
            };

            if (sym.row.abs_diff(n.row) <= 1) 
                && (new_min..(n.col.1+2)).contains(&sym.col){
                res += n.value;
            }
        }
    }
    res
}

#[aoc(day3, part2)]
pub fn part2(schematic: &(Vec<Symbl>, Vec<Numbr>)) -> u32{
    let mut res = 0;

    for sym in &schematic.0 {

        if sym.value == '*' {
            let mut adder = 1;
            let mut counter: usize = 0;

            for n in &schematic.1 {

                let new_min = if n.col.0 == 0 {
                    n.col.0
                } else {
                    n.col.0 - 1
                };
    
                if (sym.row.abs_diff(n.row) <= 1) && (new_min..(n.col.1+2)).contains(&sym.col){
                    
                    counter += 1;
                    if counter == 3 {
                        break;
                    } else {
                        adder *= n.value;
                    }
                }
            }
            if counter == 2 {
                res += adder;
            }
        }   
    }
    res
}
