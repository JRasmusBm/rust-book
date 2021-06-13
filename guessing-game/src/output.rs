pub fn print_result(guess: u32, actual: u32) -> Result<(), ()> {
    println!("You guessed: {}", guess);
    if guess < actual {
        println!("The guess is too low!");
        return Err(());
    }
    if actual < guess {
        println!("The gusss is too high!");
        return Err(());
    }

    println!("You guessed right!");
    return Ok(())
}
