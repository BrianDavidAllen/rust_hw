pub struct WordProblem {
    command: String,
}

impl WordProblem {
    pub fn new(input: &str) -> WordProblem {
        WordProblem {
            command: input.to_string().trim_right_matches('?').to_string(),
        }
    }

    pub fn answer(&self) -> Result<i32, &'static str> {
        let v: Vec<&str> = self.command.split(' ').collect();
        let mut ops: Vec<String> = Vec::new();
        let mut nums: Vec<i32> = Vec::new();
        let mut total: i32 = 0;

        for words in &v {
            match words.as_ref() {
                "plus" => ops.push("plus".to_string()),
                "minus" => ops.push("minus".to_string()),
                "divided" => ops.push("divided".to_string()),
                "multiplied" => ops.push("multiplied".to_string()),
                _ => continue,
            }
        }
        for i in 0..v.len() {
            if v[i].parse::<i32>().is_ok() {
                nums.push(v[i].parse::<i32>().unwrap());
            }
        }

        for i in 0..ops.len() {
            if i < 1 {
                match ops[i].as_ref() {
                    "plus" => total = nums[i] + nums[i + 1],
                    "minus" => total = nums[i] - nums[i + 1],
                    "divided" => total = nums[i] / nums[i + 1],
                    "multiplied" => total = nums[i] * nums[i + 1],
                    _ => continue,
                }
            } else {
                match ops[i].as_ref() {
                    "plus" => total += nums[i + 1],
                    "minus" => total -= nums[i + 1],
                    "divided" => total = total / nums[i + 1],
                    "multiplied" => total *= nums[i + 1],
                    _ => continue,
                }
            }
        }
        if nums.len() < 1 || ops.len() < 1 {
            return Err("Nope");
        } else {
            Ok(total)
        }
    }
}
