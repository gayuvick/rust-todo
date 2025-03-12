# Todo List Application in Rust

This is a command-line based task management application written in Rust. It allows users to add, view, delete, search, and sort tasks.

## Features

- Add new tasks with a description, due date, and priority.
- View all tasks.
- Delete tasks by their number.
- Search tasks by description.
- Sort tasks alphabetically or by completion status.

## Project Structure

- `Cargo.toml`: Defines the project metadata and dependencies.
- `Cargo.lock`: Records the exact versions of dependencies used.
- `.gitignore`: Specifies files and directories to be ignored by Git.
- `src/main.rs`: Contains the main application logic.
- `src/task.rs`: Defines the `Task` struct and its associated methods.
- `src/task_operations.rs`: Contains functions for task operations (add, delete, view, search).
- `src/utils.rs`: Contains utility functions for loading and saving tasks.
- `tasks.txt`: Stores the list of tasks.
  
## Running the Project

To run the project, use the following command:
```sh
cargo run
```

## Usage

When you run the application, you will see a menu with the following options:
```sh
1. Add Task
2. View Tasks
3. Delete Task
4. Search Task
5. Exit
```

Adding a Task
Select option 1 and follow the prompts to enter the task description, due date, and priority.

Viewing Tasks
Select option 2 to view all tasks.

Deleting a Task
Select option 3 and enter the task number to delete.

Searching for a Task
Select option 4, enter the search query, and view the matching tasks.

Exiting the Application
Select option 5 to exit the application.


## Task Struct

The Task struct is defined in src/task.rs and represents a task with the following fields:

description: A string describing the task.
completed: A boolean indicating whether the task is completed.
due_date: A string representing the due date of the task.
priority: A string representing the priority of the task.
