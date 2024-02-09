// fn main() {
//     println!("Hello, world!");
// }

use std::collections::HashMap;

struct Task {
    id: usize,
    description: String,
    completed: bool,
}

struct TaskList {
    tasks: HashMap<usize, Task>,
    next_id: usize,
}

impl TaskList {
    fn new() -> TaskList {
        TaskList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: &str) -> Task {
        let task = Task {
            id: self.next_id,
            description: String::from(description),
            completed: false,
        };

        self.tasks.insert(self.next_id, task.clone());
        self.next_id += 1;

        task
    }

    fn complete_task(&mut self, id: usize) -> Option<&Task> {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.completed = true;
            Some(task)
        } else {
            None
        }
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks in the ToDo list.");
        } else {
            println!("Task List:");
            for (id, task) in &self.tasks {
                println!("ID: {}, Description: {}, Completed: {}", id, task.description, task.completed);
            }
        }
    }
}

fn main() {
    let mut todo_list = TaskList::new();

    let task1 = todo_list.add_task("Complete Rust program");
    let task2 = todo_list.add_task("Learn a new programming language");

    todo_list.list_tasks();

    todo_list.complete_task(task1.id);

    println!("After completing a task:");
    todo_list.list_tasks();
}
