use std::fs::File;
use std::io::{self, Read};
use rusqlite::{Connection, Result};

fn main() -> std::io::Result<()> {
    let conn = Connection::open("cards.db").unwrap();

    let sql = get_create_db_string("scripts/sqlite/create_text.sql")?;
    conn.execute(&sql, []).unwrap();

    let sql = get_create_db_string("scripts/sqlite/create_cards.sql")?;
    conn.execute(&sql, []).unwrap();

    Ok(())
}


fn get_create_db_string(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
