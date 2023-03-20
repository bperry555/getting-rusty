use std::{io};

fn main() {

    println!("Enter the temperature to convert");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) 
        .expect("Failed to read line");

    let temp: f64 = guess.trim().parse().expect("Expected a number.");

//    println!("you entered {}", guess);
    let answer: f64 = (&temp - 32.0) * (5.0/9.0);

    println!("{} degrees fahrenheit is {} degrees celsius.", temp, answer);
} //Main END

