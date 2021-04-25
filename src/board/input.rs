use super::{BoardError, Color, Command, Position};
use read_input::prelude::*;

/// a method to get a user submitted seed value
/// this method only stop when a correct seed is given
///
/// # Returns
/// * `u64` - the value for the seed
pub fn get_seed_setting() -> u64 {
    input()
        .msg("Please enter a new seed: ")
        .err("That's not a positive integer, [e.g. '2']: ")
        .get()
}

/// a method to get a user submitted color value
/// this method only stops when a correct color is given
/// it uses the from str method of the Color struct
/// that from str method wasn't done by me, it comes with the termcolor crate
///
/// # Returns
/// * `Color` - the color given by the player
pub fn get_color_setting() -> Color {
    input()
        .msg("Please input your color.\nWARNING, the closer to blue the harder the game!\n[e.g. 'red', 'cyan', '2426' ,'23,144,643']: ")
        .err("That is not a legal color, try again [e.g. 'red', 'cyan', '2426' ,'23,144,643']: ")
        .get()
}

/// a method to get a user submitted tile in `char` format
///
/// # Returns
/// * `char` - the tile given by the player
pub fn get_tile_setting() -> char {
    input()
        .msg("Please input the tile that will represent you.\nWARNING, the closer to the '~' char, the harder the game!\n[e.g. 'r', '#', '☺' ,'A']: ")
        .err("That is not a char, try again [e.g. 'r', '#', '☺' ,'A']: ")
        .get()
}

/// a method to get a user submitted setting choice
/// this choice is used to determine whiche setting
/// the user wants to change
/// see display::print_game_settings to see the list of
/// choosable parameter, the input control for this input is done
/// at the board level
///
/// # Returns
/// * `String` - the user submitted choice
pub fn get_choice_setting() -> String {
    input().msg("Please input your choice: ").get()
}

/// a method to get a user submitted Position
/// this uses the from str method from position
///
/// A position can be outside of the board, it will be modulated to fit on it
/// this panics if an unexpected error arise
///
/// # Returns
/// * `Position` - the position where the user wants to be teleported
pub fn get_position_for_teleport() -> Position {
    input()
        .msg("You can enter the position where you want to go \
        [e.g. '12,13' '[12,0xc]' '(0x12,14)'] \
        \n You can go to positions outside of the board, as the board is a torus, they will be corrected.\
        \nEnter your choice: ")
        .err_match(|e| {
            Some(match e {
                BoardError::InvalidFormat(s) => format!("{}", s),
                BoardError::Not2Dimensional(u) => format!("Your value had {} dimension(s) instead of 2",u),
                BoardError::FailedParse(s) => format!("{}", s),
                _=> panic!("impossible error"),
            })

        }).get()
}

/// a method to get a user submitted Command
/// this uses the from str method from command
/// this panics if an unexpected error arise
///
/// # Returns
/// * `Command` - the command the will decide the next step
pub fn get_choice_command() -> Command {
    input()
        .msg("Please enter your action [e.g. m, 2, search]: ")
        .err_match(|e| {
            Some(match e {
                BoardError::InvalidCommand(s) => format!("{}", s),
                BoardError::FailedParse(s) => format!("{}", s),
                BoardError::TooManyArguments(u) => format!("{} is too many elements", u),
                BoardError::NotImplemented => {
                    format!("this feature doesn't exist right now, sorry")
                }
                BoardError::InvalidMove(s) => format!("{}", s),
                _ => panic!("impossible error"),
            })
        })
        .get()
}

/// a method to get a user submitted yes or no
/// no uppercase allowed
///
/// # Returns
/// * `String` - `y`, `n`, `no` or `yes` only
pub fn get_yes_no_choice() -> String {
    input()
        .repeat_msg("Please enter your choice [y/n]: ")
        .add_test(|x| *x == "yes" || *x == "y" || *x == "no" || *x == "n")
        .get()
}
