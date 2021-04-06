/// I have no idea what is the best way to do this
/// adding a trait to tuple for my to_i64 fn
/// creating a named tuple implementing to_i64
/// creating a struct to make the field named too
/// I went with the named tuple as it seems to save more memory than the struct
#[derive(Debug)]
pub struct Position(pub u32, pub u32);

impl Position {
    /// utility cast that gives a i64 pair of the board_position
    /// this is the only reason why this struct exists
    /// if not &self as argument we have move problems
    pub fn to_i64(&self) -> (i64, i64) {
        (self.0 as i64, self.1 as i64)
    }
}

/// equal definition for my named tuple
///
/// I hoped that that wouldn't have been necessary
impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
