//  Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
//  For example, “Add Sally to Engineering” or “Add Amir to Sales.”
//  Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
//

use std::io::{self, Write};
use std::collections::HashMap;

#[warn(unused_variables)]
#[derive(Debug)]
enum Selection {
    Add,
    EmpDept,
    EmpAll,
    Quit,
}
fn main() {
    
    let mut company_info: HashMap<String, Vec<String>> = Default::default();

    loop {
        let input = promt_user();

        match input {
            Selection::Add => add_employee(&mut company_info),
            Selection::EmpDept => list_emp_dept(&company_info),
            Selection::EmpAll => list_emp_all(&mut company_info),
            Selection::Quit => quit(),
        };
    } //loop
} //main

fn promt_user() -> Selection { 
    let mut user_input = String::new();
    loop {
        println!("Enter action to take: ");
        println!("1 ) Add employee to department list.");
        println!("2 ) List employee(s) by department." );
        println!("3 ) List all employee(s).");
        println!("4 ) Quit.");
        print!(":=> ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Unable to read line");

        if let Ok(parse_num) = user_input.trim().parse::<u32>() {
            match parse_num {
                1 => return Selection::Add,
                2 => return Selection::EmpDept,
                3 => return Selection::EmpAll,
                4 => return Selection::Quit,
                _ => { println!("Number must be between 1-4\n");
                    user_input.clear();
                    continue },
            };
        } else {
            user_input.clear();
            println!("That is not a number between 1-4\n");
        };
    }; //loop
}   

fn add_employee(db: &mut HashMap<String, Vec<String>>) {
    let mut emp_entry = String::new();

    loop {
        println!("Enter the department name followed by employee name you would like to enter");
        print!(":=> ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut emp_entry)
            .expect("Unable to read line");
        
       if let Some((dept, name)) = emp_entry.trim().split_once(' ') {
            db.entry(String::from(dept))
                .or_default()
                .push(String::from(name));
            println!("Entry added- Dept[{dept}] : Employee[{name}]\n");
            break;
        } else {
            println!("You need to enter a Dept AND a name");
            emp_entry.clear();
            continue;
        };
    }; //loop
}

fn list_emp_dept(db: &HashMap<String, Vec<String>>) {
    if db.is_empty() { 
        println!("There are no Departments or Employees, add some :)\n");
        return;
    }

    let mut dept_input = String::new();

    loop {
        println!("List of available departments:");

        for k in db.keys() {
            println!("Dept:[{k}]");
        }

        println!("Enter the department name to get list of employees:");
        print!(":=> ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut dept_input)
            .expect("Expected a department name");
        
        let dept = dept_input.trim();

        if let Some((_, names)) = db.get_key_value(dept) {
            println!("{:?}", names);
            break;
        } else {
            println!("That department doesn't exist");
        }
    }; // loop
}

fn list_emp_all(db: &mut HashMap<String, Vec<String>>) {
    for (k,v) in db.iter_mut() {
        v.sort();
        println!("Dept:[{}]: Emp:{:?}", k, v);
    };
}

fn quit() {
    std::process::exit(0);
}
