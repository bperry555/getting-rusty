use std::io;
use std::collections::HashMap;


fn main() {
    
        let mut company_info: HashMap<&str, Vec<&str>> = HashMap::new();
    loop {
    
    let mut user_input = String::new();
        println!("Enter action to take: ");
        println!("1 ) Add employee to department list.");
        println!("2 ) List employee(s) by department." );
        println!("3 ) List all employee(s).");
        println!("4 ) Quit.");
        
        io::stdin()
            .read_line(&mut user_input)
            .expect("Couldn't read line");

        let user_input: u32 = user_input.trim().parse()
            .expect("Please type a number");

        println!("user_input={}", user_input);

        let mut fields = String::new();

        io::stdin()
            .read_line(&mut fields)
            .expect("couldn't read entries");

        let (dept, name)  = fields.trim().split_once(' ').unwrap();
        println!("Testing={:?} and Name={:?}", dept, name);
        
    
       // company_info.entry(dept).or_insert(vec![name]);
        let new_name = company_info.entry(dept)
            .and_modify(|e| e.push(name.clone()))
            .or_insert_with(|| vec![name]);

        println!("Hashish test! = {:?}", company_info);
    }

} //main

