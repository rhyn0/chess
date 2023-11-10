/// Module to represent an engine optimized board object and its helper functions

#[derive(PartialEq, Debug, Clone)]
enum CastleAvailable {
    QueenSide,
    KingSide,
    None,
    Both,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone)]
pub struct PerformantRepr {
    /// This is meant to be performant, so we will use 64bit numbers
    /// Chess board is 8x8 so we will have bit 0 (LSB) represent a1
    /// The next bit would be a2, so on through the ranks
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
    white_castle: CastleAvailable,
    black_castle: CastleAvailable,
    current_turn: Color,
    moves_count: usize,
    half_moves_count: u32,
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
            b_king: 576_460_752_303_423_488,
            b_queen: 1_152_921_504_606_846_976,
            b_knight: 4_755_801_206_503_243_776,
            b_bishop: 2_594_073_385_365_405_696,
            b_rook: 9_295_429_630_892_703_744,
            b_pawn: 71_776_119_061_217_280,
            white_castle: CastleAvailable::Both,
            black_castle: CastleAvailable::Both,
            current_turn: Color::White,
            moves_count: 0,
            half_moves_count: 0,
        }
    }
}

impl PerformantRepr {}
