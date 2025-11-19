use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    let secNumber = rand::thread_rng().gen_range(1..=10);

    // let mut messageCorrect = String::new();

    println!("Hey Guess a Number 1-10");

    let mut how_many = String::new();
    println!("How many random numbers do you want to");
    io::stdin()
        .read_line(&mut how_many)
        .expect("Error reading input");
    let numGuess: u8 = how_many.trim().parse().expect("Error");

    let mut correct = Vec::new();

    for i in 0..numGuess {
        correct.push(rand::thread_rng().gen_range(1..=10));
    }
    println!("{correct:?}");

    let mut guessMade = 0;

    while guessMade < numGuess {
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Error Reading input");

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("This is the error");
                continue;
            }
        };

        let message = match num.cmp(&correct[guessMade as usize]) {
            Ordering::Greater => "Your Number was to High",
            Ordering::Less => "Your Number was to low",
            Ordering::Equal => {
                guessMade += 1;
                "Congrats Your Finally Correct"
            }
        };
        println!("{message}");
    }
}
// worked on mut and some basic if and else statementss - 11/18
