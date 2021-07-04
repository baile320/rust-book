use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let mut employee_directory: HashMap<String, Vec<String>> = HashMap::new();
    let stdin = std::io::stdin();

    println!("Type 'Add <name> to <department>' to add an employee");
    println!("Type 'List <department>' to list the employees of a department");
    println!("Type 'All' to list all employees by department");
    println!("Type 'Quit' to quit");
    println!();

    for line in stdin.lock().lines() {
        let input = line.expect("error: unable to read user input");

        match Command::from_input(&input) {
            Some(Command::Add { dept, name }) => add_employee(name, dept, &mut employee_directory),
            Some(Command::List(dept)) => print_employees_for_dept(dept, &employee_directory),
            Some(Command::All) => print_all_employees(&employee_directory),
            Some(Command::Quit) => break,
            None => println!("Please enter an appropriate command"),
        }
    }
}

enum Command {
    Add { dept: String, name: String },
    List(String),
    All,
    Quit,
}

impl Command {
    fn from_input(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.trim().split_whitespace().collect();

        match words.as_slice() {
            ["Add", name, "to", dept] => Some(Command::Add {
                dept: dept.to_string(),
                name: name.to_string(),
            }),
            ["All"] => Some(Command::All),
            ["List", dept] => Some(Command::List(dept.to_string())),
            ["Quit"] => Some(Command::Quit),
            _ => None,
        }
    }
}

fn add_employee(name: String, dept: String, employee_directory: &mut HashMap<String, Vec<String>>) {
    employee_directory.entry(dept).or_default().push(name)
}

fn print_employees_for_dept(dept: String, employee_directory: &HashMap<String, Vec<String>>) {
    match employee_directory.get(&dept) {
        Some(names) => {
            println!("Employees in {}", dept);
            for name in names {
                println!("{}", name);
            }
            println!();
        }
        None => println!("There are no employees for that department"),
    }
}
fn print_all_employees(employee_directory: &HashMap<String, Vec<String>>) {
    println!("All Employees");
    for (dept, names) in employee_directory {
        let mut names = names.clone();
        names.sort();
        for name in names {
            println!("{}: {}", dept, name);
        }
    }
    println!();
}
