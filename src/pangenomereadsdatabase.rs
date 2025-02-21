use crate::filesplitpattern::fastareturn;
use rusqlite::{Connection, Result};
use std::error::Error;

/*
  Author Gaurav Sablok
  SLB Potsdam
  Date: 2025-2-21

*/

pub fn readsdatabase(pathreads: &str) -> Result<String, Box<dyn Error>> {
    let sequencereads = fastareturn(pathreads).unwrap();
    let conn = Connection::open("pangenomereads.db")?;
    conn.execute(
        "create table if not exists pangenomereads (
             id integer primary key,
             name text not null 
             sequence text not null
         )",
        [],
    )?;
    conn.execute(
        "create table if not exists pangenomereads (
             id integer primary key,
             name text not null,
             sequence text not null
         )",
        [],
    )?;

    for i in sequencereads.iter() {
        conn.execute(
            "INSERT INTO pangenomereads(name) VALUES (?1)",
            [i.header.clone()],
        )?;
    }
    for seq in sequencereads.iter() {
        conn.execute(
            "INSERT INTO pangenomereads(sequence) VALUES (?2)",
            [seq.sequence.clone()],
        )?;
    }

    Ok("The reads database has been generated".to_string())
}
