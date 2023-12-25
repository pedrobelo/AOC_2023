use std::fs::File;
use std::io::{self, BufRead, Result, Lines, BufReader};
use std::path::Path;

fn main() {
	// File hosts.txt must exist in the current path
	if let Ok(lines) = read_lines("./input.txt") {
		println!("Problem 1: {}", problem1(lines));
	}

	if let Ok(lines) = read_lines("./input.txt") {
		println!("Problem 2: {}", problem2(lines));
	}
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

fn problem1(lines: impl Iterator<Item = Result<String>>) -> u32 {
	let mut sum = 0;
	for line in lines {
		if let Ok(ip) = line {

			let first_digit = ip.chars()
				.find(|a| a.is_digit(10))
				.and_then(|a| a.to_digit(10));

			let last_digit = ip.chars()
				.rev()
				.find(|a| a.is_digit(10))
				.and_then(|a| a.to_digit(10));

			sum = sum + first_digit.unwrap_or(0)*10 + last_digit.unwrap_or(0);
		}
	}
	return sum;
}

fn problem2(lines: impl Iterator<Item = Result<String>>) -> u32 {
	let mut sum = 0;
	for line in lines {
		if let Ok(ip) = line {
			let ip = ip
				.replace("one", "one1one")
				.replace("two", "two2two")
				.replace("three", "three3three")
				.replace("four", "four4four")
				.replace("five", "five5five")
				.replace("six", "six6six")
				.replace("seven", "seven7seven")
				.replace("eight", "eight8eight")
				.replace("nine", "nine9nine");

			let first_digit = ip.chars()
				.find(|a| a.is_digit(10))
				.and_then(|a| a.to_digit(10));

			let last_digit = ip.chars()
				.rev()
				.find(|a| a.is_digit(10))
				.and_then(|a| a.to_digit(10));

			sum = sum + first_digit.unwrap_or(0)*10 + last_digit.unwrap_or(0);
		}
	}
	return sum;
}
