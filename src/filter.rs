use crate::filesplitpattern::fastareturn;
use crate::nanoporepacbio::Sequence;
use std::error::Error;
use std::fs::File;
use std::io::Write;

/*
 Author Gaurav Sablok
 SLB Potsdam
 Date: 2025-2-14

*/

pub fn readlength(path: &str, length: &str) -> Result<String, Box<dyn Error>> {
    let readiter: Vec<Sequence> = fastareturn(path).unwrap();
    let lengththreshold: usize = length.parse::<usize>().unwrap();

    let mut filterseq: Vec<Sequence> = Vec::new();

    for i in readiter.iter() {
        if i.sequence.len() >= lengththreshold {
            filterseq.push(Sequence {
                header: i.header.clone(),
                sequence: i.sequence.clone(),
            });
        } else {
            continue;
        }
    }

    let mut filewrite = File::create("filtered-ones.fasta").expect("file not found");
    for i in filterseq.iter() {
        writeln!(filewrite, ">{}\n{}", i.header, i.sequence).expect("line not found");
    }

    Ok("The filtered reads have been written".to_string())
}
