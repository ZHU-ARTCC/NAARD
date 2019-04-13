use rusqlite::{Connection, params};
use std::error::Error;
use std::io;
use std::fs;

static DB_DDL : &'static str = include_str!("sql/ddl.sql");

fn main() -> Result<(), Box<Error>>{

    match fs::remove_file("naard.db") {
        Ok(_) => (),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => (),
        Err(e) => return Err(e.into())
    }

    let conn = Connection::open("naard.db")?;

    // Build DDL
    conn.execute_batch(DB_DDL).expect("DDL Build Failed!");
    Ok(())
}
