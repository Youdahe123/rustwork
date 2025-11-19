use rand::Rng;
use std::io;
fn main() {
    let secNumber = rand::thread_rng().gen_range(1..=10);
    let mut messageToLow = String::new();
    let mut messageCorrect = String::new();

    println!("Correct Number -> {}", secNumber);
    println!("Hey Guess a Number 1-10");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Error Reading input");
    let num: u32 = num.trim().parse().expect("Please Enter Number");
    if secNumber < num {
        println!("Your guess is to high try to go lower")
    } else if secNumber > num {
        println!("Your guess is to low try to go higher")
    } else {
        println!("Congrats your correct!")
    }
    let mut messageToHigh = if secNumber < num { String::from("Your Number was to High");}
}

// worked on mut and some basic if and else statementss - 11/18
