use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Card{
    wins: Vec<u32>,
    ownd: Vec<u32>
}

#[aoc_generator(day4)]
pub fn day2_gen(input: &str) -> Vec<Card>{

    let mut cards: Vec<Card> =  Vec::new();

    for line in input.lines(){
        let mut tmp = line.split(": ").last().unwrap().split(" | ");

        let wins: Vec<u32> = tmp.next().unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        let ownd: Vec<u32> = tmp.next().unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();

        cards.push(Card { 
            wins, 
            ownd
        });
    }
    cards
}

#[aoc(day4, part1)]
pub fn part1(cards: &Vec<Card>) -> u32 {
    let mut res: u32 = 0;
    for card in cards {
        let mut card_res = 0;
        for ownd in &card.ownd {
            if card.wins.contains(&ownd) {
                if card_res == 0 {
                    card_res = 1;
                } else {
                    card_res *= 2;
                }
            }
        }
        res += card_res;
    }
    res
}

#[aoc(day4, part2)]
pub fn part2(cards: &Vec<Card>) -> usize {
    let mut counter: usize = 0;
    let mut res: usize = 0;
    let mut copies: Vec<(usize, usize)> = Vec::new();

    for (idx, card) in cards.iter().enumerate() {
        for ownd in &card.ownd {
            if card.wins.contains(&ownd) {
                counter += 1;    
            }
        }
        copies.push((idx, counter));
        counter = 0;
    }
    
    let mut copies_trees: Vec<usize> = vec![0; copies.len()];
    copies.reverse();

    for card in copies{

        copies_trees[card.0] += 1;

        if card.1 > 0 {
            for i in (card.0 + 1)..=(card.0 + card.1) {
                copies_trees[card.0] += copies_trees[i];
            }
            res += copies_trees[card.0];
        } else {
            res += 1;
        };

    }
    res
}
