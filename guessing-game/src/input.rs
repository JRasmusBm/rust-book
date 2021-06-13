use std::io;

pub fn get_guess_from_user() -> u32 {
    let mut guess = String::new();

    println!("Please provide a number:");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    return guess.trim().parse().expect("Please type a number!");
}
