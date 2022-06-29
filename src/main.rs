#![allow(unused)]

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
	fn new(length: u32, origin_x: u32, origin_y: u32) -> Snake {
		let mut body = Vec::<Point>::new();
		for x in 0..length {
			body.push(Point{ x: origin_x + x, y: origin_y });
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
	
	fn get_body(&self) -> &Vec<Point> {
		&self.body
	}
}

struct SnakePit {
	height: u32,
	width: u32
}

impl SnakePit {
	fn get_perimeter(&self) -> Vec<Point> {
		let mut perimeter = Vec::<Point>::new();
		perimeter.try_reserve_exact((2 * self.width + 2 * self.height - 4) as usize);
		for y in 0..self.height {
			if y == 0 || y == self.height {
				for x in 0..self.width {
					perimeter.push(Point{ x: x, y: y });
				}
			}
			else {
				perimeter.push(Point{ x: 0, y: y });
				perimeter.push(Point{ x: self.width - 1, y: y});
			}
		}
		perimeter
	}
}

struct SnakeEngine {
	snake: Snake,
	snake_pit: SnakePit
}

impl SnakeEngine {
	fn new(snake_pit_height: u32, snake_pit_width: u32) -> SnakeEngine {
		SnakeEngine {
			snake: Snake::new(3, 1, 2),
			snake_pit: SnakePit { height: snake_pit_height, width: snake_pit_width }
		}
	}
	
	fn tick(&mut self) {
		self.snake.move_to_next_position();
	}
}

fn main() {

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn points_are_equal() {
		let p1 = Point{ x: 1, y: 2 };
		let p2 = Point{ x: 1, y: 2 };
		assert_eq!(&p1, &p2);
	}

	#[test]
	fn creating_snake_with_len_3() {
		let snake = Snake::new(3, 0, 0);
		assert_eq!(&snake.body, &[Point{ x: 0, y: 0 },
								  Point{ x: 1, y: 0 },
								  Point{ x: 2, y: 0 }]);
		assert_eq!(snake.direction, Direction::Right);
	}
	
	#[test]
	fn creating_snake_with_offset() {
		let snake = Snake::new(3, 5, 5);
		assert_eq!(&snake.body, &[Point{ x: 5, y: 5 },
								  Point{ x: 6, y: 5 },
								  Point{ x: 7, y: 5 }]);
		assert_eq!(snake.direction, Direction::Right);
	}

	#[test]
	fn moving_around() {
		let mut snake = Snake::new(3, 0, 0);
		
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
		
		assert_eq!(&snake.body, &[Point{ x: 2, y: 1 },
								  Point{ x: 2, y: 0 },
								  Point{ x: 3, y: 0 }]);
	}
	
	#[test]
	fn creating_30x30_snake_engine()
	{
		let mut engine = SnakeEngine::new(30, 30);
	}
	
	#[test]
	fn snake_moving_around_in_snake_engine()
	{
		let mut engine = SnakeEngine::new(30, 30);
							 
	}
}