use super::{Board, Color};

/// the struct that represent Game settings
/// This is used to handle settings in a quick and compact way
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
