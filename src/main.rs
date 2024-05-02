mod todo;

fn main() {
    let mut todo = todo::Todo::new(1, "Learn Rust".to_string(), "2021-09-01".to_string(), false);
    todo.print();
    todo.toggle();
    todo.print();
}
