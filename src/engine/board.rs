#![allow(dead_code)]
/// Module to represent an engine optimized board object and its helper functions
use super::piece::{Color, Type};
use flagset::{flags, FlagSet};
use std::str::FromStr;

/// Given the broken up rank, file
/// # Arguments
/// * `rank` - 1 indexed file to select. 1-8
/// * `file` - index of the file to select 0-7
/// Return the int mask to select that square against state ints.
pub fn select_rank_file(rank: u8, file: u8) -> u64 {
    assert!(file.clamp(0, 7) == file, "Invalid file selected");
    assert!(rank.clamp(1, 8) == rank, "Invalid rank selected");
    let shifts = rank - 1;
    2_u64.pow((file).into()) << (8 * shifts)
}

/// Return the rank string in the form of [(rank, file, piece representation)]
/// Note that file won't be the a-h in this case, but the 0-7 index of the file
fn fen_to_rank((rank, fen_string): (usize, &str)) -> Vec<(usize, u8, char)> {
    let mut file = 0;
    fen_string
        .chars()
        .filter_map(|c| {
            if c.is_numeric() {
                file += u8::try_from(c.to_digit(10).unwrap()).unwrap();
                return None;
            }
            file += 1;
            Some((rank, file - 1, c))
        })
        .collect()
}

flags! {
    enum CastleAvailable: u8 {
        None = 0b00,
        QueenSide = 0b01,
        KingSide = 0b10,
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PerformantRepr {
    /// This is meant to be performant, so we will use 64bit numbers
    /// Chess board is 8x8 so we will have bit 0 (LSB) represent a1
    /// The next bit would be b1, so on through the files and then the ranks
    /// Each number represents a piece type
    w_king: u64,
    w_queen: u64,
    w_knight: u64,
    w_bishop: u64,
    w_rook: u64,
    w_pawn: u64,
    // black pieces
    b_king: u64,
    b_queen: u64,
    b_knight: u64,
    b_bishop: u64,
    b_rook: u64,
    b_pawn: u64,
    // board specifics
    white_castle: FlagSet<CastleAvailable>,
    black_castle: FlagSet<CastleAvailable>,
    current_turn: Color,
    moves_count: usize,
    half_moves_count: u32,
    en_passant: u64,
}

impl Default for PerformantRepr {
    fn default() -> Self {
        Self {
            w_king: 16,
            w_queen: 8,
            w_knight: 66,
            w_bishop: 36,
            w_rook: 129,
            w_pawn: 65_280,
            b_king: 1_152_921_504_606_846_976,
            b_queen: 576_460_752_303_423_488,
            b_knight: 4_755_801_206_503_243_776,
            b_bishop: 2_594_073_385_365_405_696,
            b_rook: 9_295_429_630_892_703_744,
            b_pawn: 71_776_119_061_217_280,
            white_castle: (CastleAvailable::KingSide | CastleAvailable::QueenSide),
            black_castle: (CastleAvailable::KingSide | CastleAvailable::QueenSide),
            current_turn: Color::White,
            // moves starts at 1
            moves_count: 1,
            half_moves_count: 0,
            en_passant: 0,
        }
    }
}

impl PerformantRepr {
    pub fn blank() -> Self {
        Self {
            w_king: 0,
            w_queen: 0,
            w_knight: 0,
            w_bishop: 0,
            w_rook: 0,
            w_pawn: 0,
            b_king: 0,
            b_queen: 0,
            b_knight: 0,
            b_bishop: 0,
            b_rook: 0,
            b_pawn: 0,
            white_castle: (CastleAvailable::KingSide | CastleAvailable::QueenSide),
            black_castle: (CastleAvailable::KingSide | CastleAvailable::QueenSide),
            current_turn: Color::White,
            moves_count: 1,
            half_moves_count: 0,
            en_passant: 0,
        }
    }
    fn board_position_fen(&mut self, board_fen: &str) {
        board_fen
            .split('/')
            .enumerate()
            .flat_map(fen_to_rank)
            .for_each(|(rank, file, piece)| {
                // enumerate is 0 indexed, but ranks are 1 indexed
                // FEN is also notated from the 8th rank to the 1st rank
                let mask = select_rank_file((8 - rank).try_into().unwrap(), file);
                match Type::from_str(&piece.to_string()) {
                    Ok(Type::King) => {
                        if Color::get_color_from_fen(piece) == Color::White {
                            self.w_king |= mask;
                        } else {
                            self.b_king |= mask;
                        }
                    }
                    Ok(Type::Queen) => {
                        if Color::get_color_from_fen(piece) == Color::White {
                            self.w_queen |= mask;
                        } else {
                            self.b_queen |= mask;
                        }
                    }
                    Ok(Type::Bishop) => {
                        if Color::get_color_from_fen(piece) == Color::White {
                            self.w_bishop |= mask;
                        } else {
                            self.b_bishop |= mask;
                        }
                    }
                    Ok(Type::Knight) => {
                        if Color::get_color_from_fen(piece) == Color::White {
                            self.w_knight |= mask;
                        } else {
                            self.b_knight |= mask;
                        }
                    }
                    Ok(Type::Rook) => {
                        if Color::get_color_from_fen(piece) == Color::White {
                            self.w_rook |= mask;
                        } else {
                            self.b_rook |= mask;
                        }
                    }
                    Ok(Type::Pawn) => {
                        if Color::get_color_from_fen(piece) == Color::White {
                            self.w_pawn |= mask;
                        } else {
                            self.b_pawn |= mask;
                        }
                    }
                    Err(e) => {
                        panic!("Invalid piece type: {e}");
                    }
                }
            });
    }
    fn turn_fen(&mut self, turn_fen: &str) {
        match Color::from_str(turn_fen) {
            Ok(Color::White) => {
                self.current_turn = Color::White;
            }
            Ok(Color::Black) => {
                self.current_turn = Color::Black;
            }
            Err(e) => {
                panic!("Invalid turn character: {e}");
            }
        }
    }
    fn castling_fen(&mut self, castling_fen: &str) {
        if castling_fen == "-" {
            self.white_castle = CastleAvailable::None.into();
            self.black_castle = CastleAvailable::None.into();
            return;
        }
        castling_fen.chars().for_each(|c| match c {
            'K' => {
                self.white_castle |= CastleAvailable::KingSide;
            }
            'Q' => {
                self.white_castle |= CastleAvailable::QueenSide;
            }
            'k' => {
                self.black_castle |= CastleAvailable::KingSide;
            }
            'q' => {
                self.black_castle |= CastleAvailable::QueenSide;
            }
            _ => {
                panic!("Invalid castling character");
            }
        });
    }
    /// This is still experimental as I decide how we want to represent en passant
    /// And make it usable in the engine
    #[allow(unused_variables)]
    fn en_passant_fen(&mut self, en_passant_fen: &str) {
        if en_passant_fen == "-" {
            return;
        }
        // this fen location is the algebraic square notation of the pawn that can be captured en passant
        // so file is a char of a-h and rank is a digit 1-8
        // need to adjust their input to be 0 indexed and 1 indexed respectively
        let file = en_passant_fen.chars().nth(0).unwrap();
        let rank = en_passant_fen.chars().nth(1).unwrap();
        #[allow(clippy::cast_possible_truncation)]
        let mask = select_rank_file(rank.to_digit(10).unwrap() as u8, file as u8 - b'a');
        self.en_passant = mask;
    }
    pub fn from_fen(fen_string: &str) -> Self {
        let mut board = Self::blank();
        // FEN has six fields:
        // board layout, whose turn it is, castling rights, en passant pawns, number of half moves, number of full moves.
        // These fields are separated by a space
        let fen_split = fen_string.split(' ').collect::<Vec<&str>>();
        // build board layout
        board.board_position_fen(fen_split[0]);
        board.turn_fen(fen_split[1]);
        board.castling_fen(fen_split[2]);
        // board.en_passant_fen(fen_split[3]);
        board.half_moves_count = fen_split[4].parse::<u32>().unwrap();
        board.moves_count = fen_split[5].parse::<usize>().unwrap();
        board
    }
    #[allow(unused_variables, clippy::unused_self)]
    pub fn valid_move(&self, piece: &Type, moving: &Color, from: u64, to: u64) -> bool {
        // first check that the piece can make that type of move
        // knights can't move diagonally etc
        if !piece.valid_move(from, to) {
            return false;
        }
        // TODO: implement iter for Board, so that we can iterate over the positions of pieces
        // any(piece_set & to) != 0 then we can't move there (depending on piece color)
        // then check that the piece can make that move given board state
        // piece can't occupy a square with a friendly piece
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_color_from_string() {
        assert_eq!(Color::get_color_from_fen('K'), Color::White);
        assert_eq!(Color::get_color_from_fen('Q'), Color::White);
        assert_eq!(Color::get_color_from_fen('p'), Color::Black);
        assert_eq!(Color::get_color_from_fen('n'), Color::Black);
    }
    #[test]
    #[should_panic(expected = "Invalid piece type")]
    fn test_color_from_string_number() {
        Color::get_color_from_fen('1');
    }
    #[test]
    #[should_panic(expected = "Invalid piece type")]
    fn test_color_from_string_not_chess() {
        Color::get_color_from_fen('h');
    }
    #[test]
    fn test_location_mask() {
        for rank in 1..=8 {
            for file in 0..=7 {
                let mask = select_rank_file(rank, file);
                assert_eq!(mask.count_ones(), 1);
                assert_eq!(mask.trailing_zeros(), ((rank - 1) * 8 + file).into());
            }
        }
    }
    #[test]
    #[should_panic(expected = "Invalid rank selected")]
    fn test_location_mask_rank_low() {
        select_rank_file(0, 2);
    }
    #[test]
    #[should_panic(expected = "Invalid rank selected")]
    fn test_location_mask_rank_high() {
        select_rank_file(10, 2);
    }
    #[test]
    #[should_panic(expected = "Invalid file selected")]
    fn test_location_mask_file_high() {
        select_rank_file(3, 10);
    }
    #[test]
    fn test_default_fen() {
        let starting_pos = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = PerformantRepr::from_fen(starting_pos);
        assert_eq!(board, PerformantRepr::default());
    }
}
