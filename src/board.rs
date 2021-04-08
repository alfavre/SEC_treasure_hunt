/*! Visual only `Board` functions

#
Add the missing part (`// TODO`).

You are free to modify anything, including the function parameters,
the code is provided as a support if desired.
*/

mod constant;
mod display;
mod error;
mod util;

use constant::*;
use display::*;
use error::*;
use matches::assert_matches;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use read_input::prelude::*;
use std::str::FromStr;
use termcolor::Color;
use util::{GameSettings, Position};

#[derive(Debug)]
pub struct Board {
    player_color: Color,
    player_coordinates: Position,
    treasure_coordinates: Position,
    rng: rand::prelude::StdRng,
    player_tile: char,
    //those 2 are hella dangereous, it's better to ignore them completely
    //board_width: u32,
    //board_height: u32,
}

/// where I hid all my `Board`'s function's implementation
impl Board {
    /// Gets a new pair of random coordinates
    ///
    /// respects the board proportions
    /// this method is static, it uses the rng generator given in argument
    /// It's static for constructor reasons, you should always pass the board's rng field
    ///
    /// # Arguments
    ///
    /// * `rng` - a mutable reference to the rand::prelude::StdRng used to get the coordinates
    ///
    /// # Returns
    ///
    /// * A u32 pair that have been modulated to fit in the Board
    fn random_coordinates(rng: &mut rand::prelude::StdRng) -> Position {
        // the cast is needed for coordinate_modulo
        Board::coordinate_modulo((rng.next_u32() as i64, rng.next_u32() as i64))
    }

    /// Sets the player coordinate to the one given in argument
    ///
    /// Mod is applied to simulate a torus on the board
    /// This doesn't need the c
    /// movement distance is currently not verified
    ///
    /// the coordinate pair is a i64 instead of a u32 to take advantage on torus properties of the board
    ///
    /// # Arguments
    ///
    /// * `i64_coordinates` - a i64 pair representing a posiiton
    fn set_player_coordinates(&mut self, i64_coordinates: (i64, i64)) -> () {
        self.player_coordinates = Board::coordinate_modulo(i64_coordinates);
    }

    /// Applies a mod of width and height on the given coordinate
    ///
    /// Supports all rectangle and torus boards (and all possible forms that are bijection of rectangle)
    /// static method
    ///
    /// the modulo used is the rem_euclid() as we need -5 mod 3 = 1 and not -5 mod 3 = -2
    /// therefore always giving us non-negatives results as the left hand operand is always non-negative
    ///
    /// # Arguments
    ///
    /// * `i64_pair` - the i64 pair that will be modulated to become a coordinate pair, we use i64 as all u32 can fit in it
    ///
    /// # Returns
    /// * A Position that fits in the board
    fn coordinate_modulo(i64_pair: (i64, i64)) -> Position {
        Position {
            x: (i64_pair.0.rem_euclid(Board::DEFAULT_BOARD_WIDTH as i64)) as u32,
            y: (i64_pair.1.rem_euclid(Board::DEFAULT_BOARD_HEIGHT as i64)) as u32,
        }
    }

    /// basic default constructor
    ///
    /// creates a new board
    ///
    /// # Arguments
    ///
    /// * `seed` - a u64 seed to "fix" the rng of the created board
    ///
    /// # Returns
    ///
    /// * a new Board instance
    fn new(game_settings: GameSettings) -> Board {
        let mut rng_to_move = StdRng::seed_from_u64(game_settings.seed); // not suitable for crypto, but this isn't crypto
        Board {
            player_color: game_settings.player_color,
            player_tile: game_settings.player_tile,
            player_coordinates: Board::random_coordinates(&mut rng_to_move),
            treasure_coordinates: Board::random_coordinates(&mut rng_to_move),
            rng: rng_to_move, // the rng is moved here
        }
    }

    /*
    pub fn play_game() -> Result<(),std::io::Error> {

        let mut this_board : Board = Board::init_game();


        let mut game_terminated : bool = false;

        while !game_terminated {
            let mut game_over: bool = false;
            while !game_over{
                this_board::play_turn(&mut game_over);
            }
            this_board::end_of_game(&mut game_terminated);
        }

        Ok(()) // the game ended normally
    } */

    /// put not public when the time is right
    pub fn init_game() -> Board {
        let mut game_settings = GameSettings::get_default_settings();
        let mut is_setting_over = false;

        Board::print_init();

        while !is_setting_over {
            Board::print_game_settings(&game_settings);

            let choice: String = input().msg("Please input your choice: ").get();

            match choice.as_str() {
                "0" => game_settings.seed = Board::get_seed_setting(),
                "1" => game_settings.player_color = Board::get_color_setting(),
                "2" => println!("not implemented"),
                "3" => println!("not implemented"),
                "4" => println!("not implemented"),
                "d" | "default" => game_settings = GameSettings::get_default_settings(),
                _ => is_setting_over = true,
            }
        }

        // settings are over, init board
        Board::new(game_settings)
    }

    pub fn get_seed_setting() -> u64 {
        input()
            .msg("Please enter a new seed: ")
            .err("That's not a positive integer, [e.g. '2']: ")
            .get()
    }
    pub fn get_color_setting() -> Color {
        input()
            .msg("Please input your color [e.g. 'red', 'cyan', '2426' ,'23,144,643']: ")
            .err("That is not a legal color, try again [e.g. 'red', 'cyan', '2426' ,'23,144,643']:")
            .get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_modulo_in_board() {
        // If I put them in a vec it will be faster, but then they wouldn't be named
        let bottom_left = Position { x: 0, y: 0 };
        let bottom_right = Position {
            x: Board::DEFAULT_BOARD_WIDTH - 1,
            y: 0,
        };
        let top_left = Position {
            x: 0,
            y: Board::DEFAULT_BOARD_HEIGHT - 1,
        };
        let top_right = Position {
            x: Board::DEFAULT_BOARD_WIDTH - 1,
            y: Board::DEFAULT_BOARD_HEIGHT - 1,
        };
        let somewhere_inside = Position {
            x: (Board::DEFAULT_BOARD_WIDTH - 1) / 2,
            y: (Board::DEFAULT_BOARD_HEIGHT - 1) / 2,
        };

        assert_eq!(Board::coordinate_modulo(bottom_left.to_i64()), bottom_left);
        assert_eq!(
            Board::coordinate_modulo(bottom_right.to_i64()),
            bottom_right
        );
        assert_eq!(Board::coordinate_modulo(top_left.to_i64()), top_left);
        assert_eq!(Board::coordinate_modulo(top_right.to_i64()), top_right);
        assert_eq!(
            Board::coordinate_modulo(somewhere_inside.to_i64()),
            somewhere_inside
        );
    }

    #[test]
    fn coordinate_modulator_out_of_board() {
        let bottom_left_and_one_down: (i64, i64) = (0, -1);
        let bottom_left_and_one_left: (i64, i64) = (-1, 0);
        let bottom_left_and_diagonal_out: (i64, i64) = (-1, -1);

        let bottom_right_and_one_right: (i64, i64) = (Board::DEFAULT_BOARD_WIDTH as i64, 0);
        let bottom_right_and_one_down: (i64, i64) = ((Board::DEFAULT_BOARD_WIDTH - 1) as i64, -1);
        let bottom_right_and_diagonal_out: (i64, i64) = (Board::DEFAULT_BOARD_WIDTH as i64, -1);

        let top_left_and_one_up: (i64, i64) = (0, Board::DEFAULT_BOARD_HEIGHT as i64);
        let top_left_and_one_left: (i64, i64) = (-1, (Board::DEFAULT_BOARD_HEIGHT - 1) as i64);
        let top_left_and_diagonal_out: (i64, i64) = (-1, Board::DEFAULT_BOARD_HEIGHT as i64);

        let top_right_and_one_up: (i64, i64) = (
            (Board::DEFAULT_BOARD_WIDTH - 1) as i64,
            Board::DEFAULT_BOARD_HEIGHT as i64,
        );
        let top_right_and_one_right: (i64, i64) = (
            Board::DEFAULT_BOARD_WIDTH as i64,
            (Board::DEFAULT_BOARD_HEIGHT - 1) as i64,
        );
        let top_right_and_one_diagonal_out: (i64, i64) = (
            Board::DEFAULT_BOARD_WIDTH as i64,
            Board::DEFAULT_BOARD_HEIGHT as i64,
        );

        // multiplicator should always be positive
        // it's not unsigned int here to I save my self writting `as i64` everywhere
        let multiplicator: i64 = 5;
        let oob_quadrant_1: (i64, i64) = (
            (Board::DEFAULT_BOARD_WIDTH as i64) * multiplicator,
            (Board::DEFAULT_BOARD_HEIGHT as i64) * multiplicator,
        );
        let oob_quadrant_2: (i64, i64) = (
            -(Board::DEFAULT_BOARD_WIDTH as i64) * multiplicator,
            (Board::DEFAULT_BOARD_HEIGHT as i64) * multiplicator,
        );
        let oob_quadrant_3: (i64, i64) = (
            -(Board::DEFAULT_BOARD_WIDTH as i64) * multiplicator,
            -(Board::DEFAULT_BOARD_HEIGHT as i64) * multiplicator,
        );
        let oob_quadrant_4: (i64, i64) = (
            (Board::DEFAULT_BOARD_WIDTH as i64) * multiplicator,
            -(Board::DEFAULT_BOARD_HEIGHT as i64) * multiplicator,
        );

        let oob_quadrant_1_max: (i64, i64) = (i64::MAX, i64::MAX);
        let oob_quadrant_2_min_max: (i64, i64) = (i64::MIN, i64::MAX);
        let oob_quadrant_3_min: (i64, i64) = (i64::MIN, i64::MIN);
        let oob_quadrant_4_max_min: (i64, i64) = (i64::MAX, i64::MIN);

        let bottom_left = Position { x: 0, y: 0 };
        let bottom_right = Position {
            x: Board::DEFAULT_BOARD_WIDTH - 1,
            y: 0,
        };
        let top_left = Position {
            x: 0,
            y: Board::DEFAULT_BOARD_HEIGHT - 1,
        };
        let top_right = Position {
            x: Board::DEFAULT_BOARD_WIDTH - 1,
            y: Board::DEFAULT_BOARD_HEIGHT - 1,
        };

        assert_eq!(
            Board::coordinate_modulo(bottom_left_and_one_left),
            bottom_right
        );
        assert_eq!(Board::coordinate_modulo(bottom_left_and_one_down), top_left);
        assert_eq!(
            Board::coordinate_modulo(bottom_left_and_diagonal_out),
            top_right
        );

        assert_eq!(
            Board::coordinate_modulo(bottom_right_and_one_right),
            bottom_left
        );
        assert_eq!(
            Board::coordinate_modulo(bottom_right_and_one_down),
            top_right
        );
        assert_eq!(
            Board::coordinate_modulo(bottom_right_and_diagonal_out),
            top_left
        );

        assert_eq!(Board::coordinate_modulo(top_left_and_one_left), top_right);
        assert_eq!(Board::coordinate_modulo(top_left_and_one_up), bottom_left);
        assert_eq!(
            Board::coordinate_modulo(top_left_and_diagonal_out),
            bottom_right
        );

        assert_eq!(Board::coordinate_modulo(top_right_and_one_up), bottom_right);
        assert_eq!(Board::coordinate_modulo(top_right_and_one_right), top_left);
        assert_eq!(
            Board::coordinate_modulo(top_right_and_one_diagonal_out),
            bottom_left
        );

        assert_eq!(Board::coordinate_modulo(oob_quadrant_1), bottom_left);
        assert_eq!(Board::coordinate_modulo(oob_quadrant_2), bottom_left);
        assert_eq!(Board::coordinate_modulo(oob_quadrant_3), bottom_left);
        assert_eq!(Board::coordinate_modulo(oob_quadrant_4), bottom_left);

        // a Position has 2 u32, therefore always >0
        assert!(
            Board::coordinate_modulo(oob_quadrant_1_max).x < Board::DEFAULT_BOARD_WIDTH
                && Board::coordinate_modulo(oob_quadrant_1_max).y < Board::DEFAULT_BOARD_HEIGHT,
            "Is not in board"
        );

        assert!(
            Board::coordinate_modulo(oob_quadrant_2_min_max).x < Board::DEFAULT_BOARD_WIDTH
                && Board::coordinate_modulo(oob_quadrant_2_min_max).y < Board::DEFAULT_BOARD_HEIGHT,
            "Is not in board"
        );

        assert!(
            Board::coordinate_modulo(oob_quadrant_3_min).x < Board::DEFAULT_BOARD_WIDTH
                && Board::coordinate_modulo(oob_quadrant_3_min).y < Board::DEFAULT_BOARD_HEIGHT,
            "Is not in board"
        );

        assert!(
            Board::coordinate_modulo(oob_quadrant_4_max_min).x < Board::DEFAULT_BOARD_WIDTH
                && Board::coordinate_modulo(oob_quadrant_4_max_min).y < Board::DEFAULT_BOARD_HEIGHT,
            "Is not in board"
        );
    }

    #[test]
    fn random_coordinates_same_seed_same_result() {
        let test_seed: u64 = 12;
        // I dont test if it's in board here, as random_cooridinates calls coordinate_modulo, which is tested in another test
        assert_eq!(
            Board::random_coordinates(&mut StdRng::seed_from_u64(test_seed)),
            Board::random_coordinates(&mut StdRng::seed_from_u64(test_seed))
        );
    }

    #[test]
    /// I don't test if it's in board here as it's tested in the coordinate modulator tests
    fn set_player_coordinates_works() {
        let mut test_board = Board::new(GameSettings::get_default_settings());
        let test_position = Position {
            x: (Board::DEFAULT_BOARD_WIDTH - 1) / 2,
            y: (Board::DEFAULT_BOARD_HEIGHT - 1) / 2,
        };
        test_board.set_player_coordinates(test_position.to_i64());
        assert_eq!(test_board.player_coordinates, test_position);
    }
}
