use std::io;

fn main() {
    println!("Which number in the fibonacci sequence do you want to find?");

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to get input.");
        
        if user_input.trim() == "quit" {
            break;
        }

        let n: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("The {} number in the fibonacci sequence is {}", user_input, fibonacci(n));
        
    return;
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
