use super::{assert_matches, BoardError, Direction, FromStr};

#[derive(Debug, PartialEq)]
pub struct Zmove {
    direction: Direction,
    speed: u32,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    //ShowBoard,
    AskTeleport,
    AskZmove,
    Zmove(Zmove), // secret command
    Search,
    Quit,
}

/// this is the implementation for general Command
/// # Errors
///
/// * `BoardError::InvalidCommand(String)` - when the str is parsed but not what we want
/// * `BoardError::FailedParse(String)` - when the parsing failed like when we parse for u32 but get a negative number
/// * `BoardError::TooManyArguments(usize)` - when the number of arguments separeted by ',' is bigger than 2
/// * `BoardError::NotImplemented` - temporary error for unimplemented things
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

#[cfg(test)]
mod tests {
    use super::*;
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
            // this will be implemented in
            Command::from_str("2,5").unwrap_err(),
            BoardError::NotImplemented
        );
    }
}
