use sqlite;

pub fn connect() -> sqlite::Connection {
    let conn = sqlite::open("tudus.db");
    create_table(&conn.as_ref().unwrap());
    conn.unwrap()
}

pub fn create_table(conn: &sqlite::Connection) {
    // Create a table for tudus
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
