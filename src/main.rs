use std::io;

fn main() {
    println!("Welcome to the game \"Guess the number\"!");
    println!("Make a number from 1 to 100 and do not inform its program.");

    let mut lower_bound = 1;
    let mut upper_bound = 100;
    let max_attempts = 10;

    for attempt in 1..=max_attempts {
        let guess = (lower_bound + upper_bound) / 2;
        println!("attempt {}: I think, this number this number {}", attempt, guess);

        println!("Enter 'more,' less' or 'true' to indicate whether I guessed correctly.");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("I could not read the line");

        match answer.trim() {
            "more" => lower_bound = guess + 1,
            "less" => upper_bound = guess - 1,
            "true" => {
                println!("Hurray up! uwu:>3 🥰 I guessed your number for {} attempts!", attempt);
                return;
            }
            _ => {
                println!("Please enter one of the following: 'more', 'less' or' true.");
                continue;
            }
        }
    }

    println!("I could not guess your number for {} attempts. You win! uwu 🥰", max_attempts);
}

