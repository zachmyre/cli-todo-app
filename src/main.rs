use chrono::{Datelike, Local};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{env, fs, io};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u64,
    description: String,
    completed: bool,
    date_created: String,
}

//TODO: update completed status

// cargo run -- add task1
// cargo run -- delete 1
// cargo run -- list
// cargo run -- help
fn main() {
    let command = env::args().nth(1).expect("no command given");
    let data = env::args().nth(2).unwrap_or_else(|| String::from(""));

    match command.as_str() {
        "add" | "-a" => add_task(&data),
        "delete" | "-d" => delete_task(&data),
        "toggle" | "-t" => toggle_completed(&data),
        "list" | "-l" => list_tasks(),
        "help" | "-h" => print_help(),
        _ => print_help(),
    }
}

fn print_help() {
    println!(
        "{}",
        "Todo CLI tool written in Rust by Zachary Myre"
            .bright_red()
            .bold()
    );
    println!(" ");
    println!("{}", "USAGE:".bold().green().underline().on_bright_yellow());
    println!("{}", "     cli-todo app [COMMAND]".cyan());
    println!("{}", "     cli-todo app [COMMAND] [DATA]".cyan());
    println!(" ");
    println!(
        "{}",
        "Available commands:"
            .bold()
            .green()
            .underline()
            .on_bright_yellow()
    );
    println!("{}", "     COMMAND [DATA]".white().bold());
    println!(
        "{}",
        "     add [task], -a [task]                  Adds task to list.".cyan()
    );
    println!(
        "{}",
        "     delete [taskID], -d [taskID]           Deletes task based on ID.".cyan()
    );
    println!(
        "{}",
        "     toggle [taskID], -t [taskID]           Toggles task completed status based on ID."
            .cyan()
    );
    println!(
        "{}",
        "     list, -l                               Lists current tasks.".cyan()
    );
    println!(
        "{}",
        "     help, -h                               Displays this help menu.".cyan()
    );
}

fn add_task(data: &String) {
    // Append the task to the JSON file
    append_task_to_file(data).expect("Failed to add task");
}

fn delete_task(data: &String) {
    // Remove the task from the JSON file
    remove_task_from_file(data).expect("Failed to delete task");
}

fn toggle_completed(data: &String) {
    // Toggle the completed value in the task
    let mut tasks = match load_tasks_from_file() {
        Ok(tasks) => tasks,
        Err(err) => {
            eprintln!("Error loading tasks from json file: {}", err);
            return;
        }
    };
    if let Some(current_task) = tasks.iter_mut().find(|t| t.id.to_string() == *data) {
        current_task.completed = !current_task.completed;
    } else {
        println!("Task with ID {} not found", data);
    }

    // Save the updated list of tasks back to the file
    if let Err(err) = save_tasks_to_file(&tasks) {
        eprintln!("Error updating task in json file: {}", err);
    }
}

fn list_tasks() {
    let tasks = load_tasks_from_file().expect("Failed to load tasks");
    println!("");
    println!(
        "{:<5} {:<70} {:<10} {}",
        "ID".bold().bright_green(),
        "Description".bold().bright_green(),
        "Completed".bold().bright_green(),
        "Date Created".bold().bright_green()
    );
    println!("{}", "-".repeat(100).bold().bright_green());
    for task in tasks {
        println!(
            "{:<5} {:<70} {:<10} {}",
            task.id.to_string().cyan(),
            task.description.to_string().cyan(),
            task.completed.to_string().cyan(),
            task.date_created.to_string().cyan()
        );
        println!("");
    }
}

fn append_task_to_file(task: &str) -> io::Result<()> {
    let mut tasks = load_tasks_from_file()?;
    let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    let current_time = Local::now();
    // Format the current time as "mm/dd/yy"
    let formatted_time = format!(
        "{:02}/{:02}/{}",
        current_time.month(),
        current_time.day(),
        current_time.year() % 100 // Get the last two digits of the year
    );

    let new_task = Task {
        id,
        description: task.to_string(),
        completed: false,
        date_created: formatted_time.to_string(),
    };
    tasks.push(new_task);
    save_tasks_to_file(&tasks)
}

fn remove_task_from_file(data: &String) -> io::Result<()> {
    let mut tasks = load_tasks_from_file()?;

    // Parse the ID from the input data
    let id_to_remove: u64 = data.parse().expect("Invalid ID");

    // Find the index of the task with the matching ID
    let index_to_remove = tasks.iter().position(|task| task.id == id_to_remove);
    if let Some(index) = index_to_remove {
        // Remove the task from the list
        tasks.remove(index);

        // Save the updated list of tasks back to the file
        save_tasks_to_file(&tasks)?;

        println!("Task with ID {} removed successfully", id_to_remove);
    } else {
        println!("Task with ID {} not found", id_to_remove);
    }
    Ok(())
}

fn load_tasks_from_file() -> io::Result<Vec<Task>> {
    let filename = "tasks.json";
    if let Ok(contents) = fs::read_to_string(filename) {
        serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    } else {
        Ok(Vec::new())
    }
}

fn save_tasks_to_file(tasks: &[Task]) -> io::Result<()> {
    let filename = "tasks.json";
    let contents = serde_json::to_string_pretty(tasks)?;
    let _ = fs::write(filename, contents);
    list_tasks();
    Ok(())
}
