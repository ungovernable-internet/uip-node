//! Content storage module

use rusqlite::{params, Connection, Result};

pub struct ContentStore {
    conn: Connection,
}

impl ContentStore {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS content (
                hash TEXT PRIMARY KEY,
                data BLOB NOT NULL
            )",
            [],
        )?;
        Ok(ContentStore { conn })
    }
    pub fn store(&self, hash: &str, data: &[u8]) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO content (hash, data) VALUES (?1, ?2)",
            params![hash, data],
        )?;
        Ok(())
    }
    pub fn retrieve(&self, hash: &str) -> Result<Option<Vec<u8>>> {
        let mut stmt = self.conn.prepare("SELECT data FROM content WHERE hash = ?1")?;
        let mut rows = stmt.query(params![hash])?;
        if let Some(row) = rows.next()? {
            let data: Vec<u8> = row.get(0)?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }
}
