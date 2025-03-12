use std::fs::OpenOptions;
use std::io::{BufRead, Write, BufReader};
use crate::task::Task;

pub fn load_tasks() -> Vec<Task> {
    let file_name = "tasks.txt";
    let mut tasks = Vec::new();

    if let Ok(file) = OpenOptions::new().read(true).open(file_name) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(task_str) = line {
                tasks.push(Task::from_string(&task_str));
            }
        }
    }

    tasks
}

pub fn save_tasks(tasks: &Vec<Task>) {
    let file_name = "tasks.txt";
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(file_name).unwrap();

    for task in tasks {
        writeln!(file, "{}", task.to_string()).unwrap();
    }
}
