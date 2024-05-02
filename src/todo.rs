pub struct Todo {
    id: u32,
    title: String,
    date: String,
    completed: bool,
}

/*
    Implementing the Todo struct with the following methods:
    - new -> creates a new Todo instance
    - toggle -> toggles the completed field of the Todo instance
    - get -> returns the Todo instance

    TODO:
    import database
    implement the methods to interact with the database
*/

impl Todo {
    pub fn new(id: u32, title: String, date: String, completed: bool) -> Self {
        Self {
            id,
            title,
            date,
            completed,
        }
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }

    pub fn print(&self) {
        println!("ID: {}", self.id);
        println!("Title: {}", self.title);
        println!("Date: {}", self.date);
        println!("Completed: {}", self.completed);
    }
}
