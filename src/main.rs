use std::io::{stdin, stdout, Write};
use rand::Rng;

fn main() {
    let _name = name();
    game();
}

fn name() -> String {
    print!("Who are you?\n> ");
    stdout().flush().unwrap();
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    print!("Hello, {}", name);
    name
}

fn game() {
    let mut rng = rand::thread_rng();
    let (mut heads, mut tails) = (0, 0);
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
}
