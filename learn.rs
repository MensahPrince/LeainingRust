use std::io;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, Write};
use std::path::Path;



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
        save_tasks(emty_vec); // Save tasks after adding
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
    if emty_vec.is_empty() {
        println!("No tasks to delete.");
        return;
    }

    for (i, task) in emty_vec.iter().enumerate() {
        println!("{}: {} \n", i + 1, task);
    }

    println!("Enter number of task you want to delete:");
    let mut num_to_del = String::new();
    io::stdin().read_line(&mut num_to_del).expect("Failed to read line.");
    
    if let Ok(index) = num_to_del.trim().parse::<usize>() {
        if index > 0 && index <= emty_vec.len() {
            let removed_task = emty_vec.remove(index - 1);
            println!("Task '{}' was deleted from the list.", removed_task);
            save_tasks(emty_vec); // Save tasks after deletion
        } else {
            println!("Error: Number does not exist in boundary");
        }
    } else {
        println!("Invalid input! Please enter a valid number.");
    }
}


fn load_tasks() -> Vec<String> {
    let file_path = "tasks.txt";

    if Path::new(file_path).exists() {
        if let Ok(file) = fs::File::open(file_path) {
            return io::BufReader::new(file)
                .lines()
                .filter_map(Result::ok)
                .collect();
        }
    }
    
    println!("The file '{}' does not exist!", file_path);
    Vec::new() // Return an empty list if the file doesn't exist or cannot be opened
}


fn save_tasks(load_task: &Vec<String>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) // Clears file before writing new tasks
        .create(true)
        .open("tasks.txt")
        .expect("Failed to open file");

    for task in load_task {
        writeln!(file, "{}", task).expect("Failed to write to file");
    }
}

// Main function
fn main() {
    let mut emty_vec = load_tasks(); // Load tasks from file
    if emty_vec.is_empty() {
        println!("No tasks found. Start adding tasks!");
    }
    interface(&mut emty_vec);
}
