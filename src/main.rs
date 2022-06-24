#[cfg(test)]

#[derive(Debug, PartialEq, Eq)]
struct Point {
	x: u32,
	y: u32
}

#[derive(Debug, PartialEq)]
enum Direction {
	Up,
	Down,
	Left,
	Right
}

struct Snake {
	body: Vec<Point>,
	direction: Direction
}

impl Snake {
	fn new(length: u32) -> Snake {
		let mut body = Vec::<Point>::new();
		for x in 0..length {
			body.push(Point{ x: x, y: 0 });
		}
		Snake {
			body,
			direction: Direction::Right
		}
	}
	
	fn move_to_next_position(&mut self) {
		let head = self.body.last().unwrap();
		let updated_head = match self.direction {
			Direction::Up    => Point { x: head.x, y: head.y.saturating_sub(1) },
			Direction::Down  => Point { x: head.x, y: head.y.saturating_add(1) },
			Direction::Left  => Point { x: head.x.saturating_sub(1), y: head.y },
			Direction::Right => Point { x: head.x.saturating_add(1), y: head.y }
		};
		self.body.push(updated_head);
		self.body.remove(0);
	}
	
	fn change_direction(&mut self, direction: Direction) {
		self.direction = direction;
	}
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn points_are_equal() {
	let p1 = Point{ x: 1, y: 2 };
	let p2 = Point{ x: 1, y: 2 };
	assert_eq!(&p1, &p2);
}

#[test]
fn creating_snake_with_len_3() {
	let snake = Snake::new(3);
	assert_eq!(&snake.body, &[Point{ x: 0, y: 0 },
	                          Point{ x: 1, y: 0 },
							  Point{ x: 2, y: 0 }]);
	assert_eq!(snake.direction, Direction::Right);
}

#[test]
fn moving_around() {
	let mut snake = Snake::new(3);
	
	snake.move_to_next_position();
	
	assert_eq!(&snake.body, &[Point{ x: 1, y: 0 },
	                          Point{ x: 2, y: 0 },
							  Point{ x: 3, y: 0 }]);
	
	snake.change_direction(Direction::Down);
	snake.move_to_next_position();
	
	assert_eq!(&snake.body, &[Point{ x: 2, y: 0 },
	                          Point{ x: 3, y: 0 },
							  Point{ x: 3, y: 1 }]);
							  
	snake.change_direction(Direction::Left);
	snake.move_to_next_position();
	
	assert_eq!(&snake.body, &[Point{ x: 3, y: 0 },
	                          Point{ x: 3, y: 1 },
							  Point{ x: 2, y: 1 }]);
							  
	snake.change_direction(Direction::Up);
	snake.move_to_next_position();
	
	assert_eq!(&snake.body, &[Point{ x: 3, y: 1 },
	                          Point{ x: 2, y: 1 },
							  Point{ x: 2, y: 0 }]);
							  
	snake.change_direction(Direction::Right);
	snake.move_to_next_position();
	
	assert_eq!(&snake.body, &[Point{ x: 3, y: 1 },
	                          Point{ x: 2, y: 1 },
							  Point{ x: 2, y: 0 }]);
}