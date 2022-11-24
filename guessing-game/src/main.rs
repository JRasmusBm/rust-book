mod random;
mod input;
mod output;

fn main() {
    let target = random::generate_target_from_range(1, 5000);

    println!("Welcome to the guessing game!");
    println!("The secret number is between {} and {}.", start, end);

    loop {
        let guess = input::get_guess_from_user();
        match output::print_result(guess, target) {
            Ok(_) => break,
            Err(_) => {}
        }
    }
}
