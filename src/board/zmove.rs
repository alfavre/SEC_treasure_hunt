use super::{assert_matches, Board, BoardError, FromStr, Position, Regex};

/// A Zmove is a more intuitive way to move on a grid than teleportation
/// It is inspired by video games, where a move is done relatively from
/// the character perspective instead of a global absolute perspective
/// A Zmove is defined by a direction and a speed
///
/// # Atrributes
/// * `direction` - is a Direction
/// * `speed` - u32 which is the distance done in one zMove
#[derive(Debug, PartialEq)]
pub struct Zmove {
    direction: Direction,
    speed: u32,
}

impl Zmove {
    /// build a zmove from a u32 direction and u32 speed
    ///
    /// # Arguments
    /// * `direction` - a numpad digit direction
    /// * `speed` - the distance the zmove will traverse
    ///
    /// # Returns
    /// * `Zmove` - a legal zmove
    /// * `BoardError::InvalidMove` - if the values are invalid
    pub fn new(direction: u32, speed: u32) -> Result<Zmove, BoardError> {
        let tmp_direction: Direction;
        let tmp_speed: u32;

        if speed > Board::MOVE_MAX_DISTANCE {
            return Err(BoardError::InvalidMove(format!(
                "Your speed is too high, max is {}.",
                Board::MOVE_MAX_DISTANCE
            )));
        } else if speed == 0 {
            return Err(BoardError::InvalidMove(format!(
                "Your speed is 0, you can't move if you have no speed."
            )));
        } else {
            tmp_speed = speed;
        }

        match Direction::get_direction_from_num_pad_int(direction) {
            Ok(dir) => tmp_direction = dir,
            Err(err) => return Err(err), // transmits error upwards
        }

        Ok(Zmove {
            direction: tmp_direction,
            speed: tmp_speed,
        })
    }

    /// this returns the vector representing the zmove
    ///
    /// # Returns
    /// * `(i64,i64)` - the 2dimensial vector
    pub fn get_vector(&self) -> (i64, i64) {
        (
            self.speed as i64 * Direction::get_i64_pair_from_direction(&self.direction).0,
            self.speed as i64 * Direction::get_i64_pair_from_direction(&self.direction).1,
        )
    }
}

impl FromStr for Zmove {
    type Err = BoardError;

    /// from str implementation for zmove
    /// accepetd encapsulator: (), []. none
    /// accepted number pair: only positive integer, separated by a ,
    /// hex accepted, they need to start with 0x
    ///
    /// # Arguments
    /// * `s` - the str we will extract a zmove from
    ///
    /// # Returns
    /// * `Zmove` - the zmove we got from str
    /// * `BoardError::InvalidFormat` - if format is incorrect for () []
    /// * `BoardError::InvalidMove` - if the given parameter are incorect
    /// * `BoardError::FailedParse` - if the number is negative or not a number
    /// * `BoardError::Not2Dimensional` - if there aren't 2 values separated with ,
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let no_space_s = s.trim().replace(|x| x == ' ', ""); //we got rid of spaces

        if !Regex::new(Board::PARENTHESIS_REGEX)
            .unwrap()
            .is_match(no_space_s.as_str())
        {
            return Err(
                BoardError::InvalidFormat(
                    "Incorrect parenthesis format, please format your zmove like this '12,13' '[12,0xc]' '(0x12,14)'".to_string(),
                )
            );
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

            let zmove_to_return = Zmove::new(
                match Position::parse_dec_or_hex(number_value[0]) {
                    Ok(int) => int,
                    Err(err) => return Err(err), // this err should be a failed parse
                },
                match Position::parse_dec_or_hex(number_value[1]) {
                    Ok(int) => int,
                    Err(err) => return Err(err), // this err should be a failed parse
                },
            ); // we get our Result<Zmove,BoardError>

            match zmove_to_return {
                Ok(zmove) => return Ok(zmove),
                Err(err) => return Err(err), //this error should be an invalid move
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Right,
    UpRight,
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
}

impl Direction {
    /// utilitiy method that gives a Direction for the corresponding num pad digit
    ///
    /// # Returns
    /// * `Direction` - the associated direction
    /// * `BoardError::InvalidMove` - when the int isn't a num pad digit (or is 5, which is not a direction)
    pub fn get_direction_from_num_pad_int(int: u32) -> Result<Direction, BoardError> {
        match int {
            6 => Ok(Direction::Right),
            9 => Ok(Direction::UpRight),
            8 => Ok(Direction::Up),
            7 => Ok(Direction::UpLeft),
            4 => Ok(Direction::Left),
            1 => Ok(Direction::DownLeft),
            2 => Ok(Direction::Down),
            3 => Ok(Direction::DownRight),
            _ => Err(BoardError::InvalidMove(
                "This direction doesn't exist on a num pad [only value: 6,9,8,7,4,1,2,3]"
                    .to_string(),
            )),
        }
    }

    /// returns the 2d vector associated to a direction
    ///
    /// # Arguments
    /// * `direction` - the direction from which we want the vector
    ///
    /// # Returns
    /// * `(i64,i64)` - The vector representing a direction
    pub fn get_i64_pair_from_direction(direction: &Direction) -> (i64, i64) {
        match direction {
            Direction::Right => (1, 0),
            Direction::UpRight => (1, 1),
            Direction::Up => (0, 1),
            Direction::UpLeft => (-1, 1),
            Direction::Left => (-1, 0),
            Direction::DownLeft => (-1, -1),
            Direction::Down => (0, -1),
            Direction::DownRight => (1, -1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_zmove_from_str_format() {
        // this tests the format more than the type of speed or direction

        // test format
        assert_eq!(
            Zmove::from_str("1,1").unwrap(),
            Zmove {
                direction: Direction::DownLeft,
                speed: 1
            }
        );

        assert_eq!(
            Zmove::from_str("0x2,0x1").unwrap(),
            Zmove {
                direction: Direction::Down,
                speed: 1
            }
        );

        assert_eq!(
            Zmove::from_str("[0x2,0x1]").unwrap(),
            Zmove {
                direction: Direction::Down,
                speed: 1
            }
        );
    }

    #[test]
    fn valid_zmove_from_str_spaces() {
        // test spaces
        assert_eq!(
            Zmove::from_str("             0x2,0x1              ").unwrap(),
            Zmove {
                direction: Direction::Down,
                speed: 1
            }
        );

        assert_eq!(
            Zmove::from_str("        0          x       2,          0x1       ").unwrap(),
            Zmove {
                direction: Direction::Down,
                speed: 1
            }
        );
    }

    #[test]
    fn valid_zmove_from_str_normal_values() {
        assert_eq!(
            Zmove::from_str("1,1").unwrap(),
            Zmove {
                direction: Direction::DownLeft,
                speed: 1
            }
        );

        assert_eq!(
            Zmove::from_str(format!("9,{}", Board::MOVE_MAX_DISTANCE).as_str()).unwrap(),
            Zmove {
                direction: Direction::UpRight,
                speed: Board::MOVE_MAX_DISTANCE
            }
        );

        assert_eq!(
            Zmove::from_str(format!("9,{}", Board::MOVE_MAX_DISTANCE / 2).as_str()).unwrap(),
            Zmove {
                direction: Direction::UpRight,
                speed: Board::MOVE_MAX_DISTANCE / 2
            }
        );
    }

    #[test]
    fn invalid_zmove_from_str_illegal() {
        // 0 is not legal speed
        assert_matches!(
            Zmove::from_str("1,0").unwrap_err(),
            BoardError::InvalidMove(_)
        );

        // 0 is not a direction
        assert_matches!(
            Zmove::from_str("0,1").unwrap_err(),
            BoardError::InvalidMove(_)
        );

        // speed limits are laws
        assert_matches!(
            Zmove::from_str(format!("1,{}", Board::MOVE_MAX_DISTANCE + 1).as_str()).unwrap_err(),
            BoardError::InvalidMove(_)
        );

        // 5 is not a direction
        assert_matches!(
            Zmove::from_str("5,1").unwrap_err(),
            BoardError::InvalidMove(_)
        );

        // over 9 is not a direction
        assert_matches!(
            Zmove::from_str("10,1").unwrap_err(),
            BoardError::InvalidMove(_)
        );

        // better verifiy max is also not a direction
        assert_matches!(
            Zmove::from_str(format!("{},1", u32::MAX).as_str()).unwrap_err(),
            BoardError::InvalidMove(_)
        );
    }

    #[test]
    fn invalid_zmove_from_str_regex() {
        // regex test here we gooooooooooooooooooooo
        assert_matches!(
            Zmove::from_str("(2,2(").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        assert_matches!(
            Zmove::from_str(")2,2)").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        assert_matches!(
            Zmove::from_str("]2,2]").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        assert_matches!(
            Zmove::from_str("[2,2[").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        assert_matches!(
            Zmove::from_str("(2,2").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        assert_matches!(
            Zmove::from_str("2,2)").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        assert_matches!(
            Zmove::from_str("2,2]").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        assert_matches!(
            Zmove::from_str("[2,2").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        assert_matches!(
            Zmove::from_str("((2,2))").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        assert_matches!(
            Zmove::from_str("[[2,2]]").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        assert_matches!(
            Zmove::from_str("[2,2)").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        assert_matches!(
            Zmove::from_str("(2,2]").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        assert_matches!(
            Position::from_str("[13,0xc]hello").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
        assert_matches!(
            Position::from_str("(13,0xc)hello").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
        assert_matches!(
            Position::from_str("13,0xchello").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        assert_matches!(
            Position::from_str("hello[13,0xc]").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
        assert_matches!(
            Position::from_str("hello(13,0xc)").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
        assert_matches!(
            Position::from_str("hello13,0xc").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        // stupid regex tests

        assert_matches!(
            Zmove::from_str("hello,you good").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        assert_matches!(
            Zmove::from_str("hello,you good, no").unwrap_err(),
            BoardError::InvalidFormat(_)
        );

        assert_matches!(
            Zmove::from_str("").unwrap_err(),
            BoardError::InvalidFormat(_)
        );
    }

    #[test]
    fn invalid_zmove_from_str_dimensions() {
        //dimensions tests

        assert_matches!(
            Zmove::from_str("1,0,0xAF").unwrap_err(),
            BoardError::Not2Dimensional(_)
        );

        assert_matches!(
            Zmove::from_str("1,0,0xAF,,,,").unwrap_err(),
            BoardError::Not2Dimensional(_)
        );

        assert_matches!(
            Zmove::from_str("1").unwrap_err(),
            BoardError::Not2Dimensional(_)
        );
    }

    #[test]
    fn valid_new_zmove() {
        //direction is already tested in valid_direction
        assert!(Zmove::new(6, Board::MOVE_MAX_DISTANCE / 2).is_ok(),);

        assert!(Zmove::new(6, Board::MOVE_MAX_DISTANCE).is_ok(),);

        assert!(Zmove::new(6, 1).is_ok(),);
    }

    #[test]
    fn invalid_new_zmove() {
        // the content of the string is actually important here
        // the invalid directions are already tested in valid_direction() and invalid_direction()
        // we only test the speed
        assert_eq!(
            BoardError::InvalidMove(format!(
                "Your speed is too high, max is {}.",
                Board::MOVE_MAX_DISTANCE
            )),
            Zmove::new(6, Board::MOVE_MAX_DISTANCE + 1).unwrap_err()
        );
        assert_eq!(
            BoardError::InvalidMove(
                "Your speed is 0, you can't move if you have no speed.".to_string()
            ),
            Zmove::new(6, 0).unwrap_err()
        );

        assert_eq!(
            BoardError::InvalidMove(format!(
                "Your speed is too high, max is {}.",
                Board::MOVE_MAX_DISTANCE
            )),
            Zmove::new(6, u32::MAX).unwrap_err()
        );
    }

    #[test]
    fn valid_direction() {
        assert_eq!(
            Direction::get_direction_from_num_pad_int(6).unwrap(),
            Direction::Right
        );

        assert_eq!(
            Direction::get_direction_from_num_pad_int(9).unwrap(),
            Direction::UpRight
        );

        assert_eq!(
            Direction::get_direction_from_num_pad_int(8).unwrap(),
            Direction::Up
        );

        assert_eq!(
            Direction::get_direction_from_num_pad_int(7).unwrap(),
            Direction::UpLeft
        );

        assert_eq!(
            Direction::get_direction_from_num_pad_int(4).unwrap(),
            Direction::Left
        );

        assert_eq!(
            Direction::get_direction_from_num_pad_int(1).unwrap(),
            Direction::DownLeft
        );

        assert_eq!(
            Direction::get_direction_from_num_pad_int(2).unwrap(),
            Direction::Down
        );

        assert_eq!(
            Direction::get_direction_from_num_pad_int(3).unwrap(),
            Direction::DownRight
        );
    }

    #[test]
    fn invalid_direction() {
        assert_matches!(
            Direction::get_direction_from_num_pad_int(0).unwrap_err(),
            BoardError::InvalidMove(_) // 0 doesn't count as a move, it's not a direction, it's not really in the numpad
        );
        assert_matches!(
            Direction::get_direction_from_num_pad_int(5).unwrap_err(),
            BoardError::InvalidMove(_) // 5 doesn't count as a move, it's not a direction, it's in the middle
        );
        assert_matches!(
            Direction::get_direction_from_num_pad_int(10).unwrap_err(),
            BoardError::InvalidMove(_)
        );

        assert_matches!(
            Direction::get_direction_from_num_pad_int(u32::MAX).unwrap_err(),
            BoardError::InvalidMove(_)
        );
    }
}
