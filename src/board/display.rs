use super::Board;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use std::io::{self, Write};

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
    pub(super) fn print_game_board(&self) -> io::Result<()> {
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
                if x == self.player_coordinates.x && y == self.player_coordinates.y {
                    Board::tile_painter(
                        &mut buffer,
                        self.player_color,
                        Board::PLAYER_TILE,
                    )?;
                } else if x == self.treasure_coordinates.x && y == self.treasure_coordinates.y {
                    Board::tile_painter(
                        &mut buffer,
                        termcolor::Color::Yellow,
                        Board::TREASURE_TILE,
                    )?;
                } else {
                    Board::tile_painter(
                        &mut buffer,
                        termcolor::Color::Blue,
                        Board::WATER_TILE,
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
    fn tile_painter(buffer: &mut termcolor::Buffer, color: Color, tile: char) -> io::Result<()> {
        buffer.set_color(ColorSpec::new().set_fg(Some(color)))?;
        write!(buffer, "{:^3}", tile)?;
        Ok(())
    }
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
        assert!(test_board.print_game_board().is_ok());
    }
}
