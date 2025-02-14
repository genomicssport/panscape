use crate::filesplitpattern::fastareturn;
use crate::nanoporepacbio::Sequence;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
 Author Gaurav Sablok
 SLB Potsdam
 Date: 2025-2-14

*/

pub fn selected(path: &str, ids: &str) -> Result<String, Box<dyn Error>> {
    let unpackreads: Vec<Sequence> = fastareturn(path).unwrap();
    let mut idscheck: Vec<String> = Vec::new();

    let idsopen = File::open(ids).expect("file not present");
    let idsread = BufReader::new(idsopen);
    for i in idsread.lines() {
        let line = i.expect("line not present");
        idscheck.push(line.trim().to_string());
    }

    let mut selectedones: Vec<Sequence> = Vec::new();
    for i in unpackreads.iter() {
        for j in idscheck.iter() {
            if i.header == j.to_string() {
                selectedones.push(Sequence {
                    header: i.header.clone(),
                    sequence: i.sequence.clone(),
                })
            }
        }
    }

    let mut filewrite = File::create("selectedones.fasta").expect("file not present");
    for i in selectedones.iter() {
        writeln!(filewrite, ">{}\n{}", i.header, i.sequence).expect("file not present");
    }

    Ok("The selected reads have been written".to_string())
}
