use sqlite;

pub fn connect() -> sqlite::Connection {
    sqlite::open("tudus.db").unwrap()
}

pub fn create_table(conn: &sqlite::Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tudus (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            created_at TEXT NOT NULL,
            due_date TEXT,
            reminder INTEGER,
            reminder_at TEXT,
            completed INTEGER
        )",
    )
    .unwrap();
}
