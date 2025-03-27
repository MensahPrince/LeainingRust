mod utils;

use utils::{add_task, view_task, delete_task, load_tasks};

fn interface(emty_vec: &mut Vec<String>) {
    println!("Welcome to the Todo List! \nWhat would you like to do?");
    println!("1. Add a task");
    println!("2. View tasks");
    println!("3. Delete tasks");
    println!("4. Exit");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");

    match choice.trim() {
        "1" => add_task(emty_vec),
        "2" => view_task(emty_vec),
        "3" => delete_task(emty_vec),
        "4" => {
            println!("Exiting... Goodbye!");
            return;
        }
        _ => println!("Invalid choice. Please try again."),
    }

    interface(emty_vec);
}

fn main() {
    let mut emty_vec = load_tasks();
    interface(&mut emty_vec);
}
