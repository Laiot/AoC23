use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32{
    input.lines()
		.map(|line| {
			let mut line = line.chars().filter_map(|c| c.to_digit(10));
			let first_char = line.next().unwrap().to_string();
			let last_char = line.last();
			if last_char.is_some(){
				return  (first_char + &last_char.unwrap().to_string()).parse::<u32>().unwrap();
			} else {
				return (first_char.repeat(2)).parse::<u32>().unwrap();
			}
		}).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    0
}