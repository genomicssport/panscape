use crate::filesplitpattern::fastareturn;
use crate::nanoporepacbio::{CdsExtract, CdsExtractSeq, Sequence};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

/*
 Author Gaurav Sablok
 SLB Potsdam
 Date: 2025-2-20

*/

pub fn computeintergenic(
    pathalignment: &str,
    pathreadsfasta: &str,
) -> Result<String, Box<dyn Error>> {
    let sequenceundone: Vec<Sequence> = fastareturn(pathreadsfasta).unwrap();

    let genomescale = File::open(pathalignment).expect("file not present");
    let genomeread = BufReader::new(genomescale);

    let mut linevecstore: Vec<String> = Vec::new();
    let mut alignmentids: Vec<String> = Vec::new();
    let mut uniqueids: HashSet<String> = HashSet::new();

    for i in genomeread.lines() {
        let line = i.expect("line not found");
        let linevec: Vec<String> = line.split("\t").map(|x| x.to_string()).collect::<Vec<_>>();
        alignmentids.push(linevec[0].clone());
        uniqueids.insert(linevec[0].clone());
        linevecstore.push(line);
    }

    let mut extract: Vec<CdsExtract> = Vec::new();
    let mut extractseq: Vec<CdsExtractSeq> = Vec::new();

    for i in uniqueids.iter() {
        for j in linevecstore.iter() {
            let linecheck = j.split("\t").collect::<Vec<_>>();
            if linecheck[0] == i && linecheck[2] == "CDS" {
                let mut cdsall: Vec<(usize, usize)> = Vec::new();
                let cootuple: (usize, usize) = (
                    linecheck[3].parse::<usize>().unwrap(),
                    linecheck[4].parse::<usize>().unwrap(),
                );
                cdsall.push(cootuple);
                extract.push(CdsExtract {
                    header: i,
                    cdsordinate: cdsall.clone(),
                })
            }
        }
    }

    for i in uniqueids.iter() {
        for j in linevecstore.iter() {
            for seq in sequenceundone.iter() {
                let linecheck = j.split("\t").collect::<Vec<_>>();
                if linecheck[0] == i && linecheck[2] == "CDS" && linecheck[0] == seq.header {
                    let mut cdsall: Vec<(usize, usize)> = Vec::new();
                    let cootuple: (usize, usize) = (
                        linecheck[3].parse::<usize>().unwrap(),
                        linecheck[4].parse::<usize>().unwrap(),
                    );
                    let mut cdsseq: Vec<String> = Vec::new();
                    cdsall.push(cootuple);
                    cdsseq.push(
                        seq.sequence[linecheck[3].parse::<usize>().unwrap()
                            ..linecheck[4].parse::<usize>().unwrap()]
                            .to_string(),
                    );
                    extractseq.push(CdsExtractSeq {
                        header: i,
                        cdsordinate: cdsall.clone(),
                        cdsextract: cdsseq.clone(),
                    })
                }
            }
        }
    }

    let mut extractwrite = File::create("extract-bed-cds-coordinate.txt").expect("file not found");
    let mut extractseqwrite = File::create("cds-ccordinates.txt").expect("file not found");

    for i in extract.iter() {
        writeln!(extractwrite, "{}\t{:?}", i.header, i.cdsordinate).expect("line not found");
    }

    for i in extractseq.iter() {
        writeln!(
            extractseqwrite,
            "{}\t{:?}\t{:?}",
            i.header, i.cdsordinate, i.cdsextract
        )
        .expect("line not found");
    }

    Ok("The results have been written".to_string())
}
