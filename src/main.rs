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

    fn is_eating_snack(&self, snack_position: &Point) -> bool {
        self.body.last().unwrap() == snack_position
    }

    fn is_eating_itself(&self) -> bool {
        let head = self.body.last().unwrap();
        let all_but_head = self.body.split_last().unwrap().1;
        for body_part in all_but_head.iter() {
            if head == body_part {
                return true;
            }
        }
        return false;
    }

    fn collides_with_point(&self, point: &Point) -> bool {
        for body_part in self.body.iter() {
            if body_part == point {
                return true;
            }
        }
        return false;
    }

    fn collides_with_bounds(&self, snake_pit: &SnakePit) -> bool {
        let head = self.body.last().unwrap();
        head.x <= 0 || head.y <= 0 || head.x >= snake_pit.width || head.y >= snake_pit.height
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

#[derive(Debug, PartialEq)]
enum GameStatus {
    ContinueAtLevel(i32),
    Finished,
}

impl SnakeEngine {
    fn new(snake_pit_height: u32, snake_pit_width: u32) -> SnakeEngine {
        Self::new_with_snake_length(snake_pit_height, snake_pit_width, 3)
    }

    fn new_with_snake_length(
        snake_pit_height: u32,
        snake_pit_width: u32,
        snake_length: u32,
    ) -> SnakeEngine {
        let snake_pit = SnakePit {
            height: snake_pit_height,
            width: snake_pit_width,
        };
        let snake = Snake::new(snake_length, Point { x: 2, y: 2 });
        if let Some(snack_position) = Self::generate_snack(&snake_pit, &snake) {
            SnakeEngine {
                snake,
                snake_pit,
                snack_position,
            }
        } else {
            panic!("Not able to create SnakeEngine, impossible to generate snack!");
        }
    }

    fn change_snake_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction);
    }

    fn tick(&mut self) -> GameStatus {
        self.snake.move_to_next_position();
        if self.snake.collides_with_bounds(&self.snake_pit) || self.snake.is_eating_itself() {
            return GameStatus::Finished;
        }
        if self.snake.is_eating_snack(&self.snack_position) {
            self.snake.make_longer();
            if let Some(snack_position) = Self::generate_snack(&self.snake_pit, &self.snake) {
                self.snack_position = snack_position;
            }
        }
        return GameStatus::ContinueAtLevel(0);
    }

    fn generate_snack(snake_pit: &SnakePit, snake: &Snake) -> Option<Point> {
        let mut possible_snacks = vec![];
        for x in 1..snake_pit.width {
            for y in 1..snake_pit.height {
                let possible_position = Point { x, y };
                if !snake.collides_with_point(&possible_position) {
                    possible_snacks.push(possible_position);
                }
            }
        }
        if possible_snacks.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            let choice = rng.gen_range(0..possible_snacks.len());
            Some(possible_snacks.swap_remove(choice))
        }
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

fn display_snack(snack_position: &Point) {
    let mut stdout = stdout();
    stdout.execute(cursor::MoveTo(
        snack_position.x as u16,
        snack_position.y as u16,
    ));
    stdout.execute(Print("#"));
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
    let mut snake_engine = SnakeEngine::new(20, 30);

    loop {
        clear_display();
        display_snake_pit(&snake_engine.snake_pit);
        display_snack(&snake_engine.snack_position);
        display_snake(&snake_engine.snake.body);
        let event = wait_for_latest_event(1000);
        match event {
            Some(event::KeyCode::Up) => snake_engine.change_snake_direction(Direction::Up),
            Some(event::KeyCode::Left) => snake_engine.change_snake_direction(Direction::Left),
            Some(event::KeyCode::Right) => snake_engine.change_snake_direction(Direction::Right),
            Some(event::KeyCode::Down) => snake_engine.change_snake_direction(Direction::Down),
            _ => (),
        }
        match snake_engine.tick() {
            GameStatus::Finished => {
                clear_display();
                println!("The game has finished!");
                return;
            }
            _ => (),
        }
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

    #[test]
    fn snake_collides_with_point() {
        let mut snake = Snake::new(3, Point { x: 3, y: 3 });
        assert_eq!(snake.collides_with_point(&Point { x: 3, y: 3 }), true);
        assert_eq!(snake.collides_with_point(&Point { x: 4, y: 3 }), true);
        assert_eq!(snake.collides_with_point(&Point { x: 5, y: 3 }), true);
        assert_eq!(snake.collides_with_point(&Point { x: 6, y: 3 }), false);
    }

    #[test]
    fn snake_collides_with_pit() {
        let mut snake = Snake::new(3, Point { x: 3, y: 3 });
        assert_eq!(
            snake.collides_with_bounds(&SnakePit {
                width: 5,
                height: 10
            }),
            true
        );

        let mut snake = Snake::new(3, Point { x: 3, y: 3 });
        assert_eq!(
            snake.collides_with_bounds(&SnakePit {
                width: 10,
                height: 3
            }),
            true
        );

        let mut snake = Snake::new(1, Point { x: 0, y: 3 });
        assert_eq!(
            snake.collides_with_bounds(&SnakePit {
                width: 5,
                height: 10
            }),
            true
        );

        let mut snake = Snake::new(3, Point { x: 3, y: 0 });
        assert_eq!(
            snake.collides_with_bounds(&SnakePit {
                width: 5,
                height: 10
            }),
            true
        );
    }

    #[test]
    fn game_finishes_when_snake_hits_wall() {
        let mut engine = SnakeEngine::new(3, 5);
        assert_eq!(engine.tick(), GameStatus::Finished);

        let mut engine = SnakeEngine::new(3, 6);
        engine.change_snake_direction(Direction::Down);
        assert_eq!(engine.tick(), GameStatus::Finished);

        let mut engine = SnakeEngine::new(3, 6);
        engine.change_snake_direction(Direction::Up);
        assert_eq!(engine.tick(), GameStatus::ContinueAtLevel(0));
        assert_eq!(engine.tick(), GameStatus::Finished);

        let mut engine = SnakeEngine::new(3, 6);
        engine.change_snake_direction(Direction::Up);
        assert_eq!(engine.tick(), GameStatus::ContinueAtLevel(0));
        engine.change_snake_direction(Direction::Left);
        assert_eq!(engine.tick(), GameStatus::ContinueAtLevel(0));
        assert_eq!(engine.tick(), GameStatus::ContinueAtLevel(0));
        assert_eq!(engine.tick(), GameStatus::ContinueAtLevel(0));
    }

    #[test]
    fn snake_eats_itself() {
        let mut snake = Snake::new(5, Point { x: 3, y: 3 });
        assert_eq!(snake.is_eating_itself(), false);
        snake.change_direction(Direction::Down);
        snake.move_to_next_position();
        assert_eq!(snake.is_eating_itself(), false);
        snake.change_direction(Direction::Left);
        snake.move_to_next_position();
        assert_eq!(snake.is_eating_itself(), false);
        snake.change_direction(Direction::Up);
        snake.move_to_next_position();
        assert_eq!(snake.is_eating_itself(), true);
    }

    #[test]
    fn game_finishes_when_snake_eats_itself() {
        let mut engine = SnakeEngine::new_with_snake_length(20, 20, 5);
        engine.change_snake_direction(Direction::Down);
        assert_eq!(engine.tick(), GameStatus::ContinueAtLevel(0));
        engine.change_snake_direction(Direction::Left);
        assert_eq!(engine.tick(), GameStatus::ContinueAtLevel(0));
        engine.change_snake_direction(Direction::Up);
        assert_eq!(engine.tick(), GameStatus::Finished);
    }
}
