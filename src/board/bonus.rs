//This method only exists for me to understand stdin
pub fn get_bricolage() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let action: String;
    let integer: u32 = 0;
    let mut pair: (i64, i64) = (0, 0);
    let mut triple: (u32, u32, u32) = (0, 0, 0);

    let salut = Board::DEFAULT_BOARD_HEIGHT;
    let salut2 = super::Board::DEFAULT_BOARD_WIDTH;

    println!("You entered: {}", buffer);

    let trimmed = buffer.trim();

    let inputs: Vec<i64> = trimmed
        .split(' ')
        .map(|x| x.parse().expect("not a i64!"))
        .collect();

    println!("here is buffer: {} here is trim: {}", buffer, trimmed);

    println!("here are the {} inputs splited:", inputs.len());

    let mut i: usize = 0;
    for input in inputs {
        println!("input {} is {}", i, input);
        i += 1;
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