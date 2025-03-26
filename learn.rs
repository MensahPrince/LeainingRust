use std::io;

// Display interface for the todo list and logic
fn interface(emty_vec: &mut Vec<String>) {
    println!("Welcome to the Todo List! \nWhat would you like to do?");
    println!("1. Add a task");
    println!("2. View tasks");
    println!("3. Delete tasks");
    println!("4. Exit");
    println!("Please enter your choice (1-3):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    match choice.trim() {
        "1" => {
            println!("-------Add a task-------");
            add_task(emty_vec);
        }
        "2" => {
            println!("-------View tasks-------");
            view_task(emty_vec);
        }
        "3" => {
            println!("------Delete tasks-------");
            delete_task(emty_vec)
        }
        "4" => {
            println!("Exiting... Goodbye!");
            return;
        }
        _ => {
            println!("Invalid choice. Please try again.");
        }
    }

    // Call interface again for next action
    interface(emty_vec);
}

// Function to add a task
fn add_task(emty_vec: &mut Vec<String>) {
    println!("Enter a new task:");
    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("Failed to read line");

    let task = task.trim().to_string(); // Trim whitespace and newline
    let init_len = emty_vec.len(); // Store initial length before push

    emty_vec.push(task.clone()); // Push task to vector

    // Check if push was successful
    if emty_vec.len() > init_len {
        println!("Task '{}' was added to the list.", task);
    } else {
        println!("Failed to add task.");
    }
}

// Function to view tasks
fn view_task(emty_vec: &Vec<String>) {
    if emty_vec.is_empty() {
        println!("No tasks available.");
    } else {
        println!("Tasks:");
        for (i, task) in emty_vec.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }
}

fn delete_task(emty_vec: &mut Vec<String>){
    for (i, task) in emty_vec.iter().enumerate() {
        println!("{}: {} \n", i + 1, task);
    }

    println!("Enter number of task you want to delete");
    let mut num_to_del = String::new();
    io::stdin().read_line(&mut num_to_del).expect("Failed to read line.");
    let index: i32 = (num_to_del.trim().parse::<i32>().unwrap()) - 1;
    if index >= 0 && (index as usize) < emty_vec.len() {
        let removed_task = emty_vec.remove(index as usize);
        println!("Task '{}' was deleted from the list.", removed_task);
    } else {
        println!("Error: Number does not exist in boundary");
    }   
}

// Main function
fn main() {
    let mut emty_vec: Vec<String> = Vec::new(); // Create task list
    interface(&mut emty_vec); // Pass it to interface
}
