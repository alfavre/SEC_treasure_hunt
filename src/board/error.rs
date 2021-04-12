#[derive(Debug, PartialEq)]
pub enum BoardError {
    InvalidMove(String),
    InvalidCommand(String),
    FailedParse(String),
    Not2Dimensional(usize),
    NonNumeric,
    TooManyArguments(usize),
    NotImplemented,
}
