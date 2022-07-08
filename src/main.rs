use rand::Rng;

fn main() {
    game();
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
