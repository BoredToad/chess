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

// remember to add promotion some day
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
        let mut possible = vec![];
        let direction = match self.color {
            ChessColor::White => 1,
            ChessColor::Black => -1,
        };

        let first = cord(self.pos.x, self.pos.y + direction);
        if first.in_bounds() {
            possible.push(first);
            let second = cord(self.pos.x, self.pos.y + direction * 2);
            if second.in_bounds() {
                possible.push(second)
            }
        // this check skips the useless checks for capturing pieces out of bounds
        } else {
            return possible;
        }

        // capturing
        for side in [-1, 1] {
            let target = cord(self.pos.x + direction, self.pos.y + side);

            match board.immutable_at(target.tuple()) {
                Err(_) => continue,
                Ok(square) => match square {
                    None => continue,
                    Some(piece) => {
                        if self.color == *piece.get_color() {
                            continue;
                        }
                    }
                },
            }
            possible.push(target)
        }

        possible
    }

    fn move_piece(&mut self, board: &mut Board, pos: Coordinate) -> Result<Square, PieceMoveError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::array;

    use super::*;

    #[test]
    fn pawn_unmoved_no_obstacles() {
        let mut board = Board::init();
        board.0[3][3] = Some(Box::new(Pawn::new(ChessColor::White, cord(3, 3))));
        board.0[4][4] = Some(Box::new(Pawn::new(ChessColor::Black, cord(4, 4))));
        let pawn = match &board.0[3][3] {
            Some(i) => i,
            _ => panic!("Pawn is None"),
        };
        assert_eq!(
            pawn.get_moves(&board),
            vec![cord(3, 4), cord(3, 5), cord(4, 4)]
        );
    }
}
