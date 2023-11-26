use std::str::FromStr;

use strum_macros::EnumString;

#[derive(Eq, PartialEq, Debug, Clone, EnumString)]
pub enum Type {
    // match char case-insensitive to get the piece type
    #[strum(serialize = "k", ascii_case_insensitive)]
    King,
    #[strum(serialize = "q", ascii_case_insensitive)]
    Queen,
    #[strum(serialize = "b", ascii_case_insensitive)]
    Bishop,
    #[strum(serialize = "n", ascii_case_insensitive)]
    Knight,
    #[strum(serialize = "r", ascii_case_insensitive)]
    Rook,
    #[strum(serialize = "p", ascii_case_insensitive)]
    Pawn,
}

impl Type {
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn index_moved(from: u64, to: u64) -> i64 {
        // board locations are defined as 2^i where i is the index of the location
        // a1 being 0 and b1 being 2 so on
        // so to get the index of the location we can use log2
        // the difference between the two locations should be 1 or 7 or 8 or 9
        (from as f64).log2() as i64 - (to as f64).log2() as i64
    }
    fn king_valid_move(from: u64, to: u64) -> bool {
        // board locations are defined as 2^i where i is the index of the location
        // a1 being 0 and b1 being 2 so on
        // so to get the index of the location we can use log2
        // the difference between the two locations should be 1 or 7 or 8 or 9
        let diff = Self::index_moved(from, to);
        diff.abs() == 1 || diff.abs() == 7 || diff.abs() == 8 || diff.abs() == 9
    }
    fn queen_valid_move(from: u64, to: u64) -> bool {
        // queen is a combination of rook and bishop
        Self::rook_valid_move(from, to) || Self::bishop_valid_move(from, to)
    }
    fn bishop_valid_move(from: u64, to: u64) -> bool {
        // bishop can move diagonally
        // the difference between the two locations should be multiple of 7 or 9
        let diff = Self::index_moved(from, to);
        diff.abs() % 7 == 0 || diff.abs() % 9 == 0
    }
    fn knight_valid_move(from: u64, to: u64) -> bool {
        // knight can move in L shape
        // the difference between the two locations should be 6 or 10 or 15 or 17
        let diff = Self::index_moved(from, to);
        diff.abs() == 6 || diff.abs() == 10 || diff.abs() == 15 || diff.abs() == 17
    }
    fn rook_valid_move(from: u64, to: u64) -> bool {
        // rook can move horizontally or vertically
        // the difference between the two locations vertically should be multiple of 8
        // the difference between two locations horizontally should be less than 8
        let diff = Self::index_moved(from, to);
        diff.abs() % 8 == 0 || diff.abs() < 8
    }
    fn pawn_valid_move(from: u64, to: u64) -> bool {
        // pawn can move forward by 8 or 16 on first move
        // pawn can attack diagonally by 7 or 9
        let diff = Self::index_moved(from, to);
        // check if the pawn is able to double move forward
        if diff.abs() == 16 {
            // check if the pawn is in the starting position
            // if the pawn is in the starting position then the difference between the two locations should be 16
            // if the pawn is not in the starting position then the difference between the two locations should be 8
            // TODO: fix this, this checks if it is on b1 or g1 not the 1st or 6th rank
            return false;
        }
        diff.abs() == 8 || diff.abs() == 7 || diff.abs() == 9
    }
    pub fn valid_move(&self, from: u64, to: u64) -> bool {
        // we don't need to check out of bounds for the board
        // since our data type is u64 and we are using bitboard
        match self {
            Self::King => Self::king_valid_move(from, to),
            Self::Queen => Self::queen_valid_move(from, to),
            Self::Bishop => Self::bishop_valid_move(from, to),
            Self::Knight => Self::knight_valid_move(from, to),
            Self::Rook => Self::rook_valid_move(from, to),
            Self::Pawn => Self::pawn_valid_move(from, to),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, EnumString)]
pub enum Color {
    #[strum(serialize = "w")]
    White,
    #[strum(serialize = "b")]
    Black,
}

impl Color {
    /// # Panics
    ///
    /// Panic occurs when the input character is not a valid chess piece identifier
    pub fn get_color_from_fen(piece_fen: char) -> Self {
        assert!(
            piece_fen.is_ascii_alphabetic() && Type::from_str(&piece_fen.to_string()).is_ok(),
            "Invalid piece type"
        );
        if piece_fen.is_uppercase() {
            return Self::White;
        }
        Self::Black
    }
}
