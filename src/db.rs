use rusqlite::Connection;

pub fn init() -> anyhow::Result<()> {
    let conn = Connection::open_in_memory()?;

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

pub struct History {
    pub id: i32,
    pub created_at: i64,
    pub preset: String,
    pub args: String,
}

pub fn last() -> rusqlite::Result<History> {
    let conn = Connection::open_in_memory()?;

    let mut result = conn
        .prepare("SELECT id, created_at, preset, args FROM history ORDER BY created_at LIMIT 1")?;

    result.query_row([], |row| {
        Ok(History {
            id: row.get(0)?,
            created_at: row.get(1)?,
            preset: row.get(2)?,
            args: row.get(3)?,
        })
    })
}
