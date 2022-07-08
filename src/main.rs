use std::io::{stdin, stdout, Write};
use rand::Rng;

fn main() {
    let name = name();
    game(name);
}

fn name() -> String {
    print!("Who are you?\n> ");
    stdout().flush().unwrap();
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    name = name.trim_end_matches('\n').to_string();
    println!("Hello, {}!", name);
    name
}

fn game(name: String) {
    let mut rng = rand::thread_rng();
    let (mut heads, mut tails) = (0, 0);
    println!("Tossing a coin...");
    for round in 1..=3 {
        if rng.gen_bool(1.0 / 2.0) {
            println!("Round {}: Heads", round);
            heads += 1;
        } else {
            println!("Round {}: Tails", round);
            tails += 1;
        }
    }
    println!("Heads: {}, Tails: {}", heads, tails);

    if heads > tails {
        println!("{} won!", name);
    } else {
        println!("{} lost!", name);
    }
}
