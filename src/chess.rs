use std::array;

use crate::pieces::*;

pub enum ChessColor {
    White,
    Black,
}

pub struct Coordinate {
    x: usize,
    y: usize,
}
impl Coordinate {
    pub fn tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    pub fn board_index(&self) -> (isize, isize) {
        (self.x as isize, self.y as isize)
    }
}

#[derive(Debug)]
pub enum BoardIndexError {
    NegativeIndex,
    LargeIndex,
}
// board[y][x]
pub type Square = Option<Box<dyn Piece>>;
pub struct Board([[Square; 8]; 8]);
impl Board {
    pub fn init() -> Board {
        // gonna have to implement the correct starting board at some point
        Board(array::from_fn(|_| array::from_fn(|_| None)))
    }

    pub fn at(&mut self, (x, y): (isize, isize)) -> Result<&mut Square, BoardIndexError> {
        if x < 0 || y < 0 {
            return Err(BoardIndexError::NegativeIndex);
        }
        if x >= 8 || y >= 8 {
            return Err(BoardIndexError::LargeIndex);
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
