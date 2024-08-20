use crate::chess::*;

enum MovePieceError {
    NotPossible,
    OutOfBounds,
}

pub trait Piece
where
    Self: Sized,
{
    fn new(color: ChessColor, pos: Coordinate) -> Self;
    fn get_color(&self) -> &ChessColor;
    fn get_pos(&self) -> &Coordinate;
    fn get_moves(&self, board: &Board) -> Vec<Coordinate>;

    fn move_piece(self, board: &mut Board, pos: Coordinate) -> Result<(), MovePieceError> {
        if !self.get_moves(board).iter().any(|i| *i == pos) {
            return Err(MovePieceError::NotPossible);
        }

        let origin = self.get_pos();
        let mut square = if let Ok(i) = board.at(pos.tuple()) {
            i
        } else {
            return Err(MovePieceError::OutOfBounds);
        };
        square = &mut Some(Box::new(self));

        Ok(())
    }
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

    fn get_pos(&self) -> &Coordinate {
        &self.pos
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

            let square = match board.immutable_at(target.tuple()) {
                Err(_) => continue,
                Ok(square) => square,
            };
            let piece = match square {
                None => continue,
                Some(piece) => piece,
            };
            if self.color == *piece.get_color() {
                continue;
            }

            possible.push(target)
        }

        // en passant is for later lol, couldn't be fucked

        possible
    }
}

struct King {
    color: ChessColor,
    pos: Coordinate,
}
impl Piece for King {
    fn new(color: ChessColor, pos: Coordinate) -> Self {
        todo!()
    }
    fn get_pos(&self) -> &Coordinate {
        todo!()
    }
    fn get_color(&self) -> &ChessColor {
        todo!()
    }
    fn get_moves(&self, board: &Board) -> Vec<Coordinate> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
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
