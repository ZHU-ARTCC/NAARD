use rusqlite::{params, Connection};
use std::error::Error;
use std::fs;
use std::io;
use std::io::BufReader;

use aixm::Airports;

static DB_DDL: &'static str = include_str!("sql/ddl.sql");

fn main() -> Result<(), Box<Error>> {
    match fs::remove_file("naard.db") {
        Ok(_) => (),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => (),
        Err(e) => return Err(e.into()),
    }

    let apt = BufReader::new(fs::File::open("/home/cpage/APT_AIXM.xml")?);

    let mut conn = Connection::open("naard.db")?;

    // Build DDL
    conn.execute_batch(DB_DDL).expect("DDL Build Failed!");
    let bulk_load_tx = conn.transaction()?;
    {
        let mut airport_stmt = bulk_load_tx.prepare(
            "INSERT INTO airport_heliport (id, designator, name, arp_x, arp_y) VALUES (?,?,?,?,?)",
        )?;

        for (id, airport) in Airports::from_reader(apt).flat_map(|x| x).enumerate() {
            let id = id as u32;
            let arp = airport.arp.and_then(|x| x.point);

            if let Some(arp) = arp {
                let x = arp.x;
                let y = arp.y;
                airport_stmt.execute(params![
                    id,
                    airport.designator,
                    airport.name,
                    x as f64,
                    y as f64
                ])?;
            }
        }
    }
    bulk_load_tx.commit()?;

    Ok(())
}
