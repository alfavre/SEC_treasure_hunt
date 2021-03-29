/*! Visual only `Board` functions

#
Add the missing part (`// TODO`).

You are free to modify anything, including the function parameters,
the code is provided as a support if desired.
*/

use std::io::{self, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};



pub struct Board {
    //TODO 


}

///Some associated constants
/// 
/// Define how boards looks like
impl Board {
    const BOARD_WIDTH : u32 = 15;
    const BOARD_HEIGHT : u32 = 15;
    const BOARD_COLOR: termcolor::Color = termcolor::Color::White;
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

                let grid_c = '~' ;
                write!(&mut buffer, "{:^3}", grid_c)?;
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

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        return bufwtr.print(&buffer);
    }

    /// basic connstructor
    /// 
    /// creates a new board
    pub fn new()-> Board{
        Board{}
    }
}
