pub type Position = (usize, usize);

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
}

impl Piece {
    fn white(kind: PieceKind) -> Piece {
        Piece {
            kind,
            color: Color::White,
        }
    }

    fn black(kind: PieceKind) -> Piece {
        Piece {
            kind,
            color: Color::Black,
        }
    }

    pub fn ascii_symbol(self) -> &'static str {
        self.kind.ascii_symbol()
    }

    pub fn unicode_symbol(self) -> &'static str {
        match (self.color, self.kind) {
            (Color::White, PieceKind::King) => "♔",
            (Color::White, PieceKind::Queen) => "♕",
            (Color::White, PieceKind::Rook) => "♖",
            (Color::White, PieceKind::Bishop) => "♗",
            (Color::White, PieceKind::Knight) => "♘",
            (Color::White, PieceKind::Pawn) => "♙",
            (Color::Black, PieceKind::King) => "♚",
            (Color::Black, PieceKind::Queen) => "♛",
            (Color::Black, PieceKind::Rook) => "♜",
            (Color::Black, PieceKind::Bishop) => "♝",
            (Color::Black, PieceKind::Knight) => "♞",
            (Color::Black, PieceKind::Pawn) => "♟",
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

    fn index(position: Position) -> usize {
        position.0 * 8 + position.1
    }

    pub fn square(&self, position: Position) -> Square {
        self.squares[Self::index(position)]
    }

    pub fn move_piece(&mut self, from: Position, to: Position) {
        if from == to {
            return;
        }

        let from_index = Self::index(from);
        let to_index = Self::index(to);
        self.squares[to_index] = self.squares[from_index];
        self.squares[from_index] = Square::Empty;
    }

}

impl Default for Board {
    fn default() -> Board {
        Board::new()
    }
}
