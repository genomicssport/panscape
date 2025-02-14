use crate::nanoporepacbio::Sequence;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 Author Gaurav Sablok
 SLB Potsdam
 Date: 2025-2-14

*/

pub fn fastareturn(path: &str) -> Result<Vec<Sequence>, Box<dyn Error>> {
    let fileopen = File::open(path).expect("path not found");
    let fileread = BufReader::new(fileopen);
    let mut sequencehead: Vec<String> = Vec::new();
    let mut sequenceseq: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        if line.starts_with("@") {
            sequencehead.push(line.replace("@", ""));
        } else if line.starts_with("+") || line.starts_with("-") {
            continue;
        } else if line.starts_with("A") && !line.contains("~")
            || !line.contains("?")
            || !line.contains("^")
            || !line.contains(";")
            || !line.contains("@")
            || !line.contains("<")
            || !line.contains("_")
            || !line.contains("}")
        {
            sequenceseq.push(line)
        } else if line.starts_with("T") && !line.contains("~")
            || !line.contains("?")
            || !line.contains("^")
            || !line.contains(";")
            || !line.contains("@")
            || !line.contains("<")
            || !line.contains("_")
            || !line.contains("}")
        {
            sequenceseq.push(line)
        } else if line.starts_with("G") && !line.contains("~")
            || !line.contains("?")
            || !line.contains("^")
            || !line.contains(";")
            || !line.contains("@")
            || !line.contains("<")
            || !line.contains("_")
            || !line.contains("}")
        {
            sequenceseq.push(line)
        } else if line.starts_with("C") && !line.contains("~")
            || !line.contains("?")
            || !line.contains("^")
            || !line.contains(";")
            || !line.contains("@")
            || !line.contains("<")
            || !line.contains("_")
            || !line.contains("}")
        {
            sequenceseq.push(line)
        }
    }

    let mut combinedseq: Vec<Sequence> = Vec::new();
    for i in 0..sequencehead.len() {
        combinedseq.push(Sequence {
            header: sequencehead[i].clone(),
            sequence: sequenceseq[i].clone(),
        })
    }

    Ok(combinedseq)
}
