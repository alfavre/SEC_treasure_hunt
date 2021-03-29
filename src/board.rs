/*! Visual only `Board` functions

#
Add the missing part (`// TODO`).

You are free to modify anything, including the function parameters,
the code is provided as a support if desired.
*/

use std::io::{self, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use rand::{RngCore,SeedableRng};
use rand::rngs::StdRng;
//use rand::{Rng, SeedableRng};
//use rand_core::SeedableRng;


#[derive(Debug)]
pub struct Board {
    player_color: termcolor::Color,
    player_coordinates: (u32,u32),
    treasure_coordinates: (u32,u32),
    rng: rand::prelude::StdRng,
}

///Some associated constants
/// 
/// Define how boards looks like
impl Board {
    const BOARD_WIDTH : u32 = 3;
    const BOARD_HEIGHT : u32 = 10;
    const BOARD_COLOR: termcolor::Color = termcolor::Color::White;
    const WATER_TILE: char = '~';
    const PLAYER_TILE: char = '@';
    const TREASURE_TILE: char = 'X';
}

impl Board {
    /// Prints the `Board` to `stdout`.
    ///
    /// When the function returns, the terminal color is `White`.
    /// This functions requires definition of the `BOARD_WIDTH`, `BOARD_HEIGHT` and `BOARD_COLOR` constants
    pub fn print(&self) -> io::Result<()> {
        let bufwtr = BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwtr.buffer();
        

        // Top row
        buffer.set_color(ColorSpec::new().set_fg(Some(Board::BOARD_COLOR)))?;
        write!(&mut buffer, "{:>4}","⌜")?;
        for _ in 0..Board::BOARD_WIDTH {
            write!(&mut buffer, "⎺-⎺")?;
        }
        writeln!(&mut buffer, "⌝")?;

        // Main grid
        for y in (0..Board::BOARD_HEIGHT).rev() {
            write!(&mut buffer, "{:>2} ∣", y)?; // Side coordinates

            for x in 0..Board::BOARD_WIDTH {
                // TODO
                if x == self.player_coordinates.0 && y == self.player_coordinates.1 {
                    write!(&mut buffer, "{:^3}", Board::PLAYER_TILE)?;
                } else if x == self.treasure_coordinates.0 && y == self.treasure_coordinates.1{ 
                    write!(&mut buffer, "{:^3}", Board::TREASURE_TILE)?; // player overwrites treasure
                } else {
                    write!(&mut buffer, "{:^3}", Board::WATER_TILE)?;
                }
                
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
        write!(&mut buffer, "{:4}","")?;
        for x in 0..Board::BOARD_WIDTH {
            write!(&mut buffer, "{:^3}", x)?;
        }
        writeln!(&mut buffer)?;

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?; // wtf is this line? lmao it sets the terminal text color
        return bufwtr.print(&buffer);
    }

    /// Gets a new pair of random coordinates
    /// 
    /// respects the board proportions
    /// this method is not static, it uses the rng generator
    fn random_coordinates(&mut self) -> (u32,u32){
        Board::coordinate_modulator((self.rng.next_u32(),self.rng.next_u32()))
    }

    /// Sets the player coordinate to the one given in argument
    /// 
    /// Mod is applied to simulate a torus on the board
    /// movement distance is currently not verified
    fn set_player_coordinates(&mut self, coordinates: (u32,u32)) -> () {
        self.player_coordinates=Board::coordinate_modulator(coordinates);
    }

    /// applies a mod of widtht and height on the given coordinate 
    /// 
    /// static method
    fn coordinate_modulator(u32_pair: (u32,u32)) -> (u32,u32) {
        (u32_pair.0 % Board::BOARD_WIDTH,u32_pair.1 % Board::BOARD_HEIGHT)
    }

    /// basic default connstructor
    /// 
    /// creates a new board
    pub fn new(seed: u64)-> Board{
        let mut rng_to_move = StdRng::seed_from_u64(seed); // not suitable for crypto, but this isn't crypto
        Board{
            player_color:Color::Red,
            player_coordinates: Board::coordinate_modulator((rng_to_move.next_u32(),rng_to_move.next_u32())),
            treasure_coordinates: Board::coordinate_modulator((rng_to_move.next_u32(),rng_to_move.next_u32())),
            rng: rng_to_move,

        }
    }



}
