use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

// Function to add a task
pub fn add_task(emty_vec: &mut Vec<String>) {
    println!("Enter a new task:");
    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("Failed to read line");

    let task = task.trim().to_string();
    let init_len = emty_vec.len();

    emty_vec.push(task.clone());

    if emty_vec.len() > init_len {
        println!("Task '{}' was added to the list.", task);
        save_tasks(emty_vec);
    } else {
        println!("Failed to add task.");
    }
}

// Function to view tasks
pub fn view_task(emty_vec: &Vec<String>) {
    if emty_vec.is_empty() {
        println!("No tasks available.");
    } else {
        println!("Tasks:");
        for (i, task) in emty_vec.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }
}

// Function to delete a task
pub fn delete_task(emty_vec: &mut Vec<String>) {
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
            save_tasks(emty_vec);
        } else {
            println!("Error: Number does not exist in boundary");
        }
    } else {
        println!("Invalid input! Please enter a valid number.");
    }
}

// Load tasks from file
pub fn load_tasks() -> Vec<String> {
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
    Vec::new()
}

// Save tasks to file
pub fn save_tasks(emty_vec: &Vec<String>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("tasks.txt")
        .expect("Failed to open file");

    for task in emty_vec {
        writeln!(file, "{}", task).expect("Failed to write to file");
    }
}
