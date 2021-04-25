// special thanks to : https://regexr.com/
// warning this regex accepts negative numbers (or nonsensical numbers like 3-4-5)

#[derive(Debug, PartialEq)]
pub enum BoardError {
    InvalidMove(String),     // when there is a move but is not a legal one
    InvalidCommand(String),  // when there is word but is not a command
    InvalidFormat(String),   // when the regex checks fails
    FailedParse(String),     // when the value couldn't be parsed
    Not2Dimensional(usize),  // when the number of dimension isn't 2
    TooManyArguments(usize), // when there are too many arguments
}
