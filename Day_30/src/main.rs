use std::io::{self, Write}; // For reading user input

#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: &str) -> Self {
        Task {
            description: description.to_string(),
            completed: false,
        }
    }

    fn mark_complete(&mut self) {
        self.completed = true;
    }

    fn display(&self) {
        let status = if self.completed { "Done" } else { "Pending" };
        println!("[{}] {}", status, self.description);
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        // Show menu
        println!("\nTo-Do List");
        println!("1. Add a task");
        println!("2. List tasks");
        println!("3. Mark task as complete");
        println!("4. Exit");

        // Get user input
        print!("Enter your choice: ");
        io::stdout().flush().unwrap(); // Flush to ensure prompt shows up before input
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                // Add a task
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                let task = Task::new(description.trim());
                tasks.push(task);
                println!("Task added!");
            }
            2 => {
                // List tasks
                println!("\nCurrent Tasks:");
                for (i, task) in tasks.iter().enumerate() {
                    println!("{}. {}", i + 1, task.description);
                    task.display();
                }
            }
            3 => {
                // Mark task as complete
                print!("Enter task number to mark as complete: ");
                io::stdout().flush().unwrap();
                let mut task_number = String::new();
                io::stdin().read_line(&mut task_number).unwrap();
                let task_number: usize = task_number.trim().parse().unwrap_or(0);

                if task_number > 0 && task_number <= tasks.len() {
                    tasks[task_number - 1].mark_complete();
                    println!("Task {} marked as complete!", task_number);
                } else {
                    println!("Invalid task number!");
                }
            }
            4 => {
                // Exit
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
