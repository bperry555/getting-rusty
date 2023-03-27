use std::io;
use std::collections::HashMap;


fn main() {
    
    let mut user_input = String::new();
    
    loop{

        println!("Enter action to take: ");
        println!("1 ) Add employee to department list.");
        println!("2 ) List employee(s) by department." );
        println!("3 ) List all employee(s).");
        println!("4 ) Quit.");
        
        io::stdin()
            .read_line(&mut user_input)
            .expect("Couldn't read line");

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match user_input {
            1 => add_employee(&user_input),
            2 => ,
            3 =>,
            4 => break,
            _ => break,
        }

    } //loop
} //main

enum Company {
    Dept_employee(HashMap::<String, vec![str]>),
}
impl Company {
    fn add_employee(&self) {
    }
}
