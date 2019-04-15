use rusqlite::{params, Connection, config::DbConfig};
use std::error::Error;
use std::fs;
use std::io;
use std::io::BufReader;

use aixm::airport::{AirportScan, RunwayScan};

static DB_DDL: &'static str = include_str!("sql/ddl.sql");

#[inline]
fn reset_reader<B: io::Read + io::Seek>(mut reader: B) {
    reader.seek(io::SeekFrom::Start(0));
}

fn main() -> Result<(), Box<Error>> {
    match fs::remove_file("naard.db") {
        Ok(_) => (),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => (),
        Err(e) => return Err(e.into()),
    }

    let mut apt = BufReader::new(fs::File::open("C:\\Users\\Carson\\Documents\\vZHU\\NASR\\Additional_Data\\AIXM\\AIXM_5.1\\XML-Subscriber-Files\\APT_AIXM.xml")?);

    let mut conn = Connection::open("naard.db")?;
    // Enable FK enforcement
    conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true)?;

    // Build DDL
    conn.execute_batch(DB_DDL).expect("DDL Build Failed!");
    let bulk_load_tx = conn.transaction()?;
    {
        let mut airport_stmt = bulk_load_tx.prepare(
            "INSERT INTO airport_heliport (id, designator, name, arp_x, arp_y) VALUES (?,?,?,?,?)",
        )?;

        for airport in AirportScan::from_reader(&mut apt).flat_map(|x| x) {
            let arp = airport.arp.and_then(|x| x.point);

            if let Some(arp) = arp {
                let x = arp.x;
                let y = arp.y;
                airport_stmt.execute(params![
                    airport.id,
                    airport.designator,
                    airport.name,
                    x as f64,
                    y as f64
                ])?;
            }
        }
        reset_reader(&mut apt);

        let mut runway_stmt = bulk_load_tx.prepare(
            "INSERT INTO runway (id, designator, associated_airport) VALUES (?,?,?)",
        )?;

        for runway in RunwayScan::from_reader(&mut apt).flat_map(|x| x) {
            let res = runway_stmt.execute(params![
                runway.id,
                runway.designator,
                runway.assoc_airport
            ]);

            if let Err(res) = res {
                println!("{:?}\n--FAILING STRUCT--\n{:?}", res, runway);
            }
        }
    }
    bulk_load_tx.commit()?;

    Ok(())
}
