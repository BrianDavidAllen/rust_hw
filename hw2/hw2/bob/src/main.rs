extern crate bob;
extern crate inflections;
use std::env; 
use inflections::case::is_upper_case;
fn main() {
	let input: Vec<String> = env::args().collect();
	let mut message: String = String::new();
	let vec_size = input.len();
	for words in &input[1..]{
		message = message + " ";
		message = message + words;
	}
	
	println!("{}" ,bob::reply(&message));
}
