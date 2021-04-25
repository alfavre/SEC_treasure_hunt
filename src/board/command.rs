use super::{assert_matches, BoardError, FromStr, Zmove};

/// A Command used to determine what should be done
/// I don't know how to comment an enum
#[derive(Debug, PartialEq)]
pub enum Command {
    //ShowBoard,
    AskTeleport,
    AskZmove,
    Zmove(Zmove), // secret command
    Search,
    Quit,
}

impl FromStr for Command {
    type Err = BoardError;

    /// this is the implementation of from str for commands
    ///
    /// # Arguments
    /// * `s` - the str from which will deduce our command
    ///
    /// # Returns
    /// * `Command` - the command extracted from string s
    /// * `BoardError::InvalidCommand(String)` - when the str is parsed but not what we want
    /// * `BoardError::FailedParse(String)` - when the parsing failed like when we parse for u32 but get a negative number
    /// * `BoardError::TooManyArguments(usize)` - when the number of arguments separeted by ',' is bigger than 2
    /// * `BoardError::NotImplemented` - temporary error for unimplemented things
    /// * `BoardError::InvalidMove` - if the quick zmove isn't correct
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean_s = s.trim().replace(|x| x == ' ', ""); //we got rid of spaces

        if !clean_s
            .contains(|c| ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', ','].contains(&c))
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
            // this is the zmove shortcut
            let zmove_to_return = Zmove::from_str(clean_s.as_str());
            match zmove_to_return {
                Ok(zmove) => Ok(Command::Zmove(zmove)),
                Err(err) => Err(err),
            }
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
            Command::from_str(" m O v E ").unwrap(),
            Command::AskTeleport
        );

        assert_eq!(Command::from_str("0").unwrap(), Command::AskTeleport);

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

        // more zmove case are tested in zmove.rs
        assert_eq!(Command::from_str("8,1").unwrap(), Command::Zmove(Zmove::new(8,1).unwrap()));
        assert_eq!(Command::from_str("0x8,0x1").unwrap(), Command::Zmove(Zmove::new(8,1).unwrap()));

        
    }

    #[test]
    fn invalid_command_from_str() {
        // these 4 were once valid, when i let the possibility to directly enter a destination
        // for simplification and streamlining this feature will probably never come back
        assert_matches!(
            Command::from_str("(move)").unwrap_err(),
            BoardError::InvalidCommand(_)
        );
        assert_matches!(
            Command::from_str("[move]").unwrap_err(),
            BoardError::InvalidCommand(_)
        );
        assert_matches!(
            Command::from_str("(0)").unwrap_err(),
            BoardError::FailedParse(_)
        );
        assert_matches!(
            Command::from_str("[0]").unwrap_err(),
            BoardError::FailedParse(_)
        );

        assert_matches!(
            Command::from_str("unimaginative").unwrap_err(),
            BoardError::InvalidCommand(_)
        );
        // if there is a number, but isn't one we need, invalid command and not failed parse (as the parse was successful)
        assert_matches!(
            Command::from_str("36").unwrap_err(),
            BoardError::InvalidCommand(_)
        );
        // if there is a number, it triggers the failed parse error
        assert_matches!(
            Command::from_str("I want 4 apples please").unwrap_err(),
            BoardError::FailedParse(_)
        );
        assert_matches!(
            Command::from_str("I, WANT, 4 , apples, PLEASE").unwrap_err(),
            BoardError::TooManyArguments(_)
        );
        assert_matches!(
            Command::from_str("3, 34, 4 , 54, -34").unwrap_err(),
            BoardError::TooManyArguments(_)
        );

        //zmoves errors, more are tested in zmove.rs

        assert_matches!(
            Command::from_str("hello,hello").unwrap_err(),
            BoardError::FailedParse(_)
        );

        assert_matches!(
            Zmove::from_str("0,1").unwrap_err(),
            BoardError::InvalidMove(_)
        );
    }
}
