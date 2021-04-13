use super::{assert_matches, Board, BoardError, FromStr};
use std::cmp::max;

/// I have no idea what is the best way to do this
/// adding a trait to tuple for my to_i64 fn
/// creating a named tuple implementing to_i64
/// creating a struct to make the field named too
/// I went with the named tuple as it seems to save more memory than the struct
#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    /// utility cast that gives a i64 pair of the board_position
    /// this is the only reason why this struct exists
    /// if not &self as argument we have move problems
    pub fn to_i64(&self) -> (i64, i64) {
        (self.x as i64, self.y as i64)
    }

    pub fn get_xy_dists(&self, other: &Position) -> (u32, u32) {
        (
            i64::abs(self.to_i64().0 - other.to_i64().0) as u32,
            i64::abs(self.to_i64().1 - other.to_i64().1) as u32,
        )
    }

    /// this exploits the torus properties of the board
    /// and therefore is pretty weird
    ///
    /// The board width and height have to be given, as the final goal of this method is to work with all boards
    ///
    /// the complex thing calculated is that the position to the left are actually adjecent to the ones to the right
    /// same for up and down
    ///
    ///
    /// # Arguments
    ///
    /// * `xy_dist` - this is **not** a position, but the x and y distance between 2 positions
    /// * `board_width_height` - this is the width and height value of the board
    pub fn is_dist_legal(xy_dist: (u32, u32), board_width_height: (u32, u32)) -> bool {
        if (xy_dist.0 <= Board::MOVE_MAX_DISTANCE
            || xy_dist.0 >= board_width_height.0 - Board::MOVE_MAX_DISTANCE)
            && (xy_dist.1 <= Board::MOVE_MAX_DISTANCE
                || xy_dist.0 >= board_width_height.1 - Board::MOVE_MAX_DISTANCE)
        {
            return true;
        }
        return false;
    }

    pub fn parse_dec_or_hex(s: &str) -> Result<u32, BoardError> {
        if s.contains('x') {
            // hex
            let s_hex = s.trim_start_matches("0x");

            match u32::from_str_radix(s_hex, 16) {
                Ok(num) => Ok(num),
                Err(err) => Err(BoardError::FailedParse(err.to_string())),
            }
        } else {
            // dec
            match u32::from_str(s) {
                Ok(num) => Ok(num),
                Err(err) => Err(BoardError::FailedParse(err.to_string())),
            }
        }
    }
}

impl FromStr for Position {
    type Err = BoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean_s = s
            .trim_start_matches(|p| p == '(' || p == '[')
            .trim_end_matches(|p| p == ')' || p == ']')
            .trim()
            .replace(|x| x == ' ', ""); //we got rid of spaces

        if !clean_s.contains(|c| {
            [
                '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'A', 'b', 'B', 'c', 'C',
                'd', 'D', 'e', 'E', 'f', 'F', ',', '-',
            ]
            .contains(&c)
        }) {
            //nonNumeric handling
            return Err(BoardError::NonNumeric);
        } // numeric handling
        let number_value: Vec<&str> = clean_s.split(',').collect(); // we get the strs

        if number_value.len() != 2 {
            // if not 2 dim
            return Err(BoardError::Not2Dimensional(number_value.len()));
        } else {
            // 2 dim
            let position_to_return = Position {
                x: match Position::parse_dec_or_hex(number_value[0]) {
                    Ok(int) => int,
                    Err(err) => return Err(err),
                },
                y: match Position::parse_dec_or_hex(number_value[1]) {
                    Ok(int) => int,
                    Err(err) => return Err(err),
                },
            };
            Ok(position_to_return)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legality_of_dist() {
        assert!(
            Position::is_dist_legal(
                (Board::MOVE_MAX_DISTANCE, 0),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
            "moving to max dist should be legal, I think"
        );
    }

    #[test]
    fn valid_dec_from_str() {
        assert_eq!(Position::parse_dec_or_hex("0x10").unwrap(), 16);
        assert_eq!(Position::parse_dec_or_hex("0xA").unwrap(), 10);
        assert_eq!(Position::parse_dec_or_hex("0xb").unwrap(), 11);
        assert_eq!(Position::parse_dec_or_hex("10").unwrap(), 10);
    }

    #[test]
    fn invalid_dec_from_str() {
        assert_matches!(
            Position::parse_dec_or_hex("A").unwrap_err(),
            BoardError::FailedParse(_)
        );
        assert_matches!(
            Position::parse_dec_or_hex("-20").unwrap_err(),
            BoardError::FailedParse(_)
        );
        assert_matches!(
            Position::parse_dec_or_hex("0xAG").unwrap_err(),
            BoardError::FailedParse(_)
        );
        assert!(i64::from_str("0x10").is_err()); // O ye old rusty king, why such form worketh not ?
    }

    #[test]
    fn valid_position_from_str() {
        assert_eq!(
            Position::from_str("[13,0xc]").unwrap(),
            Position { x: 13, y: 12 }
        );
        assert_eq!(
            Position::from_str("(13,0xc)").unwrap(),
            Position { x: 13, y: 12 }
        );
        assert_eq!(
            Position::from_str("[           13      ,         0xc        ]").unwrap(),
            Position { x: 13, y: 12 }
        );
        assert_eq!(
            Position::from_str(
                "[           1              3      ,         0         x          c        ]"
            )
            .unwrap(),
            Position { x: 13, y: 12 }
        );
        assert_eq!(
            Position::from_str("-0x0,-0").unwrap(),
            Position { x: 0, y: 0 }
        );
        assert_eq!(
            Position::from_str("0x12,0xc").unwrap(),
            Position { x: 18, y: 12 }
        );
    }

    #[test]
    fn invalid_position_from_str() {
        assert_matches!(
            Position::from_str("]13,0xc[").unwrap_err(),
            BoardError::FailedParse(_)
        );
        assert_matches!(
            Position::from_str(")13,0xc(").unwrap_err(),
            BoardError::FailedParse(_)
        );
        assert_matches!(
            Position::from_str("(13,0xc]").unwrap_err(),
            BoardError::FailedParse(_)
        );
        assert_matches!(
            Position::from_str("[13,0xc)").unwrap_err(),
            BoardError::FailedParse(_)
        );
        assert_matches!(
            Position::from_str("-4,-6").unwrap_err(),
            BoardError::FailedParse(_)
        ); // negative number are refused
        assert_matches!(
            Position::from_str("(13,0xc,12)").unwrap_err(),
            BoardError::Not2Dimensional(_)
        );
        assert_matches!(
            Position::from_str("12").unwrap_err(),
            BoardError::Not2Dimensional(_)
        );
        assert_matches!(Position::from_str("").unwrap_err(), BoardError::NonNumeric);
        assert_matches!(
            Position::from_str("hello there, general kenobi").unwrap_err(),
            BoardError::FailedParse(_)
        ); // contains abcdefx therefore is a "number"
        assert_matches!(
            Position::from_str("To zoom running").unwrap_err(),
            BoardError::NonNumeric
        ); // as abcdefx are considered numeric, this error only triggers for word without thoses
    }
}