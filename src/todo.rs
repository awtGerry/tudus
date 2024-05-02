pub struct Todo {
    id: u32,
    title: String,
    date: String,
    completed: bool,
}

impl std::default::Default for Todo {
    fn default() -> Todo {
        return Todo {
            id: 0,
            title: String::from(""),
            date: String::from(""),
            completed: false,
        };
    }
}

/* pub enum TodoError {
    InvalidId,
    InvalidTitle,
    InvalidDate,
} */

impl Todo {
    pub fn new(id: u32, title: String, date: String, completed: bool) -> Todo {
        return Todo {
            id,
            title,
            date,
            completed,
        };
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }

    pub fn print(&self) {
        println!("{} - {} - {} - {}", self.id, self.title, self.date, self.completed);
    }

}
