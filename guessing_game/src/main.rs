extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
	println!("Please select a version (default version 1)");
	let version = get_integer_input(1);
	if version == 1 {
		version_one();
	} else {
		println!("To be implemented");
	}
}


fn version_one() {
	println!("Guessing game version 1: Guess a number in a given range");
	let (lower, upper) = get_bounds();
	let secret_number = get_secret_number(lower, upper);
	println!("Please input your guess.");
	loop {
		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");
		let guess: u32 = match guess.trim().parse() {
			Ok(num) 	=> num,
			Err(_) 		=> continue,
		};
		println!("You guessed: {}", guess);
		
		match guess.cmp(&secret_number) {
			Ordering::Less 		=> println!("Too small"),
			Ordering::Greater   => println!("Too big"),
			Ordering::Equal		=> { 
				println!("Correct!");
				break;
			}
		}
	}
}


fn get_secret_number(lower: u32, upper: u32) -> u32 {
	rand::thread_rng().gen_range(lower, upper)
}


fn get_bounds() -> (u32, u32) {
	println!("Please input a lower bound.");
	let mut lower = get_integer_input(1);
	
	println!("Please input an upper bound.");
	let mut upper = get_integer_input(10);
	
	// Swap two bounds if they are in wrong order
	if upper < lower {
		let tmp = lower;
		lower = upper;
		upper = tmp;
	}
	println!("Lower bound is set at: {}, Upper bound is set at: {}", lower, upper);
	
	return (lower, upper);
}


fn get_integer_input(default: u32) -> u32 {
	let mut input = String::new();
	io::stdin().read_line(&mut input)
		.expect("Failed to read line");
	let input: u32 = match input.trim().parse() {
		Ok(num)		=> num,
		Err(_) 	=> default,
	};
	return input;
}