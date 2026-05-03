use std::{cmp, fmt};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    rank: usize,
    file: usize,
}

impl Position {
    pub fn new(rank: usize, file: usize) -> Position {
        if rank >= 8 || file >= 8 {
            panic!("Position out of bounds");
        }
        Position { rank, file }
    }

    pub fn path(&self, to: &Position) -> Vec<Position> {
        let mut path = vec![];
        let dx = (to.file as isize - self.file as isize).signum();
        let dy = (to.rank as isize - self.rank as isize).signum();
        let mut position = *self;

        while &position != to {
            path.push(position);
            position = Position::new(
                position.rank.strict_add_signed(dy),
                position.file.strict_add_signed(dx),
            );
        }
        path.push(position);
        path
    }

    pub fn row(&self) -> usize {
        self.rank
    }

    pub fn col(&self) -> usize {
        self.file
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
        self.rank == from.rank + 1
    }

    pub fn is_two_up(&self, from: &Position) -> bool {
        self.rank == from.rank + 2
    }
    pub fn is_one_down(&self, from: &Position) -> bool {
        self.rank + 1 == from.rank
    }

    pub fn is_two_down(&self, from: &Position) -> bool {
        self.rank + 2 == from.rank
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
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
            PieceKind::King => to.is_on_diagonal(from) && to.is_adjacent(from),
            PieceKind::Queen => to.is_on_diagonal(from) || to.is_on_main_axis(from),
            PieceKind::Rook => to.is_on_main_axis(from),
            PieceKind::Bishop => to.is_on_diagonal(from),
            PieceKind::Knight => to.is_ell_away(from),
            PieceKind::Pawn => match self.color {
                Color::White => to.is_one_up(from) || self.move_count == 0 && to.is_two_up(from),
                Color::Black => {
                    to.is_one_down(from) || self.move_count == 0 && to.is_two_down(from)
                }
            },
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
    WrongPlayer,
    PathBlocked,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveError::EmptyMove => write!(f, "Choose a different destination square."),
            MoveError::EmptyFromSquare => write!(f, "Choose a square with a piece to move."),
            MoveError::IllegalMoveForPiece => write!(f, "That piece cannot move there."),
            MoveError::WrongPlayer => write!(f, "It is not that piece's turn."),
            MoveError::PathBlocked => write!(f, "Another piece is blocking that move."),
        }
    }
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
            Square::Taken(piece) => {
                self.validate_moving_player(&piece)?;
                self.validate_move_for_piece(piece, from, to)?;
                self.validate_move_path(from, to)?;
            }
            Square::Empty => return Err(MoveError::EmptyFromSquare),
        }

        Ok(())
    }

    fn validate_move_path(&self, from: &Position, to: &Position) -> Result<(), MoveError> {
        let path = from.path(to);
        let limit = path.len() - 1;

        for i in 1..limit {
            let position = &path[i];
            let Square::Empty = self.board.square(position) else {
                return Err(MoveError::PathBlocked);
            };
        }

        Ok(())
    }

    fn current_player(&self) -> Color {
        if self.move_index.is_multiple_of(2) {
            Color::White
        } else {
            Color::Black
        }
    }

    fn validate_moving_player(&self, piece: &Piece) -> Result<(), MoveError> {
        if piece.color() == self.current_player() {
            Ok(())
        } else {
            Err(MoveError::WrongPlayer)
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
