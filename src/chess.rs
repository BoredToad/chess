use std::array;

use crate::pieces::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ChessColor {
    White,
    Black,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i8,
    pub y: i8,
}

impl Coordinate {
    pub fn tuple(&self) -> (i8, i8) {
        (self.x, self.y)
    }
    pub fn in_bounds(&self) -> bool {
        if self.x > BOARD_SIZE as i8 || self.x < 0 || self.y > BOARD_SIZE as i8 || self.y < 0 {
            false
        } else {
            true
        }
    }
}

// honestly should have been a macro lol
pub fn cord(x: i8, y: i8) -> Coordinate {
    Coordinate { x, y }
}

#[derive(Debug)]
pub enum BoardIndexError {
    NegativeIndex,
    LargeIndex,
}

pub const BOARD_SIZE: usize = 8;
// board[y][x]
pub type Square = Option<Box<dyn Piece>>;
// TODO: make the array field private so I can abstract shidd away later
pub struct Board(pub [[Square; BOARD_SIZE]; BOARD_SIZE]);
impl Board {
    pub fn init() -> Board {
        // gonna have to implement the correct starting board at some point
        Board(array::from_fn(|_| array::from_fn(|_| None)))
    }

    // NOTE: maybe unneeded
    pub fn immutable_at(&self, (x, y): (i8, i8)) -> Result<&Square, &str> {
        if !cord(x, y).in_bounds() {
            return Err("out of bounds");
        }
        Ok(&self.0[x as usize][y as usize])
    }

    pub fn at(&mut self, (x, y): (i8, i8)) -> Result<&mut Square, &str> {
        if !cord(x, y).in_bounds() {
            return Err("out of bounds");
        }
        Ok(&mut self.0[x as usize][y as usize])
    }
}

pub struct ChessGame {
    board: Board,
}

impl ChessGame {
    pub fn init() -> ChessGame {
        ChessGame {
            board: Board::init(),
        }
    }
}
