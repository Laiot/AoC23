use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Card{
    wins: Vec<u32>,
    ownd: Vec<u32>,
    copies: usize
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
            ownd,
            copies: 1 
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
    let mut ccards: Vec<Card> = cards.to_vec();
    let mut counter: usize = 0;
    let mut res: usize = 0;
    for (idx, card) in cards.iter().enumerate() {
        for _ in 0..ccards.get(idx).unwrap().copies {
            res += 1;
            for ownd in &card.ownd {
                if card.wins.contains(&ownd) {
                    counter += 1;    
                }
            }
            for i in 1..=counter {
                
                ccards[idx + i].copies += 1;
            }
            counter = 0;
        }
    }
    res
}
