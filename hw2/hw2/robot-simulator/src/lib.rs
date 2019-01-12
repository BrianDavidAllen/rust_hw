// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot{
	x: isize,
	y: isize,
    direction: Direction,	
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
		Robot { x: x, y: y, direction: d }	
    }

    pub fn turn_right(mut self) -> Self {
    	if self.direction == Direction::North{ 
			self.direction = Direction::East;
		}
		else if self.direction == Direction::East{
			self.direction = Direction::South;
		}
		else if self.direction == Direction::South{
			self.direction = Direction::West;
		}
		else if self.direction == Direction::West{
			self.direction = Direction::North;
		}
		self
	}

    pub fn turn_left(mut self) -> Self {
    	if self.direction == Direction::North{ 
			self.direction = Direction::West;
		}
		else if self.direction == Direction::East{
			self.direction = Direction::North;
		}
		else if self.direction == Direction::South{
			self.direction = Direction::East;
		}
		else if self.direction == Direction::West{
			self.direction = Direction::South;
		}
		self
	}

    pub fn advance(mut self) -> Self {
    	let mut _x: isize = 0;
		let mut _y: isize = 0;
		if self.direction == Direction::North{ 
			_y += 1;
		}
		else if self.direction == Direction::East{
			_x += 1;
		}
		else if self.direction == Direction::South{
			_y += -1;
		}
		else if self.direction == Direction::West{
			_x += -1;
		}
		self.x = self.x + _x;
		self.y = self.y + _y;
		self
	}

    pub fn instructions(self, instructions: &str) -> Self {
		let mut self_robot = self;    	

		for c in instructions.chars(){
			if c == 'L'{
				self_robot = self_robot.turn_left();
			}
			else if c == 'R'{
				self_robot = self_robot.turn_right();
			}
			else if c == 'A'{
				self_robot = self_robot.advance();
			}
		}
		self_robot
	}

    pub fn position(&self) -> (isize, isize) {
    	(self.x, self.y)
	}

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
