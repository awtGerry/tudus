mod todo;

fn main() {
    let mut todo = todo::Todo::new(1, String::from("Buy milk"), String::from("2021-01-01"), false);
    todo.print();
    todo.toggle();
    todo.print();
}
