pub fn raindrops(n: u32) -> String {
    //unimplemented!("what sound does Raindrop #{} make?", n)
	let mut rain_sounds = String::new();
	if n % &3 == 0{
		rain_sounds = rain_sounds + &"Pling".to_string();
	}
	if n % &5 == 0{
		rain_sounds = rain_sounds + &"Plang".to_string();
	}
	if n % &7 == 0{
		rain_sounds = rain_sounds + &"Plong".to_string();
	}
	
	if rain_sounds.len() < 1{
		rain_sounds = n.to_string();
	}	
	
	rain_sounds

}
