mod task;
mod task_operations;
mod utils;

use task_operations::{add_task, delete_task, view_tasks, search_tasks};
use utils::load_tasks;
use std::io::{self};

fn main() {
    let mut tasks = load_tasks();
    
    loop {
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Delete Task");
        println!("4. Search Task");
        println!("5. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap();

        match choice {
            1 => add_task(&mut tasks),
            2 => view_tasks(&tasks),
            3 => delete_task(&mut tasks),
            4 => {
                println!("Enter search query: ");
                let mut query = String::new();
                io::stdin().read_line(&mut query).unwrap();
                search_tasks(&tasks, query.trim());
            }
            5 => break,
            _ => println!("Invalid choice, try again!"),
        }
    }
}


