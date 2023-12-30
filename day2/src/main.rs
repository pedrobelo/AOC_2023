use std::fs::File;
use std::io::{self, BufRead, Result};
use std::path::Path;

fn main() {
	if let Ok(lines) = read_lines("./input.txt") {
		let sum: u32 = lines.iter()
			.map(|line| process_line(line))
			.sum();
		println!("{}", sum);
	}
}

fn read_lines<P>(filename: P) -> Result<Vec<String>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines()
		.map(|l| l.expect("Could not parse line"))
		.collect())
}

fn process_line(line: &String) -> u32 {
	let single_line = line.split(": ")
		.collect::<Vec<_>>();

	let line_number: u32 = single_line[0]
		.split(" ")
		.collect::<Vec<_>>()[1]
		.parse()
		.unwrap();

	let games = single_line[1]
		.split("; ")
		.map(|game| game.to_string())
		.collect::<Vec<_>>();

	let are_all_games_valid: bool = games.iter()
		.map(|color_and_value| is_set_valid(color_and_value))
		.fold(true, |acc, x| acc && x);
	
	if are_all_games_valid {
		return line_number;
	}
	return 0;
}

fn is_set_valid(set: &String) -> bool {
	set.split(", ")
		.map(|color_and_number| is_color_valid(color_and_number.to_string()))
		.fold(true, |acc, x| acc && x)
}

fn is_color_valid(color: String) -> bool {
	let color_and_value = color.split(" ").collect::<Vec<_>>();
	let value: u32 = color_and_value[0].parse().unwrap();
	let color = color_and_value[1];
	
	if color.eq("red") && value > 12 { return false };
	if color.eq("green") && value > 13 { return false };
	if color.eq("blue") && value > 14 { return false };

	true
}
