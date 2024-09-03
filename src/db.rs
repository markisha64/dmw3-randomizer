use rusqlite::Connection;

pub fn init() -> anyhow::Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "
            CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY,
                created_at INTEGER NOT NULL,
                preset TEXT NOT NULL,
                arguments TEXT NOT NULL,
            );
        ",
        (),
    )?;

    Ok(())
}
