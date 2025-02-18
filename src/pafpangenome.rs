use crate::nanoporepacbio::Pafanalyzer;
use crate::nanoporepacbio::Tableinformation;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tabled::Table;

/*
 Author Gaurav Sablok
 SLB Potsdam
 Date: 2025-2-18

*/

pub fn pangenome_summarize(path: &str) -> Result<String, Box<dyn Error>> {
    let pafopen = File::open(path).expect("file not found");
    let pafread = BufReader::new(pafopen);
    let mut pafstruct: Vec<Pafanalyzer> = Vec::new();
    for i in pafread.lines() {
        let line = i.expect("line not found");
        let pafvec = line.split("\t").collect::<Vec<_>>();
        pafstruct.push(Pafanalyzer {
            query: pafvec[0].to_string(),
            query_length: pafvec[1].parse::<usize>().unwrap(),
            query_start: pafvec[2].parse::<usize>().unwrap(),
            query_end: pafvec[3].parse::<usize>().unwrap(),
            strand: pafvec[4].to_string(),
            target: pafvec[5].to_string(),
            target_length: pafvec[6].parse::<usize>().unwrap(),
            target_start: pafvec[7].parse::<usize>().unwrap(),
            target_end: pafvec[8].parse::<usize>().unwrap(),
            residue_matches: pafvec[9].parse::<usize>().unwrap(),
            alignment_length: pafvec[10].parse::<usize>().unwrap(),
            quality: pafvec[11].parse::<usize>().unwrap(),
        });
    }

    let mut queryaligned: usize = 0usize;
    let mut targetaligned: usize = 0usize;
    let mut residuematches: usize = 0usize;
    let mut alignmentlength: usize = 0usize;

    for i in pafstruct.iter() {
        queryaligned += i.query_end - i.query_start
    }

    for i in pafstruct.iter() {
        targetaligned += i.target_end - i.target_start
    }

    for i in pafstruct.iter() {
        residuematches += i.residue_matches
    }

    for i in pafstruct.iter() {
        alignmentlength += i.alignment_length
    }

    let finaltable = vec![Tableinformation {
        information: "Genome",
        query: queryaligned,
        residue: residuematches,
        alignment: alignmentlength,
        target: targetaligned,
    }];

    let _finaltable = Table::new(finaltable).to_string();

    Ok("The pangenome has been summarized".to_string())
}
