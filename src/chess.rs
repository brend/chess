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
}

#[derive(Copy, Clone, Debug)]
pub enum Square {
    Empty,
    Taken(Piece),
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

    pub fn print(&self) {
        for row in 0..8 {
            for col in 0..8 {
                let symbol = match &self.squares[row * 8 + col] {
                    Square::Empty => ".",
                    Square::Taken(piece) => match &piece.kind {
                        PieceKind::Bishop => "B",
                        PieceKind::Rook => "R",
                        PieceKind::Knight => "k",
                        PieceKind::King => "K",
                        PieceKind::Queen => "Q",
                        PieceKind::Pawn => "p",
                    },
                };
                print!("{symbol}");
            }
            println!();
        }
    }
}
