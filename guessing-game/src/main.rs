mod input;
mod output;

fn main() {
    let target = 5;

    loop {
        let guess = input::get_guess_from_user();
        match output::print_result(guess, target) {
            Ok(_) => break,
            Err(_) => {}
        }
    }
}
