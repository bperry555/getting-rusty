use std::io;
use std::collections::HashMap;
use std::num::ParseIntError;
#[warn(unused_variables)]
#[derive(Debug)]
struct Adduser{
}
fn main() {
    
        let mut company_info: HashMap<String, Vec<String>> = HashMap::new();

    loop {

        let choice = promt_user();

        println!("WTF = {}", choice);

        let choice: u32 = choice.trim().parse().expect("FINE");
        println!("JUST MAYBE.. {}", choice);
    

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

fn promt_user() -> String { 
    println!("Enter action to take: ");
    println!("1 ) Add employee to department list.");
    println!("2 ) List employee(s) by department." );
    println!("3 ) List all employee(s).");
    println!("4 ) Quit.");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input);

    return user_input
}   

fn add_employee() {
    println!("Enter the department name followed by employee name you would like to enter \\n");
    let mut data_entry = String::new();
    io::stdin()
        .read_line(&mut data_entry)
        .expect("Need to enter: A department name, space, employee name");
    
    let (dept, name) = data_entry.trim().split_once(' ').unwrap();
    
//    company_info.entry(String::from(dept)).or_insert_with();


}
