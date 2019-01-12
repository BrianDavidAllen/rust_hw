pub fn encrypt(input: &str) -> String {
    //unimplemented!("Encrypt {:?} using a square code", input)
    let mut words = Vec::new();
    let mut x = 0;
    let mut y = 0;
    let mut square: Vec<String> = Vec::new();
    let mut words_vec: Vec<String> = Vec::new();
    let mut count = 0;
    let input_string = input.to_lowercase().to_string();
    for c in input_string.chars() {
        if c.is_alphanumeric() == true {
            words.push(c);
        }
    }
    let message_length = words.len();

    if message_length > 0 {
        for i in 1..message_length {
            if i * i - 1 >= message_length {
                x = i;
                y = i - 1;
                break;
            } else if i * i >= message_length {
                x = i;
                y = i;
                break;
            }
        }

        for _i in 0..x {
            square.push(String::new());
            words_vec.push(String::new());
        }

        for i in 0..y {
            for _z in 0..x {
                if count < words.len() {
                    words_vec[i].push(words[count]);
                    count += 1;
                } else {
                    words_vec[i].push(' ');
                }
            }
        }
        count = 0;
        for i in 0..x {
            for z in 0..y {
                let mut a_word = &words_vec[z];
                if i > a_word.len() {
                    break;
                } else {
                    let mut c = a_word.chars().nth(i).unwrap();
                    square[i].push(c);
                    count += 1;
                }
            }
        }
    }
    let mut output: String = String::new();

    count = 0;
    for a_words in &square {
        count += 1;
        if x % y == 0 {
            if count < square.len() {
                output.push_str(&a_words);
                output.push(' ');
            } else {
                output.push_str(&a_words);
            }
        } else {
            if count < square.len() {
                output.push_str(&a_words);
                output.push(' ');
            } else {
                output.push_str(&a_words);
            }
        }
    }
    output
}
