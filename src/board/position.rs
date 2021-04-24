use super::{assert_matches, Board, BoardError, FromStr, Regex};
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

// special thanks to : https://regexr.com/
// warning this regex accepts negative numbers (or nonsensical numbers like 3-4-5)
static PARENTHESIS_REGEX: &str =
    r"^([(]{1}[0-9,a-fxA-F\-]+[)]{1}$|[\[]{1}[0-9,a-fxA-F\-]+[\]]{1}$|[0-9,a-fxA-F\-]+$)";

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

    /// This returns either an error or the integer corresponding to the number in the str
    /// It works for hex and dec
    ///
    /// # Arguments
    ///
    /// * `s` - the str reference from which we get an integer
    ///
    /// # Returns
    ///
    /// * `u32` representing the number
    /// * `BoardError::FailedParse` if the number is negative or not a number
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
        let no_space_s = s.trim().replace(|x| x == ' ', ""); //we got rid of spaces

        if !Regex::new(PARENTHESIS_REGEX)
            .unwrap()
            .is_match(no_space_s.as_str())
        {
            return Err(BoardError::InvalidFormat(
                "Incorrect parenthesis format".to_string(),
            ));
        }

        let clean_s = s
            .trim_start_matches(|p| p == '(' || p == '[')
            .trim_end_matches(|p| p == ')' || p == ']')
            .trim()
            .replace(|x| x == ' ', ""); //we got rid of spaces

        // numeric handling
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
    fn valid_dist() {
        assert!(
            Position::is_dist_legal(
                (Board::MOVE_MAX_DISTANCE, 0),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
            "moving to max x dist should be legal"
        );
        assert!(
            Position::is_dist_legal(
                (0, Board::MOVE_MAX_DISTANCE),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
            "moving to max y dist should be legal"
        );
        assert!(
            Position::is_dist_legal(
                (Board::MOVE_MAX_DISTANCE, Board::MOVE_MAX_DISTANCE),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
            "max x and max y is legal"
        );
        assert!(
            Position::is_dist_legal(
                (Board::MOVE_MAX_DISTANCE / 2, Board::MOVE_MAX_DISTANCE / 2),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
            "somewhere in the middle is legal"
        );

        assert!(
            Position::is_dist_legal(
                (0, 0),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
            "not moving at all should be legal"
        );
    }

    #[test]
    fn invalid_dist() {
        assert!(
            !Position::is_dist_legal(
                (Board::MOVE_MAX_DISTANCE + 1, 0),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
            "over max dist x should not be legal"
        );
        assert!(
            !Position::is_dist_legal(
                (Board::MOVE_MAX_DISTANCE + 1, Board::MOVE_MAX_DISTANCE + 1),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
            "over max dist x and y should not be legal"
        );
        assert!(
            !Position::is_dist_legal(
                (0, Board::MOVE_MAX_DISTANCE + 1),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
            "over max dist y should not be legal"
        );
        //negative dist are impossible as they take u32
    }

    #[test]
    fn valid_dec_from_str() {
        assert_eq!(Position::parse_dec_or_hex("0x10").unwrap(), 16);
        assert_eq!(Position::parse_dec_or_hex("0xA").unwrap(), 10);
        assert_eq!(Position::parse_dec_or_hex("0xb").unwrap(), 11);
        assert_eq!(Position::parse_dec_or_hex("10").unwrap(), 10);
        assert_eq!(Position::parse_dec_or_hex("0").unwrap(), 0);
        assert_eq!(Position::parse_dec_or_hex("0x0").unwrap(), 0);
        assert_eq!(Position::parse_dec_or_hex("4294967295").unwrap(), 4294967295);
    }

    #[test]
    fn invalid_dec_from_str() {
        assert_matches!(
            Position::parse_dec_or_hex("A").unwrap_err(),
            BoardError::FailedParse(_)
        );
        assert_matches!(
            Position::parse_dec_or_hex("12-34").unwrap_err(),
            BoardError::FailedParse(_)
        );
        assert_matches!(
            Position::parse_dec_or_hex("x34").unwrap_err(),
            BoardError::FailedParse(_)
        );
        assert_matches!(
            Position::parse_dec_or_hex("-0x34").unwrap_err(),
            BoardError::FailedParse(_)
        );
        assert_matches!(
            Position::parse_dec_or_hex("-20").unwrap_err(),
            BoardError::FailedParse(_)
        ); // negative number are refused for u32
        assert_matches!(
            Position::parse_dec_or_hex("0xAG").unwrap_err(),
            BoardError::FailedParse(_)
        );
        assert_matches!(
            Position::parse_dec_or_hex("4294967296").unwrap_err(), // bigger than u32
            BoardError::FailedParse(_)
        );
        assert_matches!(
            Position::parse_dec_or_hex("0x100000000").unwrap_err(), // bigger than u32
            BoardError::FailedParse(_)
        );


        assert!(i64::from_str("0x10").is_err()); // rust should handle this, but alas, doesn't
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
            Position::from_str("0x0,0").unwrap(),
            Position { x: 0, y: 0 }
        );
        assert_eq!(
            Position::from_str("0x12,0xc").unwrap(),
            Position { x: 18, y: 12 }
        );
    }

    #[test]
    fn invalid_position_from_str() {
        // incorrect format `(1,2)` `[1,2]` `1,2`
        assert_matches!(
            Position::from_str("]13,0xc[").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
        assert_matches!(
            Position::from_str(")13,0xc(").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
        assert_matches!(
            Position::from_str("((13,0xc))").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
        assert_matches!(
            Position::from_str("[[13,0xc]]").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
        assert_matches!(
            Position::from_str("13,0xc]").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
        assert_matches!(
            Position::from_str("13,0xc)").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
        assert_matches!(
            Position::from_str("(13,0xc").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
        assert_matches!(
            Position::from_str("[13,0xc").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
        assert_matches!(
            Position::from_str("(13,0xc]").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
        assert_matches!(
            Position::from_str("[13,0xc)").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        // incorrect number
        assert_matches!(
            Position::from_str("-4,-6").unwrap_err(),
            BoardError::FailedParse(_)
        ); // negative number are refused
        assert_matches!(
            Position::from_str("-0,-0x0").unwrap_err(),
            BoardError::FailedParse(_)
        ); // negative number are refused

        // other number tests are done in

        // inccorect numbers of values
        assert_matches!(
            Position::from_str("(13,0xc,12)").unwrap_err(),
            BoardError::Not2Dimensional(_)
        );
        assert_matches!(
            Position::from_str("(13,12,)").unwrap_err(),
            BoardError::Not2Dimensional(_)
        );
        assert_matches!(
            Position::from_str("(13,,12)").unwrap_err(),
            BoardError::Not2Dimensional(_)
        );
        assert_matches!(
            Position::from_str("12").unwrap_err(),
            BoardError::Not2Dimensional(_)
        );

        // there is no more difference between numeric and non numeric error handling
        // the next matches are just kept even if they test the same thing.

        // empty string
        assert_matches!(
            Position::from_str("").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
        // words contaings A-F and x (used for hex)
        assert_matches!(
            Position::from_str("hello there, general kenobi").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        // words not contaings A-F and x (used for hex)
        assert_matches!(
            Position::from_str("To zoom running").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
    }
}
