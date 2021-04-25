use super::{Board, Color};

/// the struct that represent Game settings
/// This is used to handle settings in a quick and compact way
#[derive(Debug, PartialEq)]
pub struct GameSettings {
    pub seed: u64,
    pub player_color: Color,
    pub player_tile: char,
}

impl GameSettings {
    /// this grabs the default settings, it makes life easier
    ///
    /// # Returns
    /// * `GamesSettings` - the default game settings, in a compact and easy struct
    pub fn get_default_settings() -> GameSettings {
        GameSettings {
            seed: Board::DEFAULT_SEED,
            player_color: Board::DEFAULT_PLAYER_COLOR,
            player_tile: Board::DEFAULT_PLAYER_TILE,
        }
    }
}
