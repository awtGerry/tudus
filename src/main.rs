mod todo;

fn main() {
    let mut todo = todo::Todo::new();
    todo.add_task("Buy milk".to_string(), "Buy before night".to_string(), None, None);
    todo.add_task("Buy bread".to_string(), "Buy before night".to_string(), None, None);
    todo.toggle_task(0);
    for task in todo.list_all_tasks() {
        println!("Task: {}", task.title);
    }
}
