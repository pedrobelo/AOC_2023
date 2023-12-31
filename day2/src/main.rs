use std::cmp::max;
use std::collections::HashMap;
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
	let mut map: HashMap<&str, u32> = HashMap::new();

	let single_line = line.split(": ")
		.collect::<Vec<_>>();

	let binding = single_line[1]
		.replace(",", ";");
 	let colors = binding
		.split("; ")
		.collect::<Vec<_>>();

	colors.iter()
		.for_each(|color| {
			let color_and_value = color.split(" ").collect::<Vec<_>>();
			let color = color_and_value[1];
			let value: u32 = color_and_value[0].parse().unwrap();
			map.entry(color)
				.and_modify(|in_map| { *in_map =  max::<u32>(*in_map, value); })
				.or_insert(value);
		});

	let mut result = 1;
	map.iter()
		.for_each(|(_, value)| result *= value);

	return result;
}
