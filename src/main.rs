#![allow(unused)]

use crossterm::{
    cursor,
    event::{self, Event},
    style::Print,
    terminal, ExecutableCommand,
};
use rand::Rng;
use std::io::{stdout, Write};
use std::{
    thread,
    time::{self, Instant},
};

#[derive(Debug, PartialEq, Eq, Default)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: Vec<Point>,
    direction: Direction,
    longer_on_next_move: bool,
}

impl Snake {
    fn new(length: u32, origin: Point) -> Snake {
        let mut body = Vec::<Point>::new();
        for x in 0..length {
            body.push(Point {
                x: origin.x + x,
                y: origin.y,
            });
        }
        Snake {
            body,
            direction: Direction::Right,
            longer_on_next_move: false,
        }
    }

    fn move_to_next_position(&mut self) {
        let head = self.body.last().unwrap();
        let updated_head = match self.direction {
            Direction::Up => Point {
                x: head.x,
                y: head.y.saturating_sub(1),
            },
            Direction::Down => Point {
                x: head.x,
                y: head.y.saturating_add(1),
            },
            Direction::Left => Point {
                x: head.x.saturating_sub(1),
                y: head.y,
            },
            Direction::Right => Point {
                x: head.x.saturating_add(1),
                y: head.y,
            },
        };
        self.body.push(updated_head);
        if !self.longer_on_next_move {
            self.body.remove(0);
        }
        self.longer_on_next_move = false;
    }

    fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn get_body(&self) -> &Vec<Point> {
        &self.body
    }

    fn make_longer(&mut self) {
        self.longer_on_next_move = true;
    }
}

struct SnakePit {
    height: u32,
    width: u32,
}

impl SnakePit {
    fn get_perimeter(&self) -> Vec<Point> {
        let mut perimeter = Vec::<Point>::new();
        perimeter.try_reserve_exact((2 * self.width + 2 * self.height - 4) as usize);
        for y in 0..self.height {
            if y == 0 || y == self.height {
                for x in 0..self.width {
                    perimeter.push(Point { x: x, y: y });
                }
            } else {
                perimeter.push(Point { x: 0, y: y });
                perimeter.push(Point {
                    x: self.width - 1,
                    y: y,
                });
            }
        }
        perimeter
    }
}

struct SnakeEngine {
    snake: Snake,
    snake_pit: SnakePit,
    snack_position: Point,
}

impl SnakeEngine {
    fn new(snake_pit_height: u32, snake_pit_width: u32) -> SnakeEngine {
        SnakeEngine {
            snake: Snake::new(3, Point { x: 2, y: 2 }),
            snake_pit: SnakePit {
                height: snake_pit_height,
                width: snake_pit_width,
            },
            snack_position: Point::default(),
        }
    }

    fn change_snake_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction);
    }

    fn tick(&mut self) {
        self.snake.move_to_next_position();
    }

    fn generate_snack(&mut self) {
        let mut rng = rand::thread_rng();
        self.snack_position = Point {
            x: rng.gen_range(1..self.snake_pit.width),
            y: rng.gen_range(1..self.snake_pit.height),
        };
    }
}

fn clear_display() {
    stdout().execute(terminal::Clear(terminal::ClearType::All));
}

fn display_snake_pit(snake_pit: &SnakePit) {
    stdout().execute(cursor::MoveTo(0, 0));
    for y in 0..snake_pit.height {
        if y == 0 || y == snake_pit.height - 1 {
            for x in 0..snake_pit.width {
                print!("#");
            }
            print!("\n");
        } else {
            print!("#");
            for x in 1..snake_pit.width - 1 {
                print!(" ");
            }
            print!("#\n");
        }
    }
}

fn display_snake(snake_body: &Vec<Point>) {
    let mut stdout = stdout();
    for point in snake_body.iter() {
        stdout.execute(cursor::MoveTo(point.x as u16, point.y as u16));
        stdout.execute(Print("#"));
    }
}

fn wait_for_latest_event(timeout: u32) -> Option<event::KeyCode> {
    let limit = Instant::now() + time::Duration::from_millis(1000);
    let mut latest_event: Option<event::KeyCode> = None;
    while limit - Instant::now() > time::Duration::from_millis(0) {
        if event::poll(limit - Instant::now()).unwrap() {
            match event::read().unwrap() {
                Event::Key(event) => latest_event = Some(event.code),
                _ => (),
            }
        }
    }
    latest_event
}

fn main() {
    let mut snake_engine = SnakeEngine::new(30, 30);

    loop {
        clear_display();
        display_snake_pit(&snake_engine.snake_pit);
        display_snake(&snake_engine.snake.body);
        let event = wait_for_latest_event(1000);
        match event {
            Some(event::KeyCode::Up) => snake_engine.change_snake_direction(Direction::Up),
            Some(event::KeyCode::Left) => snake_engine.change_snake_direction(Direction::Left),
            Some(event::KeyCode::Right) => snake_engine.change_snake_direction(Direction::Right),
            Some(event::KeyCode::Down) => snake_engine.change_snake_direction(Direction::Down),
            _ => (),
        }
        snake_engine.tick();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_are_equal() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 1, y: 2 };
        assert_eq!(&p1, &p2);
    }

    #[test]
    fn creating_snake_with_len_3() {
        let snake = Snake::new(3, Point { x: 0, y: 0 });
        assert_eq!(
            &snake.body,
            &[
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 }
            ]
        );
        assert_eq!(snake.direction, Direction::Right);
    }

    #[test]
    fn creating_snake_with_offset() {
        let snake = Snake::new(3, Point { x: 5, y: 5 });
        assert_eq!(
            &snake.body,
            &[
                Point { x: 5, y: 5 },
                Point { x: 6, y: 5 },
                Point { x: 7, y: 5 }
            ]
        );
        assert_eq!(snake.direction, Direction::Right);
    }

    #[test]
    fn moving_around() {
        let mut snake = Snake::new(3, Point { x: 0, y: 0 });

        snake.move_to_next_position();

        assert_eq!(
            &snake.body,
            &[
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 }
            ]
        );

        snake.change_direction(Direction::Down);
        snake.move_to_next_position();

        assert_eq!(
            &snake.body,
            &[
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
                Point { x: 3, y: 1 }
            ]
        );

        snake.change_direction(Direction::Left);
        snake.move_to_next_position();

        assert_eq!(
            &snake.body,
            &[
                Point { x: 3, y: 0 },
                Point { x: 3, y: 1 },
                Point { x: 2, y: 1 }
            ]
        );

        snake.change_direction(Direction::Up);
        snake.move_to_next_position();

        assert_eq!(
            &snake.body,
            &[
                Point { x: 3, y: 1 },
                Point { x: 2, y: 1 },
                Point { x: 2, y: 0 }
            ]
        );

        snake.change_direction(Direction::Right);
        snake.move_to_next_position();

        assert_eq!(
            &snake.body,
            &[
                Point { x: 2, y: 1 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 }
            ]
        );
    }

    #[test]
    fn creating_30x30_snake_engine() {
        let mut engine = SnakeEngine::new(30, 30);

        assert_eq!(engine.snake_pit.height, 30);
        assert_eq!(engine.snake_pit.width, 30);

        let snake_body = &engine.snake.body;

        assert_eq!(snake_body.len(), 3);
        assert_eq!(
            snake_body,
            &[
                Point { x: 2, y: 2 },
                Point { x: 3, y: 2 },
                Point { x: 4, y: 2 }
            ]
        );
    }

    #[test]
    fn snake_moving_around_in_snake_engine() {
        let mut engine = SnakeEngine::new(30, 30);

        engine.tick();

        let snake_body = &engine.snake.body;

        assert_eq!(snake_body.len(), 3);
        assert_eq!(
            snake_body,
            &[
                Point { x: 3, y: 2 },
                Point { x: 4, y: 2 },
                Point { x: 5, y: 2 }
            ]
        );
    }

    #[test]
    fn make_snake_longer() {
        let mut snake = Snake::new(3, Point { x: 0, y: 0 });

        snake.make_longer();
        snake.move_to_next_position();

        assert_eq!(snake.get_body().len(), 4);
        assert_eq!(
            snake.get_body(),
            &[
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
            ]
        );

        snake.change_direction(Direction::Down);
        snake.move_to_next_position();
        snake.make_longer();
        snake.move_to_next_position();

        assert_eq!(snake.get_body().len(), 5);
        assert_eq!(
            snake.get_body(),
            &[
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
                Point { x: 3, y: 1 },
                Point { x: 3, y: 2 },
            ]
        );
    }
}
