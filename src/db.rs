use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::result::Result::Ok;
use std::fmt;

#[derive(Debug)]
pub enum DbError {
    ConnectionFailed,
    QueryFailed,
}

impl Error for DbError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        match *self {
            DbError::ConnectionFailed => "Failed to connect to database",
            DbError::QueryFailed => "Failed to execute query",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl fmt::Display for DbError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DbError::ConnectionFailed => write!(f, "Failed to connect to database"),
            DbError::QueryFailed => write!(f, "Failed to execute query"),
        }
    }
}


#[derive(Debug)]
pub struct Log {
    pub id: u32,
    pub title: String,
    pub message: String,
    pub timestamp: String,
}

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new() -> Result<Db, DbError> {
        let conn = Connection::open("serialconnect.db").map_err(|_| DbError::ConnectionFailed)?;
        Ok(Db { conn })
    }

    pub fn init(&self) -> Result<(), DbError> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS logs (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL,
                    message TEXT NOT NULL,
                    timestamp DEFAULT CURRENT_TIMESTAMP
                )",
                params![],
            )
            .map_err(|_| DbError::QueryFailed)?;
        Ok(())
    }

    pub fn add_log(&self, title: &str, message: &str) -> Result<(), DbError> {
        self.conn
            .execute("INSERT INTO logs (title, message) VALUES (?1, ?2)", params![title, message])
            .map_err(|_| DbError::QueryFailed)?;

        Ok(())
    }

    pub fn update_log_title(&self, id: u32, title: &str) -> Result<(), DbError> {
        self.conn
            .execute(
                "UPDATE logs SET title = ?1 WHERE id = ?2",
                params![title, id],
            )
            .map_err(|_| DbError::QueryFailed)?;
        Ok(())
    }

    pub fn update_log(&self, id: u32, message: &str) -> Result<(), DbError> {
        self.conn
            .execute(
                "UPDATE logs SET message = ?1 WHERE id = ?2",
                params![message, id],
            )
            .map_err(|_| DbError::QueryFailed)?;
        Ok(())
    }

    pub fn get_logs(&self) -> Result<Vec<Log>, DbError> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM logs")
            .map_err(|_| DbError::QueryFailed)?;
        let logs_iter = stmt
            .query_map(params![], |row| {
                Ok(Log {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    message: row.get(2)?,
                    timestamp: row.get(3)?,
                })
            })
            .map_err(|_| DbError::QueryFailed)?;

        let mut logs = Vec::new();
        for log in logs_iter {
            logs.push(log.map_err(|_| DbError::QueryFailed)?);
        }
        Ok(logs)
    }
}

