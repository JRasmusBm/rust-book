use rand::Rng;

mod input;
mod output;

fn main() {
    let target = rand::thread_rng().gen_range(1..101);
    println!("Welcome to the guessing game!");
    println!("The secret number is between 1 and 100.");

    loop {
        let guess = input::get_guess_from_user();
        match output::print_result(guess, target) {
            Ok(_) => break,
            Err(_) => {}
        }
    }
}
