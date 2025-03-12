#[derive(Debug, Clone)]
pub struct Task {
    pub description: String,
    pub completed: bool,
    pub due_date: String,
    pub priority: String,
}

impl Task {
    pub fn new(description: String, due_date: String, priority: String) -> Task {
        Task {
            description,
            completed: false,
            due_date,
            priority,
        }
    }

    pub fn from_string(task_str: &str) -> Self {
        let parts: Vec<&str> = task_str.splitn(4, ' ').collect();
        let completed = parts[0] == "[X]";
        let description = parts.get(1).unwrap_or(&"").to_string();
        let due_date = parts.get(2).unwrap_or(&"").to_string();
        let priority = parts.get(3).unwrap_or(&"").to_string();

        Task {
            description,
            completed,
            due_date,
            priority,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "[{}] {} | Due: {} | Priority: {}",
            if self.completed { "X" } else { " " },
            self.description, self.due_date, self.priority
        )
    }
}
