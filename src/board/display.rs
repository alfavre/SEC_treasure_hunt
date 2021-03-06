use super::{Board, GameSettings, Position};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use std::io::{self, Write};

/// simple method to print the initial message
pub fn print_init() -> () {
    println!(
        "
    #################################\n
    #                               #\n
    #  Welcome to the pirate game!  #\n
    #                               #\n
    #################################
    "
    );
}

/// simple method to print the settings
///
/// # Arguments
/// * `game_settings` - a compact way to handle all our settings
pub fn print_game_settings(game_settings: &GameSettings) -> () {
    println!("\n\tYour settings are:");
    println!("\t0: Seed\t\t {}", game_settings.seed);
    println!("\t1: your color\t {:?}", game_settings.player_color);
    println!("\t2: your tile\t {}", game_settings.player_tile);

    println!("\nTo change a setting, please enter the corresponding number.");
    println!("To reset to default enter 'default' (or 'd')");
    println!("To continue enter 'continue' (or 'c') or anything else")
}

/// simple method to print the list of awailable commands
pub fn print_turn_command() -> () {
    println!("\n\tYour possible actions are:");
    println!("\t0: Move");
    println!("\t1: Search");
    println!("\t2: Quit");

    println!("\nTo choose your action please enter the corresponding number or name.");
    println!("You can also enter the first letter of the wanted action.");
    println!("You can also move immediately by entering a Zmove.");
    println!("To know more about zmoves, enter Zmove or z.")
}

/// simple method to print the final message
pub fn print_end_screen() {
    println!(
        "
    #################################\n
    #                               #\n
    #           GAME OVER           #\n
    #                               #\n
    #################################
    "
    );

    println!("\nDo you want to replay?");
    println!("If you do, you'll be brought back to the settings screen.");
    println!("If you don't the game will stop and you'll have your terminal back.");
}

/// a simple method to print the closing message
pub fn print_goodbye() {
    println!("Have a nice day!");
}

/// a simple method to print the winner message
pub fn print_win_screen() {
    println!(
        "
    #################################\n
    #                               #\n
    #        A WINNER IS YOU        #\n
    #                               #\n
    #################################
    "
    );
}

/// a simple method to print the nothing found message and to indicate the distance to the treasure
///
/// # Arguments
/// * `shortest_dist` - an integer corresponding to the shortest distance to the treasure
pub fn print_found_nothing(shortest_dist: u32) {
    println!("You searched your current position but sadly found nothing.");
    println!("You update the tracker and look at your broken compass.");
    println!("From your broken compass you managed to deduce the distance to the treasure!");
    println!(
        "The tresure is {} tile(s) from your current position",
        shortest_dist
    );
    println!("Adventure awaits.")
}

/// simple method to print the message for the corrector
/// In a real product, the correction shall be done automatically
///
/// # Arguments
/// * `oob_postion` - The out of bound position the user entered
/// * `ib_position` - The corresponding modulated in bound position the user entered.
pub fn print_special_corrector_message(oob_position: &Position, ib_position: &Position) -> () {
    println!(
        "Howdy, you entered a out of bound position: {}.",
        oob_position
    );
    println!(
        "As the board is a torus, we can correct it to the in bound position: {}.",
        ib_position
    );
    println!("Would you like that?");
}

/// Paints the given tile in the given color for the board print function
///
/// Tile is a char and not a str because long str are ugly when in board.
///
/// # Arguments
/// * `buffer` - a mutable reference to the termcolor::Buffer that will be written to
/// * `color` - a termcolor::Color that will be used for the text written in the buffer
/// * `tile` - the char representing the tile that will be written once in the buffer
///
/// # Returns
/// * A Result containing either a "void" or an error
fn tile_painter(buffer: &mut termcolor::Buffer, color: Color, tile: char) -> io::Result<()> {
    buffer.set_color(ColorSpec::new().set_fg(Some(color)))?;
    write!(buffer, "{:^3}", tile)?;
    Ok(())
}

impl Board {
    /// Prints the `Board` to `stdout`.
    ///
    /// This functions requires definition of the `DEFAULT_BOARD_WIDTH`, `DEFAULT_BOARD_HEIGHT` and `BOARD_COLOR` constants
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
        write!(&mut buffer, "{:>4}", "???")?;
        for _ in 0..Board::DEFAULT_BOARD_WIDTH {
            write!(&mut buffer, "???-???")?;
        }
        writeln!(&mut buffer, "???")?;

        // Main grid
        for y in (0..Board::DEFAULT_BOARD_HEIGHT).rev() {
            write!(&mut buffer, "{:>2} ???", y)?; // Side coordinates

            for x in 0..Board::DEFAULT_BOARD_WIDTH {
                //TODO dont forget to make the treasure invisble in the realese version
                if x == self.player_coordinates.x && y == self.player_coordinates.y {
                    tile_painter(&mut buffer, self.player_color, self.player_tile)?;
                } else {
                    let water_color: Color = match self.tracker[x as usize][y as usize] {
                        true => self.player_color,
                        false => Color::Blue,
                    };

                    tile_painter(
                        &mut buffer,
                        water_color,
                        Board::WATER_TILE,
                        // considering creating a painted_tile struct that encapsulate a "tile" in 2d videogame terms
                    )?;
                }
                // we dont forget to restore the buffer to white for the borders
                buffer.set_color(ColorSpec::new().set_fg(Some(Board::BOARD_COLOR)))?;
            }

            writeln!(&mut buffer, "???")?; // Side column
        }

        // Bottom row
        write!(&mut buffer, "{:>4}", "???")?;
        for _ in 0..Board::DEFAULT_BOARD_WIDTH {
            write!(&mut buffer, "_???_")?;
        }
        writeln!(&mut buffer, "???")?;

        // Bottom coordinates
        write!(&mut buffer, "{:4}", "")?;
        for x in 0..Board::DEFAULT_BOARD_WIDTH {
            write!(&mut buffer, "{:^3}", x)?;
        }
        writeln!(&mut buffer)?;

        // print function ends here, we restore the buffer color to the "normal one"
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?; // I don't know how I can find the default terminal color, so it's white now
        return bufwtr.print(&buffer);
    }
}
