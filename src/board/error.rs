#[derive(Debug, PartialEq)]
pub enum BoardError {
    InvalidCommand(String),
    FailedParse(String),
    Not2Dimensional(usize),
    NonNumeric,
    TooManyArguments(usize),
    NotImplemented,
}
