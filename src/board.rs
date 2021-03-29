/*! Visual only `Board` functions

#
Add the missing part (`// TODO`).

You are free to modify anything, including the function parameters,
the code is provided as a support if desired.
*/

use std::io::{self, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

struct Board {
    //TODO 
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
        buffer.set_color(ColorSpec::new().set_fg(Some(BOARD_COLOR)))?;
        write!(&mut buffer, "{:>4}","⌜")?;
        for _ in 0..BOARD_WIDTH {
            write!(&mut buffer, "⎺-⎺")?;
        }
        writeln!(&mut buffer, "⌝")?;

        // Main grid
        for y in (0..BOARD_HEIGHT).rev() {
            write!(&mut buffer, "{:>2} ∣", y)?; // Side coordinates

            for x in 0..BOARD_WIDTH {
                // TODO

                write!(&mut buffer, "{:^3}", grid_c)?;
                buffer.set_color(ColorSpec::new().set_fg(Some(BOARD_COLOR)))?;
            }

            writeln!(&mut buffer, "∣")?; // Side column
        }

        // Bottom row
        write!(&mut buffer, "{:>4}", "⌞")?;
        for _ in 0..BOARD_WIDTH {
            write!(&mut buffer, "_⎽_")?;
        }
        writeln!(&mut buffer, "⌟")?;

        // Bottom coordinates
        write!(&mut buffer, "{:4}","")?;
        for x in 0..BOARD_WIDTH {
            write!(&mut buffer, "{:^3}", x)?;
        }
        writeln!(&mut buffer)?;

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        return bufwtr.print(&buffer);
    }
}
