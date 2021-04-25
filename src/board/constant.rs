use super::{Board, Color};

/// the `Board`'s associated constants
/// they are pub(super) to stop main or any not Board thing to access it
impl Board {
    pub(super) const DEFAULT_BOARD_WIDTH: u32 = 15;
    pub(super) const DEFAULT_BOARD_HEIGHT: u32 = 15;

    pub(super) const BOARD_COLOR: Color = Color::White;

    pub(super) const WATER_TILE: char = '~';
    pub(super) const DEFAULT_PLAYER_TILE: char = '@';
    pub(super) const TREASURE_TILE: char = 'X';

    pub(super) const DEFAULT_SEED: u64 = 2;
    pub(super) const DEFAULT_PLAYER_COLOR: Color = Color::Red;

    pub(super) const MOVE_MAX_DISTANCE: u32 = 4;

    // special thanks to : https://regexr.com/
    // warning this regex accepts negative numbers (or nonsensical numbers like 3-4-5)
    pub(super) const PARENTHESIS_REGEX: &'static str =
        r"^([(]{1}[0-9,a-fxA-F\-]+[)]{1}$|[\[]{1}[0-9,a-fxA-F\-]+[\]]{1}$|[0-9,a-fxA-F\-]+$)";
}
