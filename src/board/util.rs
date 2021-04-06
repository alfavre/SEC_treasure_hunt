use std::io::{self};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};


pub enum Command {
    ColorRGB(u64,u64,u64),
    ColorName(Color),
    Move(i64,i64),
    Search,
    Quit,

}

/// I have no idea what is the best way to do this
/// adding a trait to tuple for my to_i64 fn
/// creating a named tuple implementing to_i64
/// creating a struct to make the field named too
/// I went with the named tuple as it seems to save more memory than the struct
#[derive(Debug, PartialEq)]
pub struct Position(pub u32, pub u32);

impl Position {
    /// utility cast that gives a i64 pair of the board_position
    /// this is the only reason why this struct exists
    /// if not &self as argument we have move problems
    pub fn to_i64(&self) -> (i64, i64) {
        (self.0 as i64, self.1 as i64)
    }
}




//This method only exists for me to understand stdin
pub fn get_bricolage() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let action: String;
    let integer: u32 = 0;
    let mut pair: (i64,i64) = (0,0);
    let mut triple: (u32,u32,u32) = (0,0,0);


    println!("You entered: {}", buffer);

    let trimmed = buffer.trim();

    let inputs: Vec<i64> = trimmed.split(' ').map(|x| x.parse().expect("not a i64!")).collect();

    println!("here is buffer: {} here is trim: {}",buffer,trimmed);

    println!("here are the {} inputs splited:",inputs.len());

    let mut i:usize = 0;
    for input in inputs{
        println!("input {} is {}",i,input);
        i+=1;
    }

    /*

    match trimmed.parse::<u32>() {
        Ok(i) => integer = i,
        Err(..) => println!("{} is not a single u32",trimmed),
    };


    println!("it's over, her is the u32 {}", integer);

    */

    Ok(())
}
