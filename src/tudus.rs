use chrono;
use chrono::prelude::*;
use sqlite::State;
use sqlite;
use crate::db;

#[allow(unused)]
#[derive(Debug)]
pub struct Tudu {
    pub name: String,
    pub description: String,
    pub created_at: Option<DateTime<Local>>,
    pub due_date: Option<DateTime<Local>>,
    pub reminder: bool,
    pub reminder_at: Option<DateTime<Local>>,
    pub completed: bool,
}

#[allow(unused)]
impl Tudu {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            created_at: Some(Local::now()),
            due_date: None,
            reminder: false,
            reminder_at: None,
            completed: false,
        }
    }

    pub fn set_due_date(&mut self, due_date: DateTime<Local>) {
        self.due_date = Some(due_date);
    }

    pub fn set_reminder(&mut self, reminder_at: DateTime<Local>) {
        self.reminder = true;
        self.reminder_at = Some(reminder_at);
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn uncomplete(&mut self) {
        self.completed = false;
    }

    pub fn save(&self) {
        let conn = db::connect();
        db::create_table(&conn);
        let query = format!(
            "INSERT INTO tudus (
                name,
                description,
                created_at,
                due_date,
                reminder,
                reminder_at,
                completed
            ) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}');",
                &self.name,
                &self.description,
                &self.created_at.unwrap_or(Local::now()).to_rfc3339().to_string(),
                // add 1 day to due_date if not set
                &self.due_date.unwrap_or(Local::now() + chrono::Duration::days(1)).to_rfc3339().to_string(),
                if self.reminder { 1 } else { 0 },
                // if reminder is set, use reminder_at, else leave it empty
                &self.reminder_at.unwrap().to_rfc3339().to_string(),
                if self.completed { 1 } else { 0 },
        );

        conn.execute(&query).unwrap();
    }
}

pub fn get_all() -> Vec<Tudu> {
    let mut tudus: Vec<Tudu> = Vec::new();
    let conn = db::connect();
    db::create_table(&conn);
    let query = "SELECT * FROM tudus";
    let mut stmt = conn.prepare(query).unwrap();
    stmt.bind((1, 50)).unwrap();

    while let Ok(State::Row) = stmt.next() {
        // let id = stmt.read::<i64, _>("id").unwrap();
        let name = stmt.read::<String, _>("name").unwrap();
        let description = stmt.read::<String, _>("description").unwrap();
        let created_at = stmt.read::<String, _>("created_at").unwrap();
        let due_date = stmt.read::<String, _>("due_date").unwrap();
        let reminder = stmt.read::<i64, _>("reminder").unwrap();
        let reminder_at = stmt.read::<String, _>("reminder_at").unwrap();
        let completed = stmt.read::<i64, _>("completed").unwrap();

        let tudu = Tudu {
            name,
            description,
            created_at: Some(DateTime::parse_from_rfc3339(&created_at).unwrap().into()),
            due_date: Some(DateTime::parse_from_rfc3339(&due_date).unwrap().into()),
            reminder: reminder == 1,
            reminder_at: Some(DateTime::parse_from_rfc3339(&reminder_at).unwrap().into()),
            completed: completed == 1,
        };

        tudus.push(tudu);
    }

    println!("{:?}", tudus);
    tudus
}
