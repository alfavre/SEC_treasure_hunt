use super::{assert_matches, Board, BoardError};
use read_input::prelude::*;
use std::io::{self};
use std::str::FromStr;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

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

#[derive(Debug, PartialEq)]
pub struct Zmove {
    direction: Direction,
    speed: u32,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    AskTeleport,
    AskZmove,
    Zmove(Zmove), // secret command
    Search,
    Quit,
}

/// this is the implementation for general Command
impl FromStr for Command {
    type Err = BoardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean_s = s
            .trim_start_matches(|p| p == '(' || p == '[')
            .trim_end_matches(|p| p == ')' || p == ']')
            .trim()
            .replace(|x| x == ' ', ""); //we got rid of spaces, )([])

        if !clean_s
            .contains(|c| ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', ',', '-'].contains(&c))
        {
            //nonNumeric handling
            match clean_s.to_lowercase().as_str() {
                "z" | "zmove" => return Ok(Command::AskZmove),
                "move" | "m" => return Ok(Command::AskTeleport),
                "search" | "s" => return Ok(Command::Search),
                "exit" | "quit" | "q" | "e" => return Ok(Command::Quit),
                _ => {
                    return Err(BoardError::InvalidCommand(
                        "this word is not recognised".to_string(),
                    ))
                }
            }
        } // numeric handling
        let number_value: Vec<&str> = clean_s.split(',').collect();
        if number_value.len() == 1 {
            // number choice
            match number_value[0].parse::<u32>() {
                Ok(0) => return Ok(Command::AskTeleport),
                Ok(1) => return Ok(Command::Search),
                Ok(2) => return Ok(Command::Quit),
                Ok(_) => {
                    return Err(BoardError::InvalidCommand(
                        "this u32 is not legal".to_string(),
                    ))
                }
                Err(_) => return Err(BoardError::FailedParse(
                    "You need to enter an u32 corresponding to the command, or the command itself"
                        .to_string(),
                )),
            }
        } else if number_value.len() == 2 {
            // this is the schmove shortcut
            Err(BoardError::NotImplemented)
        } else {
            // incorect number of parameter
            return Err(BoardError::TooManyArguments(number_value.len()));
        }
    }
}

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

#[derive(Debug, PartialEq)]
pub struct GameSettings {
    pub seed: u64,
    pub player_color: Color,
    pub player_tile: char,
    pub board_width: u32,
    pub board_height: u32,
}

/// should I move this in constants ?
impl GameSettings {
    pub fn get_default_settings() -> GameSettings {
        GameSettings {
            seed: Board::DEFAULT_SEED,
            player_color: Board::DEFAULT_PLAYER_COLOR,
            player_tile: Board::DEFAULT_PLAYER_TILE,
            board_width: Board::DEFAULT_BOARD_WIDTH,
            board_height: Board::DEFAULT_BOARD_HEIGHT,
        }
    }
}

//This method only exists for me to understand stdin
pub fn get_bricolage() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let action: String;
    let integer: u32 = 0;
    let mut pair: (i64, i64) = (0, 0);
    let mut triple: (u32, u32, u32) = (0, 0, 0);

    let salut = Board::DEFAULT_BOARD_HEIGHT;
    let salut2 = super::Board::DEFAULT_BOARD_WIDTH;

    println!("You entered: {}", buffer);

    let trimmed = buffer.trim();

    let inputs: Vec<i64> = trimmed
        .split(' ')
        .map(|x| x.parse().expect("not a i64!"))
        .collect();

    println!("here is buffer: {} here is trim: {}", buffer, trimmed);

    println!("here are the {} inputs splited:", inputs.len());

    let mut i: usize = 0;
    for input in inputs {
        println!("input {} is {}", i, input);
        i += 1;
    }

    /*

    match trimmed.parse::<u32>() {
        Ok(i) => integer = i,
        Err(..) => println!("{} is not a single u32",trimmed),
    };


    println!("it's over, her is the u32 {}", integer);

    */

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
            Position::from_str("[13,12)").unwrap(),
            Position { x: 13, y: 12 }
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

    #[test]
    fn valid_command_from_str() {
        assert_eq!(Command::from_str("move").unwrap(), Command::AskTeleport);
        assert_eq!(Command::from_str("m").unwrap(), Command::AskTeleport);
        assert_eq!(Command::from_str("mOvE").unwrap(), Command::AskTeleport);
        assert_eq!(
            Command::from_str("( m O v E ]").unwrap(),
            Command::AskTeleport
        );

        assert_eq!(Command::from_str("0").unwrap(), Command::AskTeleport);
        assert_eq!(Command::from_str("(0)").unwrap(), Command::AskTeleport);

        assert_eq!(Command::from_str("search").unwrap(), Command::Search);
        assert_eq!(Command::from_str("s").unwrap(), Command::Search);
        assert_eq!(Command::from_str("1").unwrap(), Command::Search);

        assert_eq!(Command::from_str("quit").unwrap(), Command::Quit);
        assert_eq!(Command::from_str("exit").unwrap(), Command::Quit);
        assert_eq!(Command::from_str("q").unwrap(), Command::Quit);
        assert_eq!(Command::from_str("e").unwrap(), Command::Quit);
        assert_eq!(Command::from_str("2").unwrap(), Command::Quit);

        assert_eq!(Command::from_str("zmove").unwrap(), Command::AskZmove);
        assert_eq!(Command::from_str("z").unwrap(), Command::AskZmove);

        /*
        assert_eq!(Command::from_str("8,2").unwrap(), Command::Zmove(direction: Up, speed: 2));
        */
    }

    #[test]
    fn invalid_command_from_str() {
        assert_matches!(
            Command::from_str("unimaginative").unwrap_err(),
            BoardError::InvalidCommand(_)
        );
        assert_matches!(
            Command::from_str("36").unwrap_err(),
            BoardError::InvalidCommand(_)
        );
        assert_matches!(
            Command::from_str("I want 4 apples please").unwrap_err(),
            BoardError::FailedParse(_)
        );
        assert_matches!(
            Command::from_str("I, WANT, 4 , apples, PLEASE").unwrap_err(),
            BoardError::TooManyArguments(_)
        );
        assert_matches!(
            Command::from_str("2,5").unwrap_err(),
            BoardError::NotImplemented
        );
    }
}
