use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Draw{
    r: u32,
    b: u32,
    g: u32
}

#[derive(Debug)]
pub struct Game{
    id: u32,
    draws: Vec<Draw>
}

#[aoc_generator(day2)]
pub fn day2_gen(input: &str) -> Vec<Game> {
    let mut res = Vec::new();
    let mut counter: u32 = 1;

    for line in input.lines(){
        let mut fdraws: Vec<Draw> = Vec::new();
        let fixd = line.split(":");

        let draws = fixd.last().unwrap().split(";");

        for draw in draws {
            let mut r = 0;
            let mut b = 0;
            let mut g = 0;

            let elems = draw.split(",");

            for elem in elems {
                let mut tmp = elem.split(" ");
                tmp.next();
                let n = tmp.next().unwrap().parse::<u32>().unwrap();
                let c = tmp.next().unwrap();

                match c {
                    "red" => r += n,
                    "blue" => b += n,
                    "green" => g += n,
                    _ => println!("Unmatched")
                }
            }
            fdraws.push(Draw { r, b, g })
        }
        res.push(Game { id: counter, draws: fdraws });
        counter += 1;
    }

    res
} 

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Game>) -> u32{
    let mut res: u32 = 0;
    
    for game in input {
        let mut is_valid = true;
        for draw in game.draws.clone() {
            if draw.r > 12 || draw.g > 13 || draw.b > 14 {
                is_valid = false;
            }
        }
        if is_valid {
            res += game.id;
        }
    }
    
    res
	}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Game>) -> u32 {
    let mut res: u32 = 0;
    
    for game in input {
        let mut min_r: u32 = 0; 
        let mut min_b: u32 = 0; 
        let mut min_g: u32 = 0; 

        for draw in game.draws.clone() {
            if draw.r > min_r {
                min_r = draw.r;
            }

            if draw.b > min_b {
                min_b = draw.b;
            }

            if draw.g > min_g {
                min_g = draw.g;
            }
        }
        res += min_r * min_b * min_g;
    }
    
    res
}