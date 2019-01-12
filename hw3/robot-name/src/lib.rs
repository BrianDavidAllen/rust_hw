extern crate rand;
use rand::{thread_rng, Rng};
pub struct Robot {
    robot_name: String,
}

impl Robot {
    pub fn new() -> Robot {
        let mut a_name = String::new();
        let mut num: u8 = 0;
        let mut c: char;
        for i in 0..2 {
            c = num as char;
            a_name += &c.to_string();
        }
        a_name.to_uppercase();
        for i in 0..3 {
            a_name += &thread_rng().gen_range(0, 9).to_string();
        }
        Robot { robot_name: a_name }
    }
    pub fn name<'a>(&'a self) -> &'a str {
        &self.robot_name
    }
    pub fn reset_name(&mut self) {
        let mut a_name = String::new();
        self.robot_name = "".to_string();
        let mut num: u8 = 0;
        let mut c: char;
        for i in 0..2 {
            num = thread_rng().gen_range(65, 90);
            c = num as char;
            a_name += &c.to_string();
        }
        a_name.to_uppercase();
        for i in 0..3 {
            a_name += &thread_rng().gen_range(0, 9).to_string();
        }
        self.robot_name = a_name;
    }
}
