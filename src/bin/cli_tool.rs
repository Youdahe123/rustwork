use std::env;

fn main() {
    // collect into vectors
    let args: Vec<String> = env::args().collect(); // collecting from the command line

    // check if the user has entered a string
    // by checking the number of arguments
    // args[0] is the name of the progam [1] - ... is the contents
    if args.len() < 2 {
        println!("Please enter a string");
        return;
    }
    //store the string in the var
    let mut input: String = args[1].clone();

    let reveresed: String = input.chars().rev().collect::<String>();

    println!("Reversed String: {reveresed}")
}
