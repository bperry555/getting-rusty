use std::{io};

fn main() {

    println!("Convert a temperature, choose conversion type to begin:");
    println!("1 ) Convert fahrenheit -> celsius.");
    println!("2 ) Convert celsius -> fahrenheit.");

    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Menu Selection Failed.");
    let selection: u32 = selection.trim().parse().expect("You must enter one of the three options available.");

    println!("Enter a temperature to convert.");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp) 
        .expect("Failed to read line");

    let temp: f64 = temp.trim().parse().expect("Expected a number.");

    match selection {
        1 => fahrenheit_to_celsius(&temp),
        2 => celsius_to_fahrenheit(&temp),
        _ => println!("Invalid Option"),
    }
} //Main END
fn fahrenheit_to_celsius(fahrenheit: &f64) {
    let answer: f64 = (fahrenheit - 32.0) / 1.8;
    println!("{} degrees fahrenheit is {} degrees celsius.", fahrenheit, answer);
}

fn celsius_to_fahrenheit(celsius: &f64) {
    let answer: f64 = (celsius * 1.8) + 32.0;
    println!("{} degrees celsius is {} degrees fahrenheit.", celsius, answer);
}
