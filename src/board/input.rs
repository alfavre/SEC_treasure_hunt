use super::{BoardError, Color, Command, Position};
use read_input::prelude::*;

pub fn get_seed_setting() -> u64 {
    input()
        .msg("Please enter a new seed: ")
        .err("That's not a positive integer, [e.g. '2']: ")
        .get()
}
pub fn get_color_setting() -> Color {
    input()
        .msg("Please input your color.\nWARNING, the closer to blue the harder the game!\n[e.g. 'red', 'cyan', '2426' ,'23,144,643']: ")
        .err("That is not a legal color, try again [e.g. 'red', 'cyan', '2426' ,'23,144,643']: ")
        .get()
}
pub fn get_choice_setting() -> String {
    input().msg("Please input your choice: ").get()
}

pub fn get_position_for_teleport() -> Position {
    input()
        .msg("You can enter the position where you want to go \
        [e.g. '12,13' '[12,0xc]' '(0x12,14)'] \
        \n You can go to positions outside the board, as the board is a torus, they will be corrected.\
        \nEnter your choice: ")
        .err_match(|e| {
            Some(match e {
                BoardError::InvalidFormat(s) => format!("{}", s),
                BoardError::Not2Dimensional(u) => format!("Your value had {} dimensions instead of 2",u),
                BoardError::FailedParse(s) => format!("{}", s),
                _=> panic!("impossible error"),
            })

        }).get()
}

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
                _ => panic!("impossible error"),
            })
        })
        .get()
}

pub fn get_replay_choice() -> String {
    input()
        .repeat_msg("Please enter your choice [y/n]: ")
        .add_test(|x| *x == "yes" || *x == "y" || *x == "no" || *x == "n")
        .get()
}
