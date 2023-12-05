use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Conversion {
    dst: u64,
    src: u64,
    rng: u64
}

fn gen_map(map: Vec<Conversion>) -> HashMap<u64, u64> {
    let mut hm: HashMap<u64, u64> = HashMap::new();

    for c in map {
        for i in 0..c.rng {
            hm.insert(c.src + i, c.dst + i);
        }
    }

    hm
}

fn convert_map(val: &mut u64, map: &Vec<Conversion>) {
    for c in map {
        if (c.src..(c.src + c.rng)).contains(&val){
            if c.dst > c.src {
                *val += c.dst - c.src;
            } else {
                *val -= c.src - c.dst;
            }
            break;
        }
    }
}

#[aoc_generator(day5)]
pub fn day2_gen(input: &str) -> (Vec<u64>, Vec<Vec<Conversion>>){

    let mut targets: Vec<u64> =  Vec::new();
    let mut line_iter = input.lines();

    for t in line_iter.next().unwrap().split(": ").last().unwrap().split_whitespace() {
        targets.push(t.parse::<u64>().unwrap());
    }

    line_iter.next();

    let mut maps: Vec<Vec<Conversion>> = Vec::new();
    let mut conversions: Vec<Conversion> = Vec::new();

    for line in line_iter {
        if line != "" {
            if line.contains(":"){
                conversions =  Vec::new();
            } else {
                let mut tmp = line.split_whitespace();
                conversions.push(Conversion { 
                    dst: tmp.next().unwrap().parse::<u64>().unwrap(), 
                    src: tmp.next().unwrap().parse::<u64>().unwrap(), 
                    rng: tmp.next().unwrap().parse::<u64>().unwrap()
                })
            }
        } else {
            maps.push(conversions.clone());
        }
    };
    maps.push(conversions.clone());

    (targets, maps)
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<u64>, Vec<Vec<Conversion>>)) -> u64 {
    let targets = &input.0;
    let maps = &input.1;
    let mut res = u64::MAX;

    for mut t in targets.clone() {
        for map in maps {
            convert_map(&mut t, map);
        }
        if res > t {
            res = t;
        }
    }
    res
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<u64>, Vec<Vec<Conversion>>)) -> u64 {
    let targets = &input.0;
    let mut maps: Vec<HashMap<u64, u64>> = Vec::new();
    let mut res = u64::MAX;

    for map in &input.1 {
        maps.push(gen_map((&map).to_vec()));
        break;
    }
    println!("asd");
    // for t in targets.chunks(2).clone() {
    //     let tmp = t[0].clone();
    //     for i in 0..t[1]{
    //         let mut new_tmp = tmp + i;
    //         for map in maps {
    //             convert_map(&mut new_tmp, map);
    //         }
    //         if res > new_tmp {
    //             res = new_tmp;
    //         }
    //     }
    // }
    res
}
