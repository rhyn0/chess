use strum::{Display, EnumProperty, EnumString};
// use strum_macros;

#[derive(Display, EnumString, Debug, PartialEq)]
enum ChessColor {
    #[strum(serialize = "b")]
    Black,
    #[strum(serialize = "w")]
    White,
}

#[derive(EnumProperty, Display, EnumString, Debug, PartialEq)]
enum PieceTypes {
    #[strum(serialize = "p", props(Value = "1"))]
    Pawn,
    #[strum(serialize = "r", props(Value = "5"))]
    Rook,
    #[strum(serialize = "n", props(Value = "3"))]
    Knight,
    #[strum(serialize = "b", props(Value = "3"))]
    Bishop,
    #[strum(serialize = "q", props(Value = "9"))]
    Queen,
    #[strum(serialize = "k", props(Value = "-1"))]
    King,
}

impl TryFrom<isize> for PieceTypes {
    type Error = &'static str;
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        // ambiguity exists for this due to bishop and knight both being 3
        // default to make a bishop
        match value {
            1 => Ok(PieceTypes::Pawn),
            3 => Ok(PieceTypes::Bishop),
            5 => Ok(PieceTypes::Rook),
            9 => Ok(PieceTypes::Queen),
            -1 => Ok(PieceTypes::King),
            _ => Err("Unknown Chess Piece type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use strum::EnumProperty;

    use super::{ChessColor, PieceTypes};

    #[test]
    fn test_chess_color_tostring() {
        assert_eq!(String::from("b"), ChessColor::Black.to_string());
        assert_eq!(String::from("w"), ChessColor::White.to_string());
    }

    #[test]
    fn test_chess_color_fromstring() {
        assert_eq!(ChessColor::from_str("b").unwrap(), ChessColor::Black);
        assert_eq!(ChessColor::from_str("w").unwrap(), ChessColor::White);
        assert!(ChessColor::from_str("h").is_err());
    }

    #[test]
    fn test_piece_value() {
        assert_eq!(
            String::from("1"),
            PieceTypes::Pawn.get_str("Value").unwrap()
        );
        assert_eq!(
            String::from("3"),
            PieceTypes::Bishop.get_str("Value").unwrap()
        );
        assert_eq!(
            String::from("-1"),
            PieceTypes::King.get_str("Value").unwrap()
        );
    }

    #[test]
    fn test_piece_string() {
        assert_eq!(String::from("p"), PieceTypes::Pawn.to_string());
        assert_eq!(String::from("k"), PieceTypes::King.to_string());
        assert_eq!(String::from("q"), PieceTypes::Queen.to_string());
    }

    #[test]
    fn test_piece_fromstring() {
        assert_eq!(PieceTypes::from_str("p").unwrap(), PieceTypes::Pawn);
        assert_eq!(PieceTypes::from_str("k").unwrap(), PieceTypes::King);
        assert_eq!(PieceTypes::from_str("q").unwrap(), PieceTypes::Queen);
        assert!(PieceTypes::from_str("e").is_err());
    }

    #[test]
    fn test_from_piece_val() {
        assert_eq!(PieceTypes::try_from(1).unwrap(), PieceTypes::Pawn);
        assert_eq!(PieceTypes::try_from(3).unwrap(), PieceTypes::Bishop);
        assert_eq!(PieceTypes::try_from(9).unwrap(), PieceTypes::Queen);
        assert!(PieceTypes::try_from(2).is_err());
    }
}
