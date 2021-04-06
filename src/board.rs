/*! Visual only `Board` functions

#
Add the missing part (`// TODO`).

You are free to modify anything, including the function parameters,
the code is provided as a support if desired.
*/
mod util;

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use std::io::{self, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use util::Position;

#[derive(Debug)]
pub struct Board {
    player_color: Color,
    player_coordinates: Position,
    treasure_coordinates: Position,
    rng: rand::prelude::StdRng,
}

/// the `Board`'s associated constants
impl Board {
    //should those 2 be usize instead ?
    const BOARD_WIDTH: u32 = 15;
    const BOARD_HEIGHT: u32 = 15;

    const BOARD_COLOR: Color = Color::White;

    // I could technically change them to &str but then I have to look into lifetimes
    const WATER_TILE: char = '~';
    const PLAYER_TILE: char = '@';
    const TREASURE_TILE: char = 'X';

    const DEFAULT_SEED: u64 = 2;
    const DEFAULT_PLAYER_COLOR: Color = Color::Red;
}

/// where I hid all my `Board`'s function's implementation
impl Board {
    /// Prints the `Board` to `stdout`.
    ///
    /// When the function returns, the terminal color is whatever a gremling decided.
    /// This functions requires definition of the `BOARD_WIDTH`, `BOARD_HEIGHT` and `BOARD_COLOR` constants
    ///
    /// # Returns
    ///
    /// A io::Result i don't know what this is
    /// Note: The actual definition of Write uses io::Result, which is just a synonym for Result<T, io::Error>.
    /// if an error is fished it's transmitted
    fn print(&self) -> io::Result<()> {
        let bufwtr = BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwtr.buffer();

        // Top row
        buffer.set_color(ColorSpec::new().set_fg(Some(Board::BOARD_COLOR)))?;
        write!(&mut buffer, "{:>4}", "⌜")?;
        for _ in 0..Board::BOARD_WIDTH {
            write!(&mut buffer, "⎺-⎺")?;
        }
        writeln!(&mut buffer, "⌝")?;

        // Main grid
        for y in (0..Board::BOARD_HEIGHT).rev() {
            write!(&mut buffer, "{:>2} ∣", y)?; // Side coordinates

            for x in 0..Board::BOARD_WIDTH {
                //TODO dont forget to make the treasure invisble in the realese
                if x == self.player_coordinates.0 && y == self.player_coordinates.1 {
                    Board::tile_painter(
                        &mut buffer,
                        self.player_color,
                        String::from(Board::PLAYER_TILE),
                    )?;
                } else if x == self.treasure_coordinates.0 && y == self.treasure_coordinates.1 {
                    Board::tile_painter(
                        &mut buffer,
                        termcolor::Color::Yellow,
                        String::from(Board::TREASURE_TILE),
                    )?;
                } else {
                    Board::tile_painter(
                        &mut buffer,
                        termcolor::Color::Blue,
                        String::from(Board::WATER_TILE),
                        // considering creating a painted_tile struct that encapsulate a "tile" in 2d videogame terms
                    )?;
                }
                // we dont forget to restore the buffer to white for the borders
                buffer.set_color(ColorSpec::new().set_fg(Some(Board::BOARD_COLOR)))?;
            }

            writeln!(&mut buffer, "∣")?; // Side column
        }

        // Bottom row
        write!(&mut buffer, "{:>4}", "⌞")?;
        for _ in 0..Board::BOARD_WIDTH {
            write!(&mut buffer, "_⎽_")?;
        }
        writeln!(&mut buffer, "⌟")?;

        // Bottom coordinates
        write!(&mut buffer, "{:4}", "")?;
        for x in 0..Board::BOARD_WIDTH {
            write!(&mut buffer, "{:^3}", x)?;
        }
        writeln!(&mut buffer)?;

        // print function ends here, we restore the buffer color to the "normal one"
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?; // I don't know how I can find the default terminal color, so it's green now
        return bufwtr.print(&buffer);
    }

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
        Position(
            (i64_pair.0.rem_euclid(Board::BOARD_WIDTH as i64)) as u32,
            (i64_pair.1.rem_euclid(Board::BOARD_HEIGHT as i64)) as u32,
        )
    }

    /// Paints the given tile in the given color for the board print function
    ///
    /// Tile is a String and not a str because I have difficulties with str.
    /// It's not a char either so the tile can be longer than one character.
    ///
    /// # Arguments
    /// * `buffer` - a mutable reference to the termcolor::Buffer that will be written to
    /// * `color` - a termcolor::Color that will be used for the text written in the buffer
    /// * `tile` - the String representing the tile that will be written once in the buffer
    ///
    /// # Returns
    /// * A Result containing either a "void" or an error
    fn tile_painter(buffer: &mut termcolor::Buffer, color: Color, tile: String) -> io::Result<()> {
        buffer.set_color(ColorSpec::new().set_fg(Some(color)))?;
        write!(buffer, "{:^3}", tile)?;
        Ok(())
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
    pub fn new(seed: u64) -> Board {
        let mut rng_to_move = StdRng::seed_from_u64(seed); // not suitable for crypto, but this isn't crypto
        Board {
            player_color: Color::Red, // TODO when this becomes private, add color as argument
            player_coordinates: Board::random_coordinates(&mut rng_to_move),
            treasure_coordinates: Board::random_coordinates(&mut rng_to_move),
            rng: rng_to_move, // the rng is moved here
        }
    }

    /*
    pub fn play_game() -> Result<(),std::io::Error> {

        let mut this_board : Board;

        Board::init_game(&mut this_board);

        let mut game_terminated : bool = false;

        while !game_terminated {
            let mut game_over: bool = false;
            while !game_over{
                this_board::play_turn(&mut game_over);
            }
            this_board::end_of_game(&mut game_terminated);
        }

        Ok(()) // the game ended normally
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    /// first test, to test tests
    /// I actually have no idea how to test a something that write to console
    /// I don't know how to not print board
    #[test]
    fn print_test() {
        let test_board = Board::new(Board::DEFAULT_SEED);
        assert!(test_board.print().is_ok());
    }

    #[test]
    fn coordinate_modulo_in_board() {
        // If I put them in a vec it will be faster, but then they wouldn't be named
        let bottom_left = Position(0, 0);
        let bottom_right = Position(Board::BOARD_WIDTH - 1, 0);
        let top_left = Position(0, Board::BOARD_HEIGHT - 1);
        let top_right = Position(Board::BOARD_WIDTH - 1, Board::BOARD_HEIGHT - 1);
        let somewhere_inside =
            Position((Board::BOARD_WIDTH - 1) / 2, (Board::BOARD_HEIGHT - 1) / 2);

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

        let bottom_right_and_one_right: (i64, i64) = (Board::BOARD_WIDTH as i64, 0);
        let bottom_right_and_one_down: (i64, i64) = ((Board::BOARD_WIDTH - 1) as i64, -1);
        let bottom_right_and_diagonal_out: (i64, i64) = (Board::BOARD_WIDTH as i64, -1);

        let top_left_and_one_up: (i64, i64) = (0, Board::BOARD_HEIGHT as i64);
        let top_left_and_one_left: (i64, i64) = (-1, (Board::BOARD_HEIGHT - 1) as i64);
        let top_left_and_diagonal_out: (i64, i64) = (-1, Board::BOARD_HEIGHT as i64);

        let top_right_and_one_up: (i64, i64) =
            ((Board::BOARD_WIDTH - 1) as i64, Board::BOARD_HEIGHT as i64);
        let top_right_and_one_right: (i64, i64) =
            (Board::BOARD_WIDTH as i64, (Board::BOARD_HEIGHT - 1) as i64);
        let top_right_and_one_diagonal_out: (i64, i64) =
            (Board::BOARD_WIDTH as i64, Board::BOARD_HEIGHT as i64);

        // multiplicator should always be positive
        // it's not unsigned int here to I save my self writting `as i64` everywhere
        let multiplicator: i64 = 5;
        let oob_quadrant_1: (i64, i64) = (
            (Board::BOARD_WIDTH as i64) * multiplicator,
            (Board::BOARD_HEIGHT as i64) * multiplicator,
        );
        let oob_quadrant_2: (i64, i64) = (
            -(Board::BOARD_WIDTH as i64) * multiplicator,
            (Board::BOARD_HEIGHT as i64) * multiplicator,
        );
        let oob_quadrant_3: (i64, i64) = (
            -(Board::BOARD_WIDTH as i64) * multiplicator,
            -(Board::BOARD_HEIGHT as i64) * multiplicator,
        );
        let oob_quadrant_4: (i64, i64) = (
            (Board::BOARD_WIDTH as i64) * multiplicator,
            -(Board::BOARD_HEIGHT as i64) * multiplicator,
        );

        let oob_quadrant_1_max: (i64, i64) = (i64::MAX, i64::MAX);
        let oob_quadrant_2_min_max: (i64, i64) = (i64::MIN, i64::MAX);
        let oob_quadrant_3_min: (i64, i64) = (i64::MIN, i64::MIN);
        let oob_quadrant_4_max_min: (i64, i64) = (i64::MAX, i64::MIN);

        let bottom_left = Position(0, 0);
        let bottom_right = Position(Board::BOARD_WIDTH - 1, 0);
        let top_left = Position(0, Board::BOARD_HEIGHT - 1);
        let top_right = Position(Board::BOARD_WIDTH - 1, Board::BOARD_HEIGHT - 1);

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
            Board::coordinate_modulo(oob_quadrant_1_max).0 < Board::BOARD_WIDTH
                && Board::coordinate_modulo(oob_quadrant_1_max).1 < Board::BOARD_HEIGHT,
            "Is not in board"
        );

        assert!(
            Board::coordinate_modulo(oob_quadrant_2_min_max).0 < Board::BOARD_WIDTH
                && Board::coordinate_modulo(oob_quadrant_2_min_max).1 < Board::BOARD_HEIGHT,
            "Is not in board"
        );

        assert!(
            Board::coordinate_modulo(oob_quadrant_3_min).0 < Board::BOARD_WIDTH
                && Board::coordinate_modulo(oob_quadrant_3_min).1 < Board::BOARD_HEIGHT,
            "Is not in board"
        );

        assert!(
            Board::coordinate_modulo(oob_quadrant_4_max_min).0 < Board::BOARD_WIDTH
                && Board::coordinate_modulo(oob_quadrant_4_max_min).1 < Board::BOARD_HEIGHT,
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
        let mut test_board = Board::new(Board::DEFAULT_SEED);
        let test_position = Position((Board::BOARD_WIDTH - 1) / 2, (Board::BOARD_HEIGHT - 1) / 2);
        test_board.set_player_coordinates(test_position.to_i64());
        assert_eq!(test_board.player_coordinates, test_position);
    }
}
