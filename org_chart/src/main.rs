use std::io;
use std::collections::HashMap;

#[derive(Debug)]
enum Action {
    Add,
    Dept,
    All,
}

fn main() {
    
        let mut company_info: HashMap<String, Vec<String>> = HashMap::new();
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

        let user_action = match user_input {
            1 => Action::Add,
            2 => Action::Dept,
            3 => {
                println!("Good Job, Genuis. {}", (user_input));
                Action::All
            } 
            4 | _ => break,
        };
        println!("new output = {:?}", user_action);
//        let mut fields = String::new();

//        io::stdin()
//            .read_line(&mut fields)
//            .expect("couldn't read entries");

//        let (dept, name)  = fields.trim().split_once(' ').unwrap();
//        println!("Testing={:?} and Name={:?}", dept, name);
        
    
       // company_info.entry(dept).or_insert(vec![name]);
//        let new_name = company_info.entry(dept)
//            .and_modify(|e| e.push(name.clone()))
//            .or_insert_with(|| vec![name]);

//        println!("Hashish test! = {:?}", company_info);
    } //loop

} //main

fn add_employee() {
    println!("Enter the department name followed by employee name you would like to enter \\n");
    let mut data_entry = String::new();
    io::stdin()
        .read_line(&mut data_entry)
        .expect("Need to enter: A department name, space, employee name");
    
    let (dept, name) = data_entry.trim().split_once(' ').unwrap();
    
//    company_info.entry(String::from(dept)).or_insert_with();


}
