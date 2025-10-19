use std::fs::{self, File};
use std::io::{self, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

fn main(){
    let mut tasks:Vec<Task> = load_tasks();

    loop{
        print!("\n ToDo list menu : ");
        print!("\n 1. Add Task");
        print!("\n 2. View Tasks");
        print!("\n 3. Mark as Task Completed");
        print!("\n 4. Delete Task");
        print!("\n 5. Exit");
        
        let choice = get_input("\n Enter your choice: ");
        match choice.trim(){
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_task_completed(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_tasks(&tasks);
                println!("Exiting...");
                break;
            },
            _ => println!("Invalid choice, please try again."),
        }
    }
}

// Function to get user input
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

// Function to load tasks from a file
fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
        
    }
}

// Function to save tasks to a file
fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string(tasks).expect("Failed to serialize tasks");
    let mut file = File::create("tasks.json").expect("Failed to create file");
    file.write_all(data.as_bytes()).expect("Failed to write to file");
}

// Function to add a new task
fn add_task(tasks: &mut Vec<Task>) {
    let description = get_input("Enter task description: ");
    let id = if let Some(last) = tasks.last() { last.id + 1 } else { 1 };
    let task = Task { id, description: description.trim().to_string(), completed: false };
    tasks.push(Task{
        id,
        description: description.trim().to_string(),
        completed: false,
    });
    println!("Task added successfully.");
}

// Function to view all tasks
fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
        return;
    }
    for task in tasks {
        println!("ID: {}, Description: {}, Completed: {}", task.id, task.description, task.completed);
    }
}   

// Function to mark a task as completed
fn mark_task_completed(tasks: &mut Vec<Task>) {
    let id_input = get_input("Enter task ID to mark as completed: ");
    if let Ok(id) = id_input.trim().parse::<usize>() {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("Task marked as completed.");
        } else {
            println!("Task with ID {} not found.", id);
        }
    } else {
        println!("Invalid ID.");            
    }
}   

// Function to delete a task
fn delete_task(tasks: &mut Vec<Task>) {
    let id_input = get_input("Enter task ID to delete: ");
    if let Ok(id) = id_input.trim().parse::<usize>() {
        if let Some(pos) = tasks.iter().position(|t| t.id == id) {
            tasks.remove(pos);
            println!("Task deleted successfully.");
        } else {
            println!("Task with ID {} not found.", id);
        }
    } else {
        println!("Invalid ID.");            
    }
}   