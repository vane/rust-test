extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1, 101);

	//println!("The secret number is: {}", secret_number);

	loop {
		println!("Please input your guess.");

		let reader = io::stdin();
		let mut input_text = String::new();

		reader.read_line(&mut input_text)
			.ok()
			.expect("Failed to read line");

		print!("You guessed: {}", input_text);
		let input_int: u32 = match input_text.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("not a number");
				continue;
			}
		};
			//.ok()
			//.expect("Please type a number!");

		match input_int.cmp(&secret_number) {
			Ordering::Less    => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal   => {
				println!("You win!");
				break;
			}
		}
	}
}