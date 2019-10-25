extern crate termion;
extern crate rand;

use rand::Rng;
use std::fmt;

const SCORE_PER_MICE: u32 = 100;

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

    pub fn new_random(min_x: u16, max_x: u16, min_y: u16, max_y: u16, symbol: char) -> Block {
        let mut rng = rand::thread_rng();
        Block::new(rng.gen_range(min_x, max_x), rng.gen_range(min_y, max_y), symbol)
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
    pub fn new(min_x: u16, max_x: u16, min_y: u16, max_y: u16) -> Mice {
        Mice { block: Block::new_random(min_x, max_x, min_y, max_y, 'O') }
    }

    pub fn new_in_screen(screen: &Screen) -> Mice {
        Mice::new(screen.min_x, screen.max_x, screen.min_y, screen.max_y)
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
        let mut snake = Snake {
            head: Block::new(x, y, 'O'),
            tail: vec![],
        };

        for i in 1..4 {
            snake.tail.push(Snake::new_tail_circle(x - i, y));
        }

        snake
    }

    pub fn new_in_screen(screen: &Screen) -> Snake {
        Snake::new((screen.max_x - screen.min_x) / 2, (screen.max_y - screen.min_y) / 2)
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

pub struct Score {
    score: u32,
    coordinates: Coordinates,
}

impl Score {
    pub fn new(position_x: u16, position_y: u16) -> Score {
        Score { score: 0 , coordinates: Coordinates::new(position_x, position_y) }
    }

    pub fn new_in_screen(screen: &Screen) -> Score {
        Score::new(screen.limit_x / 2, screen.min_y - 1)
    }

    pub fn inc(&mut self) {
        self.score += SCORE_PER_MICE;
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let score = format!("Score: {}", self.score);
        write!(
            f,
            "{}{}",
            termion::cursor::Goto(self.coordinates.x - score.len() as u16 / 2, self.coordinates.y),
            score
        )
    }
}

pub struct Screen {
    min_x: u16,
    max_x: u16,
    min_y: u16,
    max_y: u16,
    limit_x: u16,
    limit_y: u16,
}

impl Screen {
    pub fn new(size_x: u16, size_y: u16) -> Screen {
        Screen {
            min_x: 2,
            min_y: 2,
            max_x: size_x - 1,
            max_y: size_y - 1,
            limit_x: size_x,
            limit_y: size_y,
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

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dash_line = "—".repeat((self.max_x - self.min_x + 1) as usize);
        write!(f, "{}{}", termion::cursor::Goto(self.min_x, self.min_y - 1), dash_line)?;
        write!(f, "{}{}", termion::cursor::Goto(self.min_x, self.max_y + 1), dash_line)?;
        for i in self.min_y..(self.max_y + 1) {
            write!(
                f,
                "{}{}{}{}",
                termion::cursor::Goto(self.min_x - 1, i),
                '|',
                termion::cursor::Goto(self.max_x + 1, i),
                '|'
            )?;
        }
        Ok(())
    }
}

pub struct Game {
    screen: Screen,
    mice: Mice,
    snake: Snake,
    score: Score,
    is_paused: bool,
}

impl Game {
    pub fn new(size_x: u16, size_y: u16) -> Game {
        let screen = Screen::new(size_x, size_y);
        let mice = Mice::new_in_screen(&screen);
        let snake = Snake::new_in_screen(&screen);
        let score = Score::new_in_screen(&screen);
        Game {
            screen,
            mice,
            snake,
            score,
            is_paused: false,
        }
    }

    fn make_mice(&mut self) {
        self.mice = Mice::new_in_screen(&self.screen);
    }

    pub fn calc_new_frame(&mut self) {
        if self.is_paused {
            return;
        }

        self.snake.do_move();
        while self.snake.head.coordinates.is_same(&self.mice.block.coordinates) {
            self.score.inc();
            self.snake.grow();
            self.make_mice();
            while self.snake.does_intersect_tail(&self.mice.block.coordinates) {
                self.make_mice()
            }
        }
    }

    pub fn set_snake_velocity(&mut self, velocity: Velocity) {
        if self.is_paused {
            return;
        }

        self.snake.set_velocity(velocity);
    }

    pub fn switch_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    pub fn is_game_over(&self) -> bool {
        !self.screen.is_inbound(&self.snake.head.coordinates) || self.snake.is_stuck()
    }

    pub fn get_game_over_message(&self) -> String {
        self.format_message(format!("Game Over! Your score is {}", self.score.score))
    }

    pub fn get_pause_message(&self) -> String {
        self.format_message("Paused".to_string())
    }

    fn format_message(&self, message: String) -> String {
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
        write!(
            f,
            "{}{}{}{}",
            self.mice,
            self.snake,
            self.screen,
            self.score
        )?;

        if self.is_paused {
            write!(f, "{}", self.get_pause_message())?;
        }

        Ok(())
    }
}
