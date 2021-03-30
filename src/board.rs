/*! Visual only `Board` functions

#
Add the missing part (`// TODO`).

You are free to modify anything, including the function parameters,
the code is provided as a support if desired.
*/

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use std::io::{self, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

#[derive(Debug)]
pub struct Board {
    player_color: termcolor::Color,
    player_coordinates: (u32, u32),
    treasure_coordinates: (u32, u32),
    rng: rand::prelude::StdRng,
}

/// the `Board`'s associated constants
impl Board {
    const BOARD_WIDTH: u32 = 15;
    const BOARD_HEIGHT: u32 = 15;
    const BOARD_COLOR: termcolor::Color = termcolor::Color::White;
    // I could technically change them to &str but then I have to look into lifetimes
    const WATER_TILE: char = '~';
    const PLAYER_TILE: char = '@';
    const TREASURE_TILE: char = 'X';
}

/// where I hid all my `Board`'s function's implementation
impl Board {
    /// Prints the `Board` to `stdout`.
    ///
    /// When the function returns, the terminal color is `White`.
    /// This functions requires definition of the `BOARD_WIDTH`, `BOARD_HEIGHT` and `BOARD_COLOR` constants
    /// # Returns
    /// A io::Result i don't know what this is
    /// if an error is fished it's transmitted
    pub fn print(&self) -> io::Result<()> {
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
    fn random_coordinates(rng: &mut rand::prelude::StdRng) -> (u32, u32) {
        Board::coordinate_modulator((rng.next_u32(), rng.next_u32()))
    }

    /// Sets the player coordinate to the one given in argument
    ///
    /// Mod is applied to simulate a torus on the board
    /// This doesn't need the c
    /// movement distance is currently not verified
    ///
    /// # Arguments
    ///
    /// * `coordinates` - a u32 pair representing a posiiton
    fn set_player_coordinates(&mut self, coordinates: (u32, u32)) -> () {
        self.player_coordinates = Board::coordinate_modulator(coordinates);
    }

    /// Applies a mod of widtht and height on the given coordinate
    ///
    /// Supports all rectangle and torus boards (and all possible forms that are bijection of rectangle)
    /// static method
    ///
    /// # Arguments
    ///
    /// * `u32_pair` - the u32 pair that will be modulated to become a coordinate pair
    ///
    /// # Returns
    /// * A pair of u32 that fits in the board
    fn coordinate_modulator(u32_pair: (u32, u32)) -> (u32, u32) {
        (
            u32_pair.0 % Board::BOARD_WIDTH,
            u32_pair.1 % Board::BOARD_HEIGHT,
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
    /// * A Result containing either nothing or an error
    fn tile_painter(
        buffer: &mut termcolor::Buffer,
        color: termcolor::Color,
        tile: String,
    ) -> Result<(), std::io::Error> {
        buffer.set_color(ColorSpec::new().set_fg(Some(color)))?;
        write!(buffer, "{:^3}", tile)?;
        Ok(())
    }

    /// basic default connstructor
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
            player_color: Color::Red,
            player_coordinates: Board::random_coordinates(&mut rng_to_move),
            treasure_coordinates: Board::random_coordinates(&mut rng_to_move),
            rng: rng_to_move,
        }
    }
}
