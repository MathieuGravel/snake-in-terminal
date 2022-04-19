pub mod apple;
pub mod border;
pub mod game_board;
pub mod game_over;
pub mod score;
pub mod snake;
pub mod timer;

use std::ops::Add;

use accessors_rs::Accessors;
use error_chain::error_chain;
use rand::Rng;

error_chain! {
    errors {
        CannotLock
    }

    links {
        Screen(snake_in_terminal::terminus::screen::Error, snake_in_terminal::terminus::screen::ErrorKind);
    }

    foreign_links {
        Io(std::io::Error);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<(u16, u16)> for Position {
    fn from((x, y): (u16, u16)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Dimension {
    pub width: u16,
    pub height: u16,
}

impl Dimension {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn get_random_position_inside(&self) -> Position {
        let mut rng = rand::thread_rng();
        Position::new(rng.gen_range(0..self.width), rng.gen_range(0..self.height))
    }
}

impl From<(u16, u16)> for Dimension {
    fn from((width, height): (u16, u16)) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Accessors, Clone, Copy)]
#[accessors(get_copy, get_mut, set)]
pub struct Boundary {
    position: Position,
    dimension: Dimension,
}

impl Boundary {
    pub fn new(position: Position, dimension: Dimension) -> Self {
        Self {
            position,
            dimension,
        }
    }

    pub fn is_inside(&self, position: Position) -> bool {
        position.x >= self.left()
            && position.x <= self.right()
            && position.y >= self.top()
            && position.y <= self.bottom()
    }

    pub fn top(&self) -> u16 {
        self.position.y
    }

    pub fn bottom(&self) -> u16 {
        self.position.y + self.dimension.height - 1
    }

    pub fn left(&self) -> u16 {
        self.position.x
    }

    pub fn right(&self) -> u16 {
        self.position.x + self.dimension.width - 1
    }
}
