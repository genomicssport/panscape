use crate::filesplitpattern::fastareturn;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;
/*
 Author Gaurav Sablok
 SLB Potsdam
 Date: 2025-2-18

*/

pub fn minimapalignment(
    pathreads: &str,
    pathminimap: &str,
    pathproteins: &str,
    thread: &str,
) -> Result<String, Box<dyn Error>> {
    let sequencevector = fastareturn(pathreads).unwrap();
    let proteinfileopen = File::open(pathproteins).expect("file not present");
    let proteinfileread = BufReader::new(proteinfileopen);
    let mut proteinheader: Vec<String> = Vec::new();
    let mut proteinsequence: Vec<String> = Vec::new();

    for i in proteinfileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            proteinheader.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            proteinsequence.push(line)
        }
    }

    let _command = Command::new(pathminimap)
        .arg("-t")
        .arg(thread)
        .arg(pathreads)
        .arg(pathproteins)
        .arg(">")
        .arg("proteinaligned.fasta")
        .output()
        .expect("command failed");

    let pafopen = File::open("proteinaligned.fasta").expect("file not present");

    let pafread = BufReader::new(pafopen);

    let mut storemap: BTreeMap<String, (usize, usize, String)> = BTreeMap::new();

    let mut m_rna_vec: Vec<(String, usize, usize)> = Vec::new();

    for i in pafread.lines() {
        let line = i.expect("line not present");
        if line.starts_with("#") {
            continue;
        } else if !line.starts_with("#") {
            let pafline = line.split("\t").collect::<Vec<_>>();
            if pafline[2].to_string() == "mRNA" {
                let paftuple = (
                    pafline[0].to_string(),
                    pafline[3].parse::<usize>().unwrap(),
                    pafline[4].parse::<usize>().unwrap(),
                );
                m_rna_vec.push(paftuple);
            }
        }
    }

    for i in m_rna_vec.iter() {
        for j in sequencevector.iter() {
            if i.0 == j.header {
                let sequencetuple: (usize, usize, String) =
                    (i.1, i.2, j.sequence[i.1..i.2].to_string());
                storemap.insert(i.0.clone(), sequencetuple);
            }
        }
    }

    let mut filewrite =
        File::create("mrna-predicted-reads-alignment.fasta").expect("file not found");

    for i in storemap.iter() {
        writeln!(filewrite, ">{}-{}-{}\n{}", i.0, i.1 .0, i.1 .2, i.1 .2).expect("file not found");
    }

    Ok("The results have been sent to the files: {:?}".to_string())
}
