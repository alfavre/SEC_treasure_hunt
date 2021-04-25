use super::{assert_matches, Board, BoardError, FromStr, Regex};
use std::cmp::{max, min};
use std::fmt;

/// The representation of a position
/// this position can be outside the board
/// this is rarely verified in the Position level
/// It should be berified at Board level when necessary
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
    ///
    /// # Returns
    ///
    /// `i64 pair` - the position x and y in i64
    pub fn to_i64(&self) -> (i64, i64) {
        (self.x as i64, self.y as i64)
    }

    /// calculates the distance between this posiiton and another one
    /// this doesn't consider the torus properties of the board
    /// and therefore doesn't give the true distance
    /// For true distance look get_shortest_dist
    ///
    /// **important, the given position should exist in board
    /// but if it's not the case, and the returned distance is
    /// bigger than the board, everything will break**
    ///
    /// # Arguments
    /// `self` - the position itslef
    /// `other` - the other position
    ///
    /// # Returns
    /// `u32 pair` - the weird definiiton of distance in this project
    pub fn get_xy_dists(&self, other: &Position) -> (u32, u32) {
        (
            i64::abs(self.to_i64().0 - other.to_i64().0) as u32,
            i64::abs(self.to_i64().1 - other.to_i64().1) as u32,
        )
    }

    /// This will determine the real (torus) and shortest distance from a non-torus distance pair
    /// as the board is a torus, each point can either be reached from one direction or the opposite
    /// from those 2 possible distance for each axis, we determine the shortest distance pair.
    /// However as the distance is defined as the biggest (yes its weird) x **or** y distance, this methods
    /// gives the biggest of the two.
    ///
    /// # Arguments
    /// `dist` - the u32 pair of non-torus distances
    ///
    /// # Returns
    /// `u32` - the shortest distance in the board format
    pub fn get_shortest_dist(dist: (u32, u32)) -> u32 {
        let modular_inverse_dist_x =
            i64::abs(dist.0 as i64 - Board::DEFAULT_BOARD_WIDTH as i64) as u32;
        let modular_inverse_dist_y =
            i64::abs(dist.1 as i64 - Board::DEFAULT_BOARD_HEIGHT as i64) as u32;
        let min_dist_x = std::cmp::min(dist.0, modular_inverse_dist_x);
        let min_dist_y = std::cmp::min(dist.1, modular_inverse_dist_y);
        std::cmp::max(min_dist_x, min_dist_y)
    }

    /// this exploits the torus properties of the board
    /// and therefore is pretty weird
    ///
    /// The board width and height have to be given, as the final goal of this method is to work with all boards
    ///
    /// the complex thing calculated is that the position to the left are actually adjecent to the ones to the right
    /// same for up and down
    ///
    /// **IMPORTANT: the calculated dist will NEVER be bigger than the board,
    /// if it still is, the program will panic**
    ///
    /// # Arguments
    ///
    /// * `xy_dist` - this is **not** a position, but the x and y distance between 2 positions
    /// * `board_width_height` - this is the width and height value of the board
    pub fn is_dist_legal(xy_dist: (u32, u32), board_width_height: (u32, u32)) -> bool {
        if xy_dist.0 > board_width_height.0 || xy_dist.1 > board_width_height.1 {
            panic!("The dist is bigger than the board, this should never happen");
        }
        if (xy_dist.0 <= Board::MOVE_MAX_DISTANCE
            || xy_dist.0 >= board_width_height.0 - Board::MOVE_MAX_DISTANCE)
            && (xy_dist.1 <= Board::MOVE_MAX_DISTANCE
                || xy_dist.1 >= board_width_height.1 - Board::MOVE_MAX_DISTANCE)
        {
            return true;
        }
        return false;
    }

    /// This returns either an error or the integer corresponding to the number in the str
    /// It works for hex and dec, hex have to be in format: `0xCAFE`
    ///
    /// # Arguments
    ///
    /// * `s` - the str reference from which we get an integer
    ///
    /// # Returns
    ///
    /// * `u32` representing the number
    /// * `BoardError::FailedParse` - if the number is negative or not a number
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

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Position {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl FromStr for Position {
    type Err = BoardError;

    /// from str implementation for position
    /// accepetd encapsulator: (), []. none
    /// accepted number pair: only positive integer, separated by a ,
    /// hex accepted, they need to start with 0x
    ///
    /// # Arguments
    /// * `s` - the str we will extract a position from
    ///
    /// # Returns
    /// * `Position` - the position we got from str
    /// * `BoardError::InvalidFormat` - if format is incorrect for () []
    /// * `BoardError::FailedParse` - if the number is negative or not a number
    /// * `BoardError::Not2Dimensional` - if there aren't 2 values separated with ,
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

        let clean_s = no_space_s
            .trim_start_matches(|p| p == '(' || p == '[')
            .trim_end_matches(|p| p == ')' || p == ']')
            .trim();

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
        // logical expectation for legal distances
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

        // unexpected legal distances that are actually legal du to the torus properties of the board

        // x axis
        assert!(
            Position::is_dist_legal(
                (Board::DEFAULT_BOARD_WIDTH, 0),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
            "moving to left/right and doing a warp around and getting on same position (same as not moving)"
        );

        assert!(
            Position::is_dist_legal(
                (Board::DEFAULT_BOARD_WIDTH-Board::MOVE_MAX_DISTANCE, 0),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
            "moving to left/right and doing a warp around and getting on the leftest/rightest after a warparound"
        );

        assert!(
            Position::is_dist_legal(
                (Board::DEFAULT_BOARD_WIDTH-(Board::MOVE_MAX_DISTANCE/2), 0),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
            "moving to left/right and doing a warp around and getting somwhere between the furthest left/right possible after a warp around and the right/left edge of the board"
        );

        // y axis
        assert!(
            Position::is_dist_legal(
                (0, Board::DEFAULT_BOARD_HEIGHT),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
                "moving to up/down and doing a warp around and getting on same position (same as not moving)"
        );

        assert!(
            Position::is_dist_legal(
                (0, Board::DEFAULT_BOARD_HEIGHT-Board::MOVE_MAX_DISTANCE),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
                "moving to up/down and doing a warp around and getting on the downest/upest after a warparound"
            );

        assert!(
            Position::is_dist_legal(
                (0, Board::DEFAULT_BOARD_HEIGHT-(Board::MOVE_MAX_DISTANCE/2)),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
                "moving to up/down and doing a warp around and getting somwhere between the furthest down/up possible after a warp around and the down/up edge of the board"
           );

        // x and y
        assert!(
            Position::is_dist_legal(
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
                "moving to opposite quadrant and doing a warp around and getting on same position (same as not moving)"
        );

        assert!(
            Position::is_dist_legal(
                (Board::DEFAULT_BOARD_WIDTH-Board::MOVE_MAX_DISTANCE, Board::DEFAULT_BOARD_HEIGHT-Board::MOVE_MAX_DISTANCE),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
                "moving to opposite quadrant and doing a warp around and getting on the closest to start after a warparound"
            );

        assert!(
            Position::is_dist_legal(
                (Board::DEFAULT_BOARD_WIDTH-(Board::MOVE_MAX_DISTANCE/2), Board::DEFAULT_BOARD_HEIGHT-(Board::MOVE_MAX_DISTANCE/2)),
                (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
            ),
                "moving to opposite quadrant and doing a warp around and getting somwhere between the closest to start possible after a warp around and the edge of the board"
           );
    }

    #[test]
    fn invalid_dist() {
        // warning, those test only work if the torus is sufficiently large, in such a way that there actually exists illegal distances

        let board_width = Board::DEFAULT_BOARD_WIDTH;
        let board_height = Board::DEFAULT_BOARD_HEIGHT;
        let max_dist = Board::MOVE_MAX_DISTANCE;

        // there are illegal positions in x and y
        assert!(
            !Position::is_dist_legal((max_dist + 1, max_dist + 1), (board_width, board_height)),
            "over max dist x and y should not be legal"
        );

        assert!(
            !Position::is_dist_legal(
                (board_width - (max_dist + 1), board_height - (max_dist + 1)),
                (board_width, board_height)
            ),
            "warparound over max dist x and y should not be legal"
        );

        // if board_height/ 2 <= max_dist && board_width / 2 <= max_dist
        // there are no illegal position

        // else if board_height / 2 <= max_dist && !(board_width / 2 <= max_dist)
        // there are no illegal position in y

        assert!(
            !Position::is_dist_legal((max_dist + 1, 0), (board_width, board_height)),
            "over max dist x should not be legal"
        );

        assert!(
            !Position::is_dist_legal(
                (board_width - (max_dist + 1), 0),
                (board_width, board_height)
            ),
            "warparound over max dist x should not be legal"
        );

        // else if !(board_height / 2 <= max_dist) && board_width / 2 <= max_dist
        // there are no illegal position in x
        assert!(
            !Position::is_dist_legal((0, max_dist + 1), (board_width, board_height)),
            "over max dist y should not be legal"
        );

        assert!(
            !Position::is_dist_legal(
                (0, board_height - (max_dist + 1)),
                (board_width, board_height)
            ),
            "warparound over max dist y should not be legal"
        );

        // impossible values that should trigger a panic
        assert!(std::panic::catch_unwind(|| Position::is_dist_legal(
            (0, Board::DEFAULT_BOARD_HEIGHT + 1),
            (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
        ))
        .is_err());

        assert!(std::panic::catch_unwind(|| Position::is_dist_legal(
            (Board::DEFAULT_BOARD_WIDTH + 1, 0),
            (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
        ))
        .is_err());

        assert!(std::panic::catch_unwind(|| Position::is_dist_legal(
            (
                Board::DEFAULT_BOARD_WIDTH + 1,
                Board::DEFAULT_BOARD_HEIGHT + 1
            ),
            (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT)
        ))
        .is_err());
    }

    #[test]
    fn valid_dec_from_str() {
        assert_eq!(Position::parse_dec_or_hex("0x10").unwrap(), 16);
        assert_eq!(Position::parse_dec_or_hex("0xA").unwrap(), 10);
        assert_eq!(Position::parse_dec_or_hex("0xb").unwrap(), 11);
        assert_eq!(Position::parse_dec_or_hex("10").unwrap(), 10);
        assert_eq!(Position::parse_dec_or_hex("0").unwrap(), 0);
        assert_eq!(Position::parse_dec_or_hex("0x0").unwrap(), 0);
        assert_eq!(
            Position::parse_dec_or_hex("4294967295").unwrap(),
            4294967295
        );
        assert_eq!(
            Position::parse_dec_or_hex("0xFFFFFFFF").unwrap(),
            4294967295
        );
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
        // correct format: `(1,2)` `[1,2]` `1,2`
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

        // other number tests are done in dec_from_str tests

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

        // words not containg A-F and x (used for hex)
        assert_matches!(
            Position::from_str("To zim zum running").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
    }
}
