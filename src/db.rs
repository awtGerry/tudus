use sqlite;
use std::path::PathBuf;
use dirs::data_dir;

pub fn connect() -> sqlite::Connection {
    let path = default_db_path();
    let path = match path {
        Ok(path) => path,
        Err(_) => panic!("Could not get the default path for the database"),
    };
    let conn = sqlite::open(path);
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

fn default_db_path() -> Result<PathBuf, std::io::Error> {
    let mut path = data_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Data directory not found",
    ))?;
    // Create a directory for the app
    path.push("tudus");
    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&path)?;
    // Add the database file to the path
    path.push("tudus.db");
    Ok(path)
}
