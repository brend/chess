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
    InvalidDestination,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveError::EmptyMove => write!(f, "Choose a different destination square."),
            MoveError::EmptyFromSquare => write!(f, "Choose a square with a piece to move."),
            MoveError::IllegalMoveForPiece => write!(f, "That piece cannot move there."),
            MoveError::WrongPlayer => write!(f, "It is not that piece's turn."),
            MoveError::PathBlocked => write!(f, "Another piece is blocking that move."),
            MoveError::InvalidDestination => write!(f, "That piece cannot move there."),
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
                self.validate_move_path(piece, from, to)?;
            }
            Square::Empty => return Err(MoveError::EmptyFromSquare),
        }

        Ok(())
    }

    fn validate_move_path(
        &self,
        piece: Piece,
        from: &Position,
        to: &Position,
    ) -> Result<(), MoveError> {
        let path = from.path(to);

        if path.is_empty() {
            return Ok(());
        }

        let limit = path.len() - 1;

        for position in path.iter().take(limit).skip(1) {
            let Square::Empty = self.board.square(position) else {
                return Err(MoveError::PathBlocked);
            };
        }

        if let Square::Taken(other_piece) = self.board.square(path.last().unwrap())
            && piece.color() == other_piece.color()
        {
            return Err(MoveError::InvalidDestination);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(rank: usize, file: usize) -> Position {
        Position::new(rank, file)
    }

    fn empty_board() -> Board {
        Board {
            squares: [Square::Empty; 64],
        }
    }

    fn game_with(board: Board, move_index: u32) -> GameState {
        GameState { board, move_index }
    }

    fn put(board: &mut Board, position: Position, piece: Piece) {
        board.squares[Board::index(&position)] = Square::Taken(piece);
    }

    fn empty_game_with(piece: Piece, position: Position) -> GameState {
        let mut board = empty_board();
        put(&mut board, position, piece);
        game_with(
            board,
            match piece.color() {
                Color::White => 0,
                Color::Black => 1,
            },
        )
    }

    fn assert_move_ok(game: &mut GameState, from: Position, to: Position) {
        assert!(
            game.move_piece(&from, &to).is_ok(),
            "expected move from {from:?} to {to:?} to be legal"
        );
    }

    fn assert_move_err(game: &mut GameState, from: Position, to: Position) {
        assert!(
            game.move_piece(&from, &to).is_err(),
            "expected move from {from:?} to {to:?} to be illegal"
        );
    }

    #[test]
    fn white_moves_first_and_turns_alternate() {
        let mut game = GameState::default();

        assert_move_err(&mut game, pos(6, 0), pos(5, 0));
        assert_move_ok(&mut game, pos(1, 0), pos(2, 0));
        assert_move_err(&mut game, pos(1, 1), pos(2, 1));
        assert_move_ok(&mut game, pos(6, 0), pos(5, 0));
    }

    #[test]
    fn rejects_empty_source_square_and_empty_move() {
        let mut game = GameState::default();

        assert_move_err(&mut game, pos(3, 3), pos(4, 3));
        assert_move_err(&mut game, pos(1, 0), pos(1, 0));
    }

    #[test]
    fn sliding_pieces_move_on_clear_files_ranks_and_diagonals() {
        let mut game = empty_game_with(Piece::white(PieceKind::Queen), pos(3, 3));
        assert_move_ok(&mut game, pos(3, 3), pos(3, 7));

        let mut game = empty_game_with(Piece::white(PieceKind::Rook), pos(3, 3));
        assert_move_ok(&mut game, pos(3, 3), pos(7, 3));

        let mut game = empty_game_with(Piece::white(PieceKind::Bishop), pos(3, 3));
        assert_move_ok(&mut game, pos(3, 3), pos(6, 6));
    }

    #[test]
    fn sliding_pieces_cannot_jump_over_other_pieces() {
        let mut board = empty_board();
        put(&mut board, pos(0, 0), Piece::white(PieceKind::Rook));
        put(&mut board, pos(0, 3), Piece::black(PieceKind::Bishop));

        let mut game = game_with(board, 0);
        assert_move_err(&mut game, pos(0, 0), pos(0, 7));
    }

    #[test]
    fn pieces_can_capture_enemies_but_not_friendly_pieces() {
        let mut board = empty_board();
        put(&mut board, pos(0, 0), Piece::white(PieceKind::Rook));
        put(&mut board, pos(0, 4), Piece::black(PieceKind::Bishop));

        let mut game = game_with(board, 0);
        assert_move_ok(&mut game, pos(0, 0), pos(0, 4));

        let mut board = empty_board();
        put(&mut board, pos(0, 0), Piece::white(PieceKind::Rook));
        put(&mut board, pos(0, 4), Piece::white(PieceKind::Bishop));

        let mut game = game_with(board, 0);
        assert_move_err(&mut game, pos(0, 0), pos(0, 4));
    }

    #[test]
    fn knights_move_in_an_l_shape() {
        assert!(Piece::white(PieceKind::Knight).legal_move(&pos(3, 3), &pos(5, 4)));
        assert!(Piece::white(PieceKind::Knight).legal_move(&pos(3, 3), &pos(4, 5)));
        assert!(!Piece::white(PieceKind::Knight).legal_move(&pos(3, 3), &pos(5, 5)));
    }

    #[test]
    fn pawns_can_advance_one_square_or_two_from_their_starting_square() {
        let mut game = empty_game_with(Piece::white(PieceKind::Pawn), pos(1, 4));
        assert_move_ok(&mut game, pos(1, 4), pos(3, 4));

        let mut game = empty_game_with(Piece::black(PieceKind::Pawn), pos(6, 4));
        assert_move_ok(&mut game, pos(6, 4), pos(4, 4));

        let mut game = empty_game_with(
            Piece::white(PieceKind::Pawn).increase_move_count(),
            pos(2, 4),
        );
        assert_move_err(&mut game, pos(2, 4), pos(4, 4));
    }

    #[test]
    #[ignore = "pawn file and capture rules are not implemented yet"]
    fn pawns_must_advance_on_the_same_file_without_capturing() {
        let mut game = empty_game_with(Piece::white(PieceKind::Pawn), pos(1, 4));
        assert_move_err(&mut game, pos(1, 4), pos(2, 5));

        let mut board = empty_board();
        put(&mut board, pos(1, 4), Piece::white(PieceKind::Pawn));
        put(&mut board, pos(2, 4), Piece::black(PieceKind::Knight));

        let mut game = game_with(board, 0);
        assert_move_err(&mut game, pos(1, 4), pos(2, 4));
    }

    #[test]
    #[ignore = "pawn diagonal captures are not implemented yet"]
    fn pawns_capture_one_square_diagonally_forward() {
        let mut board = empty_board();
        put(&mut board, pos(1, 4), Piece::white(PieceKind::Pawn));
        put(&mut board, pos(2, 5), Piece::black(PieceKind::Knight));

        let mut game = game_with(board, 0);
        assert_move_ok(&mut game, pos(1, 4), pos(2, 5));

        let mut board = empty_board();
        put(&mut board, pos(6, 4), Piece::black(PieceKind::Pawn));
        put(&mut board, pos(5, 3), Piece::white(PieceKind::Knight));

        let mut game = game_with(board, 1);
        assert_move_ok(&mut game, pos(6, 4), pos(5, 3));

        let mut game = empty_game_with(Piece::white(PieceKind::Pawn), pos(1, 4));
        assert_move_err(&mut game, pos(1, 4), pos(2, 5));
    }

    #[test]
    #[ignore = "king orthogonal movement is not implemented yet"]
    fn kings_move_one_square_in_any_direction() {
        let mut game = empty_game_with(Piece::white(PieceKind::King), pos(3, 3));
        assert_move_ok(&mut game, pos(3, 3), pos(4, 3));

        let mut game = empty_game_with(Piece::white(PieceKind::King), pos(3, 3));
        assert_move_ok(&mut game, pos(3, 3), pos(4, 4));
    }

    #[test]
    #[ignore = "knight moves are incorrectly passed through path validation"]
    fn knights_can_jump_over_occupied_squares() {
        let mut board = empty_board();
        put(&mut board, pos(0, 1), Piece::white(PieceKind::Knight));
        put(&mut board, pos(1, 1), Piece::white(PieceKind::Pawn));
        put(&mut board, pos(2, 2), Piece::black(PieceKind::Pawn));

        let mut game = game_with(board, 0);
        assert_move_ok(&mut game, pos(0, 1), pos(2, 2));
    }

    #[test]
    #[ignore = "check detection is not implemented yet"]
    fn kings_cannot_move_into_check() {
        let mut board = empty_board();
        put(&mut board, pos(0, 4), Piece::white(PieceKind::King));
        put(&mut board, pos(1, 7), Piece::black(PieceKind::Rook));

        let mut game = game_with(board, 0);
        assert_move_err(&mut game, pos(0, 4), pos(1, 5));
    }

    #[test]
    #[ignore = "self-check detection is not implemented yet"]
    fn pieces_cannot_move_if_they_expose_their_own_king_to_check() {
        let mut board = empty_board();
        put(&mut board, pos(0, 4), Piece::white(PieceKind::King));
        put(&mut board, pos(1, 4), Piece::white(PieceKind::Rook));
        put(&mut board, pos(7, 4), Piece::black(PieceKind::Rook));

        let mut game = game_with(board, 0);
        assert_move_err(&mut game, pos(1, 4), pos(1, 5));
    }

    #[test]
    #[ignore = "castling is not implemented yet"]
    fn castling_moves_the_king_and_rook_when_all_castling_conditions_are_met() {
        let mut board = empty_board();
        put(&mut board, pos(0, 4), Piece::white(PieceKind::King));
        put(&mut board, pos(0, 7), Piece::white(PieceKind::Rook));

        let mut game = game_with(board, 0);
        assert_move_ok(&mut game, pos(0, 4), pos(0, 6));

        assert!(matches!(
            game.board.square(&pos(0, 6)),
            Square::Taken(Piece {
                color: Color::White,
                kind: PieceKind::King,
                ..
            })
        ));
        assert!(matches!(
            game.board.square(&pos(0, 5)),
            Square::Taken(Piece {
                color: Color::White,
                kind: PieceKind::Rook,
                ..
            })
        ));
    }

    #[test]
    #[ignore = "castling is not implemented yet"]
    fn castling_is_illegal_after_the_king_or_rook_has_moved_or_through_check() {
        let mut board = empty_board();
        put(
            &mut board,
            pos(0, 4),
            Piece::white(PieceKind::King).increase_move_count(),
        );
        put(&mut board, pos(0, 7), Piece::white(PieceKind::Rook));

        let mut game = game_with(board, 0);
        assert_move_err(&mut game, pos(0, 4), pos(0, 6));

        let mut board = empty_board();
        put(&mut board, pos(0, 4), Piece::white(PieceKind::King));
        put(&mut board, pos(0, 7), Piece::white(PieceKind::Rook));
        put(&mut board, pos(7, 5), Piece::black(PieceKind::Rook));

        let mut game = game_with(board, 0);
        assert_move_err(&mut game, pos(0, 4), pos(0, 6));
    }

    #[test]
    #[ignore = "en passant is not implemented yet"]
    fn en_passant_captures_only_immediately_after_a_two_square_pawn_advance() {
        let mut board = empty_board();
        put(&mut board, pos(4, 4), Piece::white(PieceKind::Pawn));
        put(&mut board, pos(6, 5), Piece::black(PieceKind::Pawn));

        let mut game = game_with(board, 1);
        assert_move_ok(&mut game, pos(6, 5), pos(4, 5));
        assert_move_ok(&mut game, pos(4, 4), pos(5, 5));
        assert!(matches!(game.board.square(&pos(4, 5)), Square::Empty));
    }

    #[test]
    #[ignore = "pawn promotion is not implemented yet"]
    fn pawns_promote_when_they_reach_the_last_rank() {
        let mut game = empty_game_with(Piece::white(PieceKind::Pawn), pos(6, 0));
        assert_move_ok(&mut game, pos(6, 0), pos(7, 0));

        assert!(matches!(
            game.board.square(&pos(7, 0)),
            Square::Taken(Piece {
                color: Color::White,
                kind: PieceKind::Queen,
                ..
            })
        ));
    }

    #[test]
    #[ignore = "checkmate state is not implemented yet"]
    fn checkmate_ends_the_game() {
        panic!("GameState needs a game result API before checkmate can be asserted");
    }

    #[test]
    #[ignore = "stalemate state is not implemented yet"]
    fn stalemate_ends_the_game_as_a_draw() {
        panic!("GameState needs a game result API before stalemate can be asserted");
    }

    #[test]
    #[ignore = "draw state is not implemented yet"]
    fn draw_rules_cover_fifty_move_threefold_repetition_and_insufficient_material() {
        panic!(
            "GameState needs history, halfmove clock, and game result APIs before draw rules can be asserted"
        );
    }
}
