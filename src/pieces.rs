use crate::chess::*;

enum PieceMoveError {
    IndexError(BoardIndexError),
}
pub trait Piece {
    fn new(color: ChessColor, pos: Coordinate) -> Self
    where
        Self: Sized;
    fn get_color(&self) -> &ChessColor;
    fn get_moves(&self, board: &Board) -> Vec<Coordinate>;
    fn move_piece(&mut self, board: &mut Board, pos: Coordinate) -> Result<Square, PieceMoveError>;
}

struct Pawn {
    color: ChessColor,
    pos: Coordinate,
    has_moved: bool,
}

impl Piece for Pawn {
    fn new(color: ChessColor, pos: Coordinate) -> Self
    where
        Self: Sized,
    {
        Pawn {
            color,
            pos,
            has_moved: false,
        }
    }

    fn get_color(&self) -> &ChessColor {
        &self.color
    }
    fn get_moves(&self, board: &Board) -> Vec<Coordinate> {
        vec![]
    }
    fn move_piece(&mut self, board: &mut Board, pos: Coordinate) -> Result<Square, PieceMoveError> {
        let square = match board.at(pos.board_index()) {
            Ok(i) => i,
            Err(e) => return Err(PieceMoveError::IndexError(e)),
        };
        self.pos = pos;
        square = self;
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn idfk() {
        assert!(false)
    }
}
