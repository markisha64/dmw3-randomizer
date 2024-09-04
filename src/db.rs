use std::{error::Error, path::PathBuf};

use chrono::Utc;
use home::home_dir;
use rusqlite::Connection;

use crate::{cli::Arguments, json::Preset};

fn init_homedir() -> anyhow::Result<PathBuf> {
    let mut home = match home_dir() {
        Some(home) => home,
        None => return Err(anyhow::anyhow!("Can't determine home directory")),
    };

    home.push(".dmw3-randomizer");

    if !home.exists() {
        std::fs::create_dir(&home)?;
    }

    home.push("db");

    Ok(home)
}

fn get_conn() -> rusqlite::Result<Connection> {
    match init_homedir() {
        Ok(path) => Connection::open(path),
        Err(_) => Connection::open("db"),
    }
}

pub fn init() -> anyhow::Result<()> {
    let conn = get_conn()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY,
                created_at INTEGER NOT NULL,
                preset TEXT NOT NULL,
                arguments TEXT NOT NULL
            );",
        (),
    )?;

    Ok(())
}

#[derive(Debug)]
pub struct History {
    pub _id: i32,
    pub created_at: i64,
    pub preset: String,
    pub arguments: String,
}

pub fn last() -> rusqlite::Result<History> {
    let conn = get_conn()?;

    let mut result = conn.prepare(
        "SELECT id, created_at, preset, arguments FROM history ORDER BY created_at DESC LIMIT 1",
    )?;

    let s = result.query_row([], |row| {
        Ok(History {
            _id: row.get(0)?,
            created_at: row.get(1)?,
            preset: row.get(2)?,
            arguments: row.get(3)?,
        })
    })?;

    Ok(s)
}

pub fn insert(preset: &Preset, arguments: &Arguments) -> Result<(), Box<dyn Error>> {
    let conn = get_conn()?;

    conn.execute(
        "INSERT INTO history (created_at, preset, arguments) VALUES (?1, ?2, ?3)",
        (
            Utc::now().timestamp() as u64,
            serde_json::to_string::<Preset>(preset)?,
            serde_json::to_string::<Arguments>(arguments)?,
        ),
    )?;

    Ok(())
}

pub fn history() -> rusqlite::Result<Vec<History>> {
    let conn = get_conn()?;

    let mut qres = conn.prepare(
        "SELECT id, created_at, preset, arguments FROM history ORDER BY created_at DESC",
    )?;

    let rows = qres.query_map([], |row| {
        Ok(History {
            _id: row.get(0)?,
            created_at: row.get(1)?,
            preset: row.get(2)?,
            arguments: row.get(3)?,
        })
    })?;

    let result: Vec<_> = rows.collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(result)
}
