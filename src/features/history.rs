use rusqlite::{Connection, Result, params};
use std::path::PathBuf;

pub struct History {
    conn: Connection,
}

#[derive(Debug)]
pub struct HistoryEntry {
    pub url: String,
    pub title: String,
    pub visited_at: i64,
}

impl History {
    pub fn open() -> Result<Self> {
        let path = data_dir().join("history.db");
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS history (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                url         TEXT NOT NULL,
                title       TEXT NOT NULL DEFAULT '',
                visited_at  INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_visited ON history(visited_at DESC);",
        )?;
        Ok(Self { conn })
    }

    pub fn add(&self, url: &str, title: &str) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        self.conn.execute(
            "INSERT INTO history (url, title, visited_at) VALUES (?1, ?2, ?3)",
            params![url, title, now],
        )?;
        Ok(())
    }

    pub fn recent(&self, limit: usize) -> Result<Vec<HistoryEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT url, title, visited_at FROM history ORDER BY visited_at DESC LIMIT ?1",
        )?;
        let entries = stmt
            .query_map(params![limit as i64], |row| {
                Ok(HistoryEntry {
                    url: row.get(0)?,
                    title: row.get(1)?,
                    visited_at: row.get(2)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(entries)
    }

    pub fn search(&self, query: &str) -> Result<Vec<HistoryEntry>> {
        let pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT url, title, visited_at FROM history
             WHERE url LIKE ?1 OR title LIKE ?1
             ORDER BY visited_at DESC LIMIT 20",
        )?;
        let entries = stmt
            .query_map(params![pattern], |row| {
                Ok(HistoryEntry {
                    url: row.get(0)?,
                    title: row.get(1)?,
                    visited_at: row.get(2)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(entries)
    }
}

fn data_dir() -> PathBuf {
    dirs_next().join("browser")
}

fn dirs_next() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(home).join("Library").join("Application Support")
    }
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(home).join(".local").join("share")
    }
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA").unwrap_or_default();
        PathBuf::from(appdata)
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    PathBuf::from(".")
}
