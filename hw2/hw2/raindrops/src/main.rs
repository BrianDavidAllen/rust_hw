extern crate raindrops;
use std::env;

fn main() {
	let input: Vec<String> = env::args().collect();
	let _number: u32 = input[input.len() -1].parse().unwrap();
	println!("{}", raindrops::raindrops(_number)); 

}
