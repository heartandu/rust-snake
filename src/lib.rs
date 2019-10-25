extern crate termion;
extern crate rand;

use rand::Rng;
use std::fmt;

pub trait CanMove {
    fn do_move(&mut self);
    fn set_velocity(&mut self, velocity: Velocity);
}

#[derive(Copy, Clone)]
pub struct Velocity {
    vel_x: i16,
    vel_y: i16,
}

impl Velocity {
    pub fn new(vel_x: i16, vel_y: i16) -> Velocity {
        Velocity { vel_x, vel_y }
    }

    pub fn is_same(&self, other: &Velocity) -> bool {
        self.vel_x == other.vel_x && self.vel_y == other.vel_y
    }
}

#[derive(Copy, Clone)]
pub struct Coordinates {
    x: u16,
    y: u16,
}

impl Coordinates {
    pub fn new(x: u16, y: u16) -> Coordinates {
        Coordinates { x, y }
    }

    pub fn is_same(&self, other: &Coordinates) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct Block {
    coordinates: Coordinates,
    velocity: Velocity,
    symbol: char,
}

impl Block {
    pub fn new(x: u16, y: u16, symbol: char) -> Block {
        Block {
            coordinates: Coordinates::new(x, y),
            velocity: Velocity::new(0, 0),
            symbol,
        }
    }

    pub fn new_random(max_x: u16, max_y: u16, symbol: char) -> Block {
        let mut rng = rand::thread_rng();
        Block::new(rng.gen_range(1, max_x), rng.gen_range(1, max_y), symbol)
    }
}

impl CanMove for Block {
    fn do_move(&mut self) {
        if self.velocity.vel_x.abs() > 0 {
            self.coordinates.x = self.coordinates.x.wrapping_add(self.velocity.vel_x as u16);
        }

        if self.velocity.vel_y.abs() > 0 {
            self.coordinates.y = self.coordinates.y.wrapping_add(self.velocity.vel_y as u16);
        }
    }

    fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            termion::cursor::Goto(self.coordinates.x, self.coordinates.y),
            self.symbol
        )
    }
}

pub struct Mice {
    block: Block,
}

impl Mice {
    pub fn new(max_x: u16, max_y: u16) -> Mice {
        Mice { block: Block::new_random(max_x, max_y, 'O') }
    }
}

impl fmt::Display for Mice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.block)
    }
}

pub struct Snake {
    head: Block,
    tail: Vec<Block>,
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Snake {
        Snake {
            head: Block::new(x, y, 'O'),
            tail: vec![
                Snake::new_tail_circle(x - 1, y),
                Snake::new_tail_circle(x - 2, y),
                Snake::new_tail_circle(x - 3, y),
            ],
        }
    }

    pub fn grow(&mut self) {
        let tail_coords = self.tail.iter().last().unwrap().coordinates;
        self.tail.push(Snake::new_tail_circle(tail_coords.x, tail_coords.y));
    }

    pub fn is_stuck(&self) -> bool {
        self.does_intersect_tail(&self.head.coordinates)
    }

    pub fn does_intersect_tail(&self, coordinates: &Coordinates) -> bool {
        match self.tail.iter().position(|piece| piece.coordinates.is_same(coordinates)) {
            Some(_) => true,
            None => false,
        }
    }

    fn new_tail_circle(x: u16, y: u16) -> Block {
        Block::new(x, y, 'o')
    }
}

impl CanMove for Snake {
    fn do_move(&mut self) {
        self.head.do_move();
        let mut previous_velocity = self.head.velocity;
        for piece in self.tail.iter_mut() {
            piece.do_move();
            if !previous_velocity.is_same(&piece.velocity) {
                let temp_velocity = piece.velocity;
                piece.set_velocity(previous_velocity);
                previous_velocity = temp_velocity;
            }
        }
    }

    fn set_velocity(&mut self, velocity: Velocity) {
        if self.head.velocity.vel_x == 0 && self.head.velocity.vel_y == 0 {
            self.head.set_velocity(velocity);
            self.tail.iter_mut().for_each(|piece| piece.set_velocity(velocity));
        }

        if self.head.velocity.vel_x != velocity.vel_x && self.head.velocity.vel_y != velocity.vel_y {
            self.head.set_velocity(velocity);
        }
    }
}

impl fmt::Display for Snake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = format!("{}", self.head);
        for piece in self.tail.iter() {
            result.push_str(format!("{}", piece).as_ref());
        }

        write!(f, "{}", result)
    }
}

pub struct Screen {
    min_x: u16,
    max_x: u16,
    min_y: u16,
    max_y: u16,
}

impl Screen {
    pub fn new(size_x: u16, size_y: u16) -> Screen {
        Screen {
            min_x: 1,
            min_y: 1,
            max_x: size_x,
            max_y: size_y,
        }
    }

    pub fn is_inbound(&self, item: &Coordinates) -> bool {
        self.is_x_inbound(item.x) && self.is_y_inbound(item.y)
    }

    pub fn is_x_inbound(&self, x: u16) -> bool {
        self.min_x <= x && x <= self.max_x
    }

    pub fn is_y_inbound(&self, y: u16) -> bool {
        self.min_y <= y && y <= self.max_y
    }
}

pub struct Game {
    screen: Screen,
    mice: Mice,
    snake: Snake,
}

impl Game {
    pub fn new(size_x: u16, size_y: u16) -> Game {
        Game {
            screen: Screen::new(size_x, size_y),
            mice: Mice::new(size_x, size_y),
            snake: Snake::new(size_x / 2, size_y / 2),
        }
    }

    pub fn calc_new_frame(&mut self) {
        self.snake.do_move();
        while self.snake.head.coordinates.is_same(&self.mice.block.coordinates) {
            self.snake.grow();
            self.mice = Mice::new(self.screen.max_x, self.screen.max_y);
            while self.snake.does_intersect_tail(&self.mice.block.coordinates) {
                self.mice = Mice::new(self.screen.max_x, self.screen.max_y);
            }
        }
    }

    pub fn set_snake_velocity(&mut self, velocity: Velocity) {
        self.snake.set_velocity(velocity);
    }

    pub fn is_game_over(&self) -> bool {
        !self.screen.is_inbound(&self.snake.head.coordinates) || self.snake.is_stuck()
    }

    pub fn get_game_over_message(&self) -> String {
        let message = "Game Over!";
        format!(
            "{}{}",
            termion::cursor::Goto(
                (self.screen.max_x - message.len() as u16) / 2,
                self.screen.max_y / 2
            ),
            message
        )
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.mice, self.snake)
    }
}
