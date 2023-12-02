use aoc_runner_derive::aoc;
use regex::Regex;

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

    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine|zero").unwrap();
	let mut res = 0;

	for line in input.lines(){
		let caps = re.find_iter(line)
			.map(|m| m.as_str())
			.collect::<Vec<_>>();

		let fst = *caps.first().unwrap();
		let lst = *caps.last().unwrap_or(&fst);
		
		let fst_n: u32 = match fst {
			"1" | "one"   => 1,
			"2" | "two"   => 2,
			"3" | "three" => 3,
			"4" | "four"  => 4,
			"5" | "five"  => 5,
			"6" | "six"   => 6,
			"7" | "seven" => 7,
			"8" | "eight" => 8,
			"9" | "nine"  => 9,
			_ => unreachable!(),
		};

		let lst_n: u32 = match lst {
			"1" | "one"   => 1,
			"2" | "two"   => 2,
			"3" | "three" => 3,
			"4" | "four"  => 4,
			"5" | "five"  => 5,
			"6" | "six"   => 6,
			"7" | "seven" => 7,
			"8" | "eight" => 8,
			"9" | "nine"  => 9,
			_ => unreachable!(),
		};

        res += (fst_n * 10) + lst_n;
	}

	res
}