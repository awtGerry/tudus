use chrono;
use chrono::prelude::*;
use sqlite::State;
use sqlite;
use crate::db;

use iced::Sandbox;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tudu {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub created_at: Option<DateTime<Local>>,
    pub due_date: Option<DateTime<Local>>,
    pub reminder: bool,
    pub reminder_at: Option<DateTime<Local>>,
    pub completed: bool,

    #[serde(skip)]
    state: TuduState,
}

#[derive(Debug, Clone)]
pub enum TuduState {
    Idle,
    Editing
}

impl Default for TuduState {
    fn default() -> Self {
        TuduState::Idle
    }
}

#[derive(Debug, Clone)]
pub enum TuduMessage {
    Edit,
}

impl Sandbox for Tudu {
    type Message = TuduMessage;

    pub fn new(name: String, description: String) -> Self {
        Tudu {
            id: None,
            name,
            description,
            created_at: Some(Local::now()),
            due_date: None,
            reminder: false,
            reminder_at: None,
            completed: false,
            state: TuduState::Idle,
        }
    }

    /* pub fn set_due_date(&mut self, due_date: DateTime<Local>) {
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
    } */
}

pub fn save(tudu: &Tudu) {
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
            &tudu.name,
            &tudu.description,
            &tudu.created_at.unwrap_or(Local::now()).to_rfc3339().to_string(),
            // add 1 day to due_date if not set
            &tudu.due_date.unwrap_or(Local::now() + chrono::Duration::days(1)).to_rfc3339().to_string(),
            if tudu.reminder { 1 } else { 0 },
            // If reminder is set, use reminder_at, else use empty string
            if tudu.reminder { tudu.reminder_at.unwrap().to_rfc3339().to_string() } else { "".to_string() },
            if tudu.completed { 1 } else { 0 },
    );

    conn.execute(&query).unwrap();
}

pub fn get_all() -> Vec<Tudu> {
    let conn = db::connect();
    let mut stmt = conn.prepare("SELECT * FROM tudus").unwrap();
    let mut tudus = Vec::new();
    while let State::Row = stmt.next().unwrap() {
        let id = stmt.read::<i64, _>("id").unwrap();
        let name = stmt.read::<String, _>("name").unwrap();
        let description = stmt.read::<String, _>("description").unwrap();
        let created_at = stmt.read::<String, _>("created_at").unwrap();
        let due_date = stmt.read::<String, _>("due_date").unwrap();
        let reminder = stmt.read::<i64, _>("reminder").unwrap();
        let reminder_at = stmt.read::<String, _>("reminder_at").unwrap();
        let completed = stmt.read::<i64, _>("completed").unwrap();
        tudus.push(Tudu {
            id: Some(id),
            name,
            description,
            created_at: Some(DateTime::parse_from_rfc3339(&created_at).unwrap().into()),
            due_date: Some(DateTime::parse_from_rfc3339(&due_date).unwrap().into()),
            reminder: reminder == 1,
            reminder_at: if reminder == 1 { Some(DateTime::parse_from_rfc3339(&reminder_at).unwrap().into()) } else { None },
            completed: completed == 1,
            state: TuduState::Idle,
        });
    }
    tudus
}
