/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {

	let isbn_nums: Vec<&str> = isbn.split("-").collect();
	let mut count: i32 = 10;
	let mut total: i32 = 0;
	let a_char: char;
	println!("{}", isbn_nums[0]);
	
	for indexes in &isbn_nums[..]{
		for c in indexes.chars(){
//			c = c.to_digit(10).unwrap();
			println!("{} {}", c, count);
	
			if c.is_numeric() && count > 1{
				total += c.to_digit(10).unwrap() as i32 *	count;
				count = count -1;
				println!("total is {}", total);
			}
			else if c.is_numeric() && count > 0{
				total += c.to_digit(10).unwrap() as i32;
				println!("last case hit! {}", total);
			}
			else if c == 'x' || c == 'X' {
				total += 10;
			} 
			
			else{
				println!("Too many numbers or invalid characters");
				return false;
			}	
		}
	}

	if total % 11 == 0{
		return true;
	}
	return false;
}
