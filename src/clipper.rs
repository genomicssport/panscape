use crate::filesplitpattern::fastareturn;
use crate::nanoporepacbio::Filesnatch;
use crate::nanoporepacbio::Sequence;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

pub fn clipperpattern(path: &str, regionfile: &str) -> Result<String, Box<dyn Error>> {
    let unpack: Vec<Sequence> = fastareturn(path).unwrap();
    let filecontent = File::open(regionfile).expect("file not present");
    let fileread = BufReader::new(filecontent);

    let mut fileanalyze: Vec<Filesnatch> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        let vecline = line.split(",").collect::<Vec<_>>();
        fileanalyze.push(Filesnatch {
            name: vecline[0].to_string(),
            start: vecline[1].parse::<usize>().unwrap(),
            end: vecline[2].parse::<usize>().unwrap(),
        })
    }

    let mut sequencesearch: Vec<Sequence> = Vec::new();
    for i in unpack.iter() {
        for j in fileanalyze.iter() {
            if i.header == j.name {
                sequencesearch.push(Sequence {
                    header: i.header.clone(),
                    sequence: i.sequence[j.start..j.end].to_string(),
                });
            }
        }
    }

    let mut filewrite = File::create("cliipedpattern.fasta").expect("file not present");
    for i in sequencesearch.iter() {
        writeln!(filewrite, "{:?}\t{:?}", i.header, i.sequence).expect("file not found");
    }

    Ok("The results have been written".to_string())
}
