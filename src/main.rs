use rusqlite::{params, Connection, config::DbConfig};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::io;
use std::io::BufReader;
use structopt::StructOpt;
use aixm::airport::{AirportScan, RunwayScan};

static DB_DDL: &'static str = include_str!("sql/ddl.sql");

#[derive(Debug, StructOpt)]
struct Args {
    input: PathBuf,
    #[structopt(default_value = "naard.db")]
    output: PathBuf
}

#[inline]
fn reset_reader<B: io::Read + io::Seek>(mut reader: B) {
    reader.seek(io::SeekFrom::Start(0));
}

fn main() -> Result<(), Box<Error>> {
    let args = Args::from_args();
    match fs::remove_file(&args.output) {
        Ok(_) => (),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => (),
        Err(e) => return Err(e.into()),
    }

    let mut apt = BufReader::new(fs::File::open(args.input)?);

    let mut conn = Connection::open(&args.output)?;
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

        let mut runway_end_stmt = bulk_load_tx.prepare(
            "INSERT INTO runway_end (runway, designator, base_end) VALUES (?,?,?)",
        )?;

        for runway in RunwayScan::from_reader(&mut apt).flat_map(|x| x) {
            
            let res = if runway.id.contains("END") {     
                let (base_id, is_base) = if runway.id.contains("BASE") {
                    (runway.id.replace("BASE_END_", ""), true)
                } else {
                    (runway.id.replace("RECIPROCAL_END_", ""), false)
                };
                
                runway_end_stmt.execute(params![
                    base_id,
                    runway.designator,
                    is_base
                ])
            } else {
                runway_stmt.execute(params![
                    runway.id,
                    runway.designator,
                    runway.assoc_airport
                ])
            };

            if let Err(res) = res {
                println!("{:?}\n--FAILING STRUCT--\n{:?}", res, runway);
            }
        }
    }
    bulk_load_tx.commit()?;

    Ok(())
}
