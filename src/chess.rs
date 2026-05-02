use std::cmp;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Position {
        if row >= 8 || col >= 8 {
            panic!("Position out of bounds");
        }
        Position { row, col }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn zero() -> Position {
        Position::new(0, 0)
    }

    fn distances(&self, other: &Position) -> (usize, usize) {
        (
            self.col().abs_diff(other.col()),
            self.row().abs_diff(other.row()),
        )
    }

    pub fn is_adjacent(&self, other: &Position) -> bool {
        let d = self.distances(other);
        cmp::max(d.0, d.1) == 1
    }

    pub fn is_on_diagonal(&self, other: &Position) -> bool {
        let d = self.distances(other);
        self != other && d.0 == d.1
    }

    pub fn is_on_main_axis(&self, other: &Position) -> bool {
        let d = self.distances(other);
        self != other && (d.0 == 0 || d.1 == 0)
    }

    pub fn is_ell_away(&self, other: &Position) -> bool {
        let d = self.distances(other);
        d.0 == 2 && d.1 == 1 || d.0 == 1 && d.1 == 2
    }

    pub fn is_one_up(&self, from: &Position) -> bool {
        self.row == from.row + 1
    }

    pub fn is_two_up(&self, from: &Position) -> bool {
        self.row == from.row + 2
    }
    pub fn is_one_down(&self, from: &Position) -> bool {
        self.row + 1 == from.row
    }

    pub fn is_two_down(&self, from: &Position) -> bool {
        self.row + 2 == from.row
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl PieceKind {
    pub fn ascii_symbol(self) -> &'static str {
        match self {
            PieceKind::King => "K",
            PieceKind::Queen => "Q",
            PieceKind::Rook => "R",
            PieceKind::Bishop => "B",
            PieceKind::Knight => "N",
            PieceKind::Pawn => "P",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    color: Color,
    kind: PieceKind,
    move_count: usize,
}

impl Piece {
    fn white(kind: PieceKind) -> Piece {
        Piece {
            kind,
            color: Color::White,
            move_count: 0,
        }
    }

    fn black(kind: PieceKind) -> Piece {
        Piece {
            kind,
            color: Color::Black,
            move_count: 0,
        }
    }

    pub fn increase_move_count(&self) -> Piece {
        Piece {
            kind: self.kind,
            color: self.color,
            move_count: self.move_count + 1,
        }
    }

    pub fn ascii_symbol(self) -> &'static str {
        self.kind.ascii_symbol()
    }

    pub fn unicode_symbol(self) -> &'static str {
        match self.kind {
            PieceKind::King => "♚",
            PieceKind::Queen => "♛",
            PieceKind::Rook => "♜",
            PieceKind::Bishop => "♝",
            PieceKind::Knight => "♞",
            PieceKind::Pawn => "♟",
        }
    }

    pub fn display_symbol(self, use_unicode: bool) -> &'static str {
        if use_unicode {
            self.unicode_symbol()
        } else {
            self.ascii_symbol()
        }
    }

    pub fn color(self) -> Color {
        self.color
    }

    pub fn legal_move(&self, from: &Position, to: &Position) -> bool {
        match self.kind {
            PieceKind::King => to.is_on_diagonal(from) && to.is_adjacent(from), // TODO: castling
            PieceKind::Queen => to.is_on_diagonal(from) || to.is_on_main_axis(from),
            PieceKind::Rook => to.is_on_main_axis(from),
            PieceKind::Bishop => to.is_on_diagonal(from),
            PieceKind::Knight => to.is_ell_away(from),
            PieceKind::Pawn => {
                match self.color {
                    Color::White => {
                        to.is_one_up(from) || self.move_count == 0 && to.is_two_up(from)
                    }
                    Color::Black => {
                        to.is_one_down(from) || self.move_count == 0 && to.is_two_down(from)
                    }
                }

                //                to.is_one_ahead(from) || piece.move_count == 0 && to.is_two_ahead(from)
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Square {
    Empty,
    Taken(Piece),
}

impl Square {
    pub fn piece(self) -> Option<Piece> {
        match self {
            Square::Empty => None,
            Square::Taken(piece) => Some(piece),
        }
    }

    pub fn increase_move_count(&self) -> Square {
        match self {
            Square::Empty => *self,
            Square::Taken(piece) => Square::Taken(piece.increase_move_count()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Board {
    squares: [Square; 64],
}

impl Board {
    pub fn new() -> Board {
        let mut squares = [Square::Empty; 64];

        // row 1
        squares[0] = Square::Taken(Piece::white(PieceKind::Rook));
        squares[1] = Square::Taken(Piece::white(PieceKind::Knight));
        squares[2] = Square::Taken(Piece::white(PieceKind::Bishop));
        squares[3] = Square::Taken(Piece::white(PieceKind::Queen));
        squares[4] = Square::Taken(Piece::white(PieceKind::King));
        squares[5] = Square::Taken(Piece::white(PieceKind::Bishop));
        squares[6] = Square::Taken(Piece::white(PieceKind::Knight));
        squares[7] = Square::Taken(Piece::white(PieceKind::Rook));
        // row 2
        for i in 0..8 {
            squares[8 + i] = Square::Taken(Piece::white(PieceKind::Pawn));
        }
        // row 7
        for i in 0..8 {
            squares[8 * 6 + i] = Square::Taken(Piece::black(PieceKind::Pawn));
        }
        // row 8
        squares[56] = Square::Taken(Piece::black(PieceKind::Rook));
        squares[57] = Square::Taken(Piece::black(PieceKind::Knight));
        squares[58] = Square::Taken(Piece::black(PieceKind::Bishop));
        squares[59] = Square::Taken(Piece::black(PieceKind::Queen));
        squares[60] = Square::Taken(Piece::black(PieceKind::King));
        squares[61] = Square::Taken(Piece::black(PieceKind::Bishop));
        squares[62] = Square::Taken(Piece::black(PieceKind::Knight));
        squares[63] = Square::Taken(Piece::black(PieceKind::Rook));

        Board { squares }
    }

    fn index(position: &Position) -> usize {
        position.row() * 8 + position.col()
    }

    pub fn square(&self, position: &Position) -> Square {
        self.squares[Self::index(position)]
    }

    pub fn move_piece(&mut self, from: &Position, to: &Position) {
        if from == to {
            return;
        }

        let from_index = Self::index(from);
        let to_index = Self::index(to);
        self.squares[to_index] = self.squares[from_index].increase_move_count();
        self.squares[from_index] = Square::Empty;
    }
}

impl Default for Board {
    fn default() -> Board {
        Board::new()
    }
}

#[derive(Debug)]
pub enum MoveError {
    EmptyMove,
    EmptyFromSquare,
    IllegalMoveForPiece,
}

#[derive(Default, Debug)]
pub struct GameState {
    pub board: Board,
    move_index: u32,
}

impl GameState {
    pub fn move_piece(&mut self, from: &Position, to: &Position) -> Result<(), MoveError> {
        self.validate_move(from, to)?;
        self.board.move_piece(from, to);
        self.move_index += 1;
        Ok(())
    }

    fn validate_move(&self, from: &Position, to: &Position) -> Result<(), MoveError> {
        if from == to {
            return Err(MoveError::EmptyMove);
        }

        match self.board.square(from) {
            Square::Taken(piece) => self.validate_move_for_piece(piece, from, to),
            Square::Empty => Err(MoveError::EmptyFromSquare),
        }
    }

    fn validate_move_for_piece(
        &self,
        piece: Piece,
        from: &Position,
        to: &Position,
    ) -> Result<(), MoveError> {
        if piece.legal_move(from, to) {
            Ok(())
        } else {
            Err(MoveError::IllegalMoveForPiece)
        }
    }
}
