use crate::task::Task;
use crate::utils::save_tasks;  // Import save_tasks function
use std::io;

pub fn add_task(tasks: &mut Vec<Task>) {
    println!("Enter the task description: ");
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();
    
    println!("Enter the task due date (e.g., 2025-01-01): ");
    let mut due_date = String::new();
    io::stdin().read_line(&mut due_date).unwrap();

    println!("Enter the task priority (e.g., High, Medium, Low): ");
    let mut priority = String::new();
    io::stdin().read_line(&mut priority).unwrap();

    let new_task = Task::new(description.trim().to_string(), due_date.trim().to_string(), priority.trim().to_string());
    tasks.push(new_task);

    // Save tasks after adding
    save_tasks(tasks);
}

pub fn delete_task(tasks: &mut Vec<Task>) {
    println!("Enter the task number to delete: ");
    let mut task_number = String::new();
    io::stdin().read_line(&mut task_number).unwrap();
    let task_number: usize = task_number.trim().parse().unwrap();

    if task_number > 0 && task_number <= tasks.len() {
        tasks.remove(task_number - 1);
        println!("Task deleted.");
    } else {
        println!("Invalid task number.");
    }

    // Save tasks after deletion
    save_tasks(tasks);
}

pub fn view_tasks(tasks: &Vec<Task>) {
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.completed { "Completed" } else { "Pending" };
        println!("{}. [{}] {}", i + 1, status, task.description);
    }
}

pub fn search_tasks(tasks: &Vec<Task>, query: &str) {
    for task in tasks {
        if task.description.contains(query) {
            println!("{}", task.to_string());
        }
    }
}
