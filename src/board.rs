/*! Visual only `Board` functions

#
Add the missing part (`// TODO`).

You are free to modify anything, including the function parameters,
the code is provided as a support if desired.
*/

mod command;
mod constant;
mod direction;
mod display;
mod error;
mod game_settings;
mod input;
mod position;

use command::*;
use constant::*;
use direction::*;
use display::*;
use error::*;
use input::*;

use game_settings::GameSettings;
use position::Position;

use matches::assert_matches;
use rand::{rngs::StdRng, RngCore, SeedableRng};
use regex::Regex;
use std::str::FromStr;
use termcolor::Color;

/// the board structure that is the basis of all the treaure hunt
///
/// # Attributes
/// * `player_color` - the color representing the player, the closer to blue, the harder the game
/// * `player_coordinates` - the position of the player on the board
/// * `treasure_coordinates` - the treasure position on the board
/// * `rng` - the standar RNG used to fix randomness during a game
/// * `player_tile` - the char that will represent the user on the map (when it was a str you could enter emojis)
/// * `tracker` - the 2d bool map of where the player has already searched
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
    tracker: Vec<Vec<bool>>,
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
    /// **Mod is applied to simulate a torus on the board**
    /// movement distance is not verified here
    ///
    /// the coordinate pair is a i64 instead of a u32 to take advantage on torus properties of the board
    /// days after the previous sentence, it seems i64 were never really used
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

    fn is_in_board(position: &Position) -> bool {
        (position.x < Board::DEFAULT_BOARD_WIDTH) && (position.y < Board::DEFAULT_BOARD_HEIGHT)
    }

    /// basic default constructor
    ///
    /// creates a new board
    ///
    /// # Arguments
    ///
    /// * `game_settings:` - a GameSettings that completely define the starting state of the game
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
            tracker: vec![
                vec![false; Board::DEFAULT_BOARD_HEIGHT as usize];
                Board::DEFAULT_BOARD_WIDTH as usize
            ],
        }
    }

    /// the starting point of the game, from there the entire workflow will be executed
    /// starting from the settings selection and finishing with a goodbye
    /// this should be the only board public method
    ///
    /// # Returns
    /// * `Ok` - if game closed normally
    /// * `Err` - if the game did not work properly
    pub fn play_game() -> Result<(), std::io::Error> {
        //while game not closing start a new game
        let mut is_game_closing: bool = false;
        while !is_game_closing {
            let mut this_board: Board = Board::init_game();

            // while game is not over play turn
            let mut is_game_over: bool = false;
            while !is_game_over {
                is_game_over = this_board.play_turn();
            }

            is_game_closing = this_board.end_of_game();
        }
        display::print_goodbye();
        Ok(()) // the game ended normally
    }

    /// the handling of the ending
    /// notably if a new game wil be started or
    /// if the game will close
    ///
    /// # Returns
    /// * `bool` - true if the game will close, false if a new game will be launched
    fn end_of_game(&self) -> bool {
        display::print_end_screen();

        match input::get_yes_no_choice().as_str() {
            "yes" | "y" => return false,
            "no" | "n" => return true,
            _ => panic!("an unexpected answer was given during the ending of the game"),
        }
    }

    /// the hendling of a turn
    /// notably the board printing
    /// the choice of this turn action and it's handling
    ///
    /// # Returns
    /// * `bool` - true if the current game is finished, true if it isn't
    fn play_turn(&mut self) -> bool {
        let mut will_game_end: bool = false;
        match self.print_game_board() {
            Ok(_) => (), //do nothing,
            Err(err) => println!(
                "The board printing failed, you are now playing blind sorry."
            ),
        }

        display::print_turn_command();

        match get_choice_command() {
            Command::AskTeleport => self.teleport(), // handle teleport input and logic
            Command::Search => will_game_end = self.search_player_position(), // handle search logic, might finish game
            Command::Quit => will_game_end = true,                            // game is now over
            Command::AskZmove => (),     // handle zmove input and logic
            Command::Zmove(zmove) => (), // handle zmove logic only
        }
        will_game_end
    }

    /// the handling of the teleport action
    /// teleport corresponds to the move command in the doc
    /// I decided to not call it a move, as it's a teleport
    fn teleport(&mut self) -> () {
        let mut is_position_validated = false;
        while !is_position_validated {
            //input move and recenter
            let mut target_position: Position = input::get_position_for_teleport();

            //verif if is oob
            if !Board::is_in_board(&target_position) {
                // oob handling
                let corrected_target_position = Board::coordinate_modulo(target_position.to_i64());
                display::print_special_corrector_message(
                    &target_position,           // oob value
                    &corrected_target_position, // ib value
                );

                target_position = corrected_target_position; // preparation if yes

                match input::get_yes_no_choice().as_str() {
                    "yes" | "y" => (),      // continue handling as nothing happened
                    "no" | "n" => continue, // this should restart the while loop
                    _ => panic!("an unexpected answer was given during the yes/no choice"),
                }
            }
            match self.teleport_logic(&target_position) {
                Ok(_) => is_position_validated = true,
                Err(BoardError::InvalidMove(s)) => println!("{}", s),
                Err(_) => panic!("impossible error"),
            }
        }

        //move done posiiton changed
    }

    /// if the teleport destination has been decided,
    /// this will apply the teleport, changing the state of the game
    /// except if the destination is too far
    /// 
    /// # Arguments
    /// * `target` - the target position where the teleport should be done
    /// 
    /// # Returns
    /// * `Ok(_)` - if the teleport could be done
    /// * `Err(BoardError::InvalidMove)` - if the distance of the teleport is too long, stopping the teleport.
    fn teleport_logic(&mut self, target: &Position) -> Result<(), BoardError> {
        // the target position will always be in board, even if not
        // the second point might be confusing but it's true
        if Position::is_dist_legal(
            self.player_coordinates.get_xy_dists(target),
            (Board::DEFAULT_BOARD_WIDTH, Board::DEFAULT_BOARD_HEIGHT),
        ) {
            //if legal do the move
            //set player coordinate will apply the modulus
            self.set_player_coordinates(target.to_i64());
            Ok(())
        } else {
            Err(BoardError::InvalidMove(
                "You can't do this move, it's too far".to_string(),
            ))
        }
    }

    /// this handles the search action
    /// it looks if the treasure is on the current player coordinate
    /// It will change the state of the game to end if the treasure is found
    /// if the treasure is not found, it will update the tracker and calculate the distance
    /// to the treasure
    /// 
    /// # Returns
    /// * `bool` - the boolean that tells if the game is won
    fn search_player_position(&mut self) -> bool {
        if self.player_coordinates == self.treasure_coordinates {
            display::print_win_screen();
            return true;
        }

        self.tracker[self.player_coordinates.x as usize][self.player_coordinates.y as usize] = true;

        let dist_to_tresure = Position::get_shortest_dist(
            self.player_coordinates
                .get_xy_dists(&self.treasure_coordinates),
        );
        display::print_found_nothing(dist_to_tresure);

        false
    }

    /// this is handle the game settings selection
    /// when the settings have been selected, it will generate the
    /// board for the game
    /// 
    /// # Returns
    /// * `Board` - the board for the game, with the user submitted settings
    fn init_game() -> Board {
        let mut game_settings = GameSettings::get_default_settings();
        let mut is_setting_over = false;

        display::print_init();

        while !is_setting_over {
            display::print_game_settings(&game_settings);

            match input::get_choice_setting().as_str() {
                "0" => game_settings.seed = input::get_seed_setting(),
                "1" => game_settings.player_color = input::get_color_setting(),
                "2" => game_settings.player_tile = input::get_tile_setting(),
                "3" => println!("not implemented"),
                "4" => println!("not implemented"),
                "d" | "default" => game_settings = GameSettings::get_default_settings(),
                _ => is_setting_over = true,
            }
        }

        // settings are over, init board
        Board::new(game_settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_position_in_board() {
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

        //verify if the positions are in board
        assert!(Board::is_in_board(&bottom_left));
        assert!(Board::is_in_board(&bottom_right));
        assert!(Board::is_in_board(&top_left));
        assert!(Board::is_in_board(&top_right));
        assert!(Board::is_in_board(&somewhere_inside));
    }

    #[test]
    fn invalid_position_in_board() {
        // as we verify positions, there are no negative value, the only quadrant verified is the first one

        let bottom_right_and_one_right = Position {
            x: Board::DEFAULT_BOARD_WIDTH,
            y: 0,
        };

        let top_left_and_one_up = Position {
            x: 0,
            y: Board::DEFAULT_BOARD_HEIGHT,
        };

        let top_right_and_one_up = Position {
            x: (Board::DEFAULT_BOARD_WIDTH - 1),
            y: Board::DEFAULT_BOARD_HEIGHT,
        };
        let top_right_and_one_right = Position {
            x: Board::DEFAULT_BOARD_WIDTH,
            y: (Board::DEFAULT_BOARD_HEIGHT - 1),
        };
        let top_right_and_one_diagonal_out = Position {
            x: Board::DEFAULT_BOARD_WIDTH,
            y: Board::DEFAULT_BOARD_HEIGHT,
        };

        // arbitrary value, it should be bigger than 1 but the result must be under u32:MAX
        let multiplicator: u32 = 5;

        // oob means out of bounds
        let oob_diagonal = Position {
            x: (Board::DEFAULT_BOARD_WIDTH) * multiplicator,
            y: (Board::DEFAULT_BOARD_HEIGHT) * multiplicator,
        };

        let oob_up = Position {
            x: 0,
            y: (Board::DEFAULT_BOARD_HEIGHT) * multiplicator,
        };

        let oob_right = Position {
            x: (Board::DEFAULT_BOARD_WIDTH) * multiplicator,
            y: 0,
        };

        let oob_diagonal_max = Position {
            x: u32::MAX,
            y: u32::MAX,
        };
        let oob_up_max = Position { x: 0, y: u32::MAX };
        let oob_right_max = Position { x: u32::MAX, y: 0 };

        assert!(!Board::is_in_board(&bottom_right_and_one_right));
        assert!(!Board::is_in_board(&top_left_and_one_up));
        assert!(!Board::is_in_board(&top_right_and_one_up));
        assert!(!Board::is_in_board(&top_right_and_one_right));
        assert!(!Board::is_in_board(&top_right_and_one_diagonal_out));
        assert!(!Board::is_in_board(&oob_diagonal));
        assert!(!Board::is_in_board(&oob_up));
        assert!(!Board::is_in_board(&oob_right));
        assert!(!Board::is_in_board(&oob_diagonal_max));
        assert!(!Board::is_in_board(&oob_up_max));
        assert!(!Board::is_in_board(&oob_right_max));
    }

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

        // verify if modulator works for in board positions
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
        // arbitrary value, it should be bigger than 1 but the result must be under i64:MAX
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
        // contrary to the others, I dont know where exactly those will land, so I just check if they're in board
        assert!(
            Board::is_in_board(&Board::coordinate_modulo(oob_quadrant_1_max)),
            "should be in board"
        );

        assert!(
            Board::is_in_board(&Board::coordinate_modulo(oob_quadrant_2_min_max)),
            "should be in board"
        );

        assert!(
            Board::is_in_board(&Board::coordinate_modulo(oob_quadrant_3_min)),
            "should be in board"
        );

        assert!(
            Board::is_in_board(&Board::coordinate_modulo(oob_quadrant_4_max_min)),
            "should be in board"
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

    #[test]
    fn search_no_treasure() {
        let mut test_board = Board::new(GameSettings::get_default_settings());
        assert!(!test_board.search_player_position());
        test_board.set_player_coordinates(Position { x: 0, y: 0 }.to_i64());
        assert!(!test_board.search_player_position());
        test_board.set_player_coordinates(Position { x: 0, y: 1 }.to_i64());
        assert!(!test_board.search_player_position());
        test_board.set_player_coordinates(Position { x: 0, y: 2 }.to_i64());
        assert!(!test_board.search_player_position());
        test_board.set_player_coordinates(Position { x: 0, y: 3 }.to_i64());
        assert!(!test_board.search_player_position());
        test_board.set_player_coordinates(Position { x: 1, y: 3 }.to_i64());
        assert!(!test_board.search_player_position());
        test_board.set_player_coordinates(Position { x: 2, y: 3 }.to_i64());
        assert!(!test_board.search_player_position());
        test_board.set_player_coordinates(
            Position {
                x: 0,
                y: Board::DEFAULT_BOARD_HEIGHT - 1,
            }
            .to_i64(),
        );
        assert!(!test_board.search_player_position());
        test_board.set_player_coordinates(
            Position {
                x: 0,
                y: Board::DEFAULT_BOARD_HEIGHT - 2,
            }
            .to_i64(),
        );
        assert!(!test_board.search_player_position());
        test_board.set_player_coordinates(
            Position {
                x: 0,
                y: Board::DEFAULT_BOARD_HEIGHT - 3,
            }
            .to_i64(),
        );

        assert!(test_board.print_game_board().is_ok());
    }


}
