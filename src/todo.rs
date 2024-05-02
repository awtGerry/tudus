/*
   Linked list implementation of a todo list

   List of operations:
   - Add a task
   - Remove a task
   - Toggle a task as done
   - Edit a task
   - Categorize a task
   - List all tasks
   - List all tasks that are done
   - List all tasks that are not done
   - List all tasks that are due today
   - List all tasks that are due within a week
*/

use std::collections::HashMap;
use std::fmt;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub due_date: Option<SystemTime>,
    pub done: bool,
    pub category: Option<String>,
}

impl Task {
    pub fn new(id: u32, title: String, description: String, due_date: Option<SystemTime>, done: bool, category: Option<String>) -> Task {
        Task {
            id,
            title,
            description,
            due_date,
            done,
            category,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let due_date = if let Some(due_date) = self.due_date {
            let due_date = due_date.duration_since(SystemTime::UNIX_EPOCH).unwrap();
            let due_date = due_date.as_secs();
            let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
            if due_date < now {
                "overdue".to_string()
            } else {
                let days = (due_date - now) / 86400;
                format!("due in {} days", days)
            }
        } else {
            "no due date".to_string()
        };
        let category = if let Some(category) = &self.category {
            category.clone()
        } else {
            "no category".to_string()
        };
        write!(f, "Task #{}: {}\nDescription: {}\nDue date: {}\nStatus: {}\nCategory: {}\n", self.id, self.title, self.description, due_date, if self.done { "done" } else { "not done" }, category)
    }
}

pub struct Todo {
    pub tasks: HashMap<u32, Task>,
    next_id: u32,
}

#[allow(unused)]
impl Todo {
    pub fn new() -> Todo {
        Todo {
            tasks: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_task(&mut self, title: String, description: String, due_date: Option<SystemTime>, category: Option<String>) -> u32 {
        let task = Task::new(self.next_id, title, description, due_date, false, category);
        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
        self.next_id - 1
    }

    pub fn remove_task(&mut self, id: u32) -> bool {
        self.tasks.remove(&id).is_some()
    }

    pub fn toggle_task(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.done = !task.done;
            true
        } else {
            false
        }
    }

    pub fn edit_task(&mut self, id: u32, title: String, description: String, due_date: Option<SystemTime>, category: Option<String>) -> bool {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.title = title;
            task.description = description;
            task.due_date = due_date;
            task.category = category;
            true
        } else {
            false
        }
    }

    pub fn categorize_task(&mut self, id: u32, category: String) -> bool {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.category = Some(category);
            true
        } else {
            false
        }
    }

    pub fn list_all_tasks(&self) -> Vec<Task> {
        self.tasks.values().cloned().collect()
    }

    pub fn list_all_done_tasks(&self) -> Vec<Task> {
        self.tasks.values().filter(|task| task.done).cloned().collect()
    }

    pub fn list_all_not_done_tasks(&self) -> Vec<Task> {
        self.tasks.values().filter(|task| !task.done).cloned().collect()
    }

    pub fn list_all_tasks_due_today(&self) -> Vec<Task> {
        let now = SystemTime::now();
        self.tasks.values().filter(|task| {
            if let Some(due_date) = task.due_date {
                let due_date = due_date.duration_since(SystemTime::UNIX_EPOCH).unwrap();
                let now = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
                due_date.as_secs() == now.as_secs()
            } else {
                false
            }
        }).cloned().collect()
    }

    pub fn list_all_tasks_due_within_week(&self) -> Vec<Task> {
        let now = SystemTime::now();
        self.tasks.values().filter(|task| {
            if let Some(due_date) = task.due_date {
                let due_date = due_date.duration_since(SystemTime::UNIX_EPOCH).unwrap();
                let now = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
                due_date.as_secs() - now.as_secs() <= 604800
            } else {
                false
            }
        }).cloned().collect()
    }

    pub fn list_all_tasks_by_category(&self, category: String) -> Vec<Task> {
        self.tasks.values().filter(|task| {
            if let Some(task_category) = &task.category {
                task_category == &category
            } else {
                false
            }
        }).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut todo = Todo::new();
        let id = todo.add_task("Task 1".to_string(), "Description 1".to_string(), None, None);
        assert_eq!(id, 0);
    }

    #[test]
    fn test_remove_task() {
        let mut todo = Todo::new();
        todo.add_task("Task 1".to_string(), "Description 1".to_string(), None, None);
        assert_eq!(todo.remove_task(0), true);
        assert_eq!(todo.remove_task(0), false);
    }

    #[test]
    fn test_toggle_task() {
        let mut todo = Todo::new();
        todo.add_task("Task 1".to_string(), "Description 1".to_string(), None, None);
        assert_eq!(todo.toggle_task(0), true);
        assert_eq!(todo.toggle_task(0), true);
    }

    #[test]
    fn test_edit_task() {
        let mut todo = Todo::new();
        todo.add_task("Task 1".to_string(), "Description 1".to_string(), None, None);
        assert_eq!(todo.edit_task(0, "Task 2".to_string(), "Description 2".to_string(), None, None), true);
        assert_eq!(todo.edit_task(1, "Task 2".to_string(), "Description 2".to_string(), None, None), false);
    }

    #[test]
    fn test_categorize_task() {
        let mut todo = Todo::new();
        todo.add_task("Task 1".to_string(), "Description 1".to_string(), None, None);
        assert_eq!(todo.categorize_task(0, "Category 1".to_string()), true);
        assert_eq!(todo.categorize_task(1, "Category 1".to_string()), false);
    }

    #[test]
    fn test_list_all_tasks() {
        let mut todo = Todo::new();
        todo.add_task("Task 1".to_string(), "Description 1".to_string(), None, None);
        todo.add_task("Task 2".to_string(), "Description 2".to_string(), None, None);
        let tasks = todo.list_all_tasks();
        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn test_list_all_done_tasks() {
        let mut todo = Todo::new();
        todo.add_task("Task 1".to_string(), "Description 1".to_string(), None, None);
        todo.add_task("Task 2".to_string(), "Description 2".to_string(), None, None);
        todo.toggle_task(0);
        let tasks = todo.list_all_done_tasks();
        assert_eq!(tasks.len(), 1);
    }

    #[test]
    fn test_list_all_not_done_tasks() {
        let mut todo = Todo::new();
        todo.add_task("Task 1".to_string(), "Description 1".to_string(), None, None);
        todo.add_task("Task 2".to_string(), "Description 2".to_string(), None, None);
        todo.toggle_task(0);
        let tasks = todo.list_all_not_done_tasks();
        assert_eq!(tasks.len(), 1);
    }

    #[test]
    fn test_list_all_tasks_due_today() {
        let mut todo = Todo::new();
        todo.add_task("Task 1".to_string(), "Description 1".to_string(), Some(SystemTime::now()), None);
        todo.add_task("Task 2".to_string(), "Description 2".to_string(), None, None);
        let tasks = todo.list_all_tasks_due_today();
        assert_eq!(tasks.len(), 1);
    }

    #[test]
    fn test_list_all_tasks_due_within_week() {
        let mut todo = Todo::new();
        todo.add_task("Task 1".to_string(), "Description 1".to_string(), Some(SystemTime::now()), None);
        todo.add_task("Task 2".to_string(), "Description 2".to_string(), Some(SystemTime::now()), None);
        let tasks = todo.list_all_tasks_due_within_week();
        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn test_list_all_tasks_by_category() {
        let mut todo = Todo::new();
        todo.add_task("Task 1".to_string(), "Description 1".to_string(), None, Some("Category 1".to_string()));
        todo.add_task("Task 2".to_string(), "Description 2".to_string(), None, Some("Category 2".to_string()));
        let tasks = todo.list_all_tasks_by_category("Category 1".to_string());
        assert_eq!(tasks.len(), 1);
    }
}
