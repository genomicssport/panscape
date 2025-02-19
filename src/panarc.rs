use crate::nanoporepacbio::{
    AlignmentGFF, CaptureSeq, Negative, Positive, Sequenceadd, CDS, MRNA, NEGATIVE, POSITIVE,
};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
#[allow(dead_code)]

/*

Author Gaurav Sablok
SLB Potsdam
Date: 2025-2-19

given an metagenome and the alignment files, will write all the annotations.
* */

pub fn metagenome_annotate(path: &str, fasta: &str) -> Result<String, Box<dyn Error>> {
    let mut vectorhold = Vec::new();
    let mut vectorstring: Vec<AlignmentGFF> = Vec::new();
    let f = File::open(&path).expect("file not present");
    let read = BufReader::new(f);
    for gffreadline in read.lines() {
        let gffline = gffreadline.expect("line not present");
        if gffline.starts_with("#") {
            continue;
        } else {
            vectorhold.push(gffline)
        }
    }

    for i in vectorhold.iter() {
        let addline = i.split("\t").collect::<Vec<&str>>();
        let idhold = addline[0];
        let genomefeaturehold = addline[2];
        let starthold = addline[3]
            .to_string()
            .parse::<usize>()
            .expect("number not present");
        let endhold = addline[4]
            .to_string()
            .parse::<usize>()
            .expect("number not present");
        let strandhold = addline[6].to_string();
        vectorstring.push(AlignmentGFF {
            id: idhold.to_string(),
            genomefeature: genomefeaturehold.to_string(),
            start: starthold,
            end: endhold,
            strand: strandhold.to_string(),
        })
    }

    let mut positive = Vec::new();
    let mut negative = Vec::new();
    let new_positive = vectorstring.clone();
    let new_negative = vectorstring.clone();
    for i in new_positive.into_iter() {
        if i.strand == "+" {
            positive.push(Positive {
                id: i.id,
                genomefeature: i.genomefeature,
                start: i.start,
                end: i.end,
                strand: i.strand,
            })
        }
    }
    for i in new_negative.into_iter() {
        if i.strand == "-" {
            negative.push(Negative {
                id: i.id,
                genomefeature: i.genomefeature,
                start: i.start,
                end: i.end,
                strand: i.strand,
            })
        }
    }

    let mut header = vec![];
    let mut sequence = vec![];
    let f = File::open(&fasta).expect("file not present");
    let read = BufReader::new(f);
    for i in read.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            header.push(line)
        } else {
            sequence.push(line)
        }
    }

    let mut final_seq: Vec<Sequenceadd> = Vec::new();
    for i in 0..header.len() {
        final_seq.push(Sequenceadd {
            id: header[i].to_string(),
            sequence: sequence[i].to_string(),
        })
    }

    let mut mRNAstruct: Vec<MRNA> = Vec::new();
    for i in final_seq.iter_mut() {
        for j in vectorstring.iter_mut() {
            if j.genomefeature == "mRNA" {
                let mutablename = j.id.to_string();
                let mutablestring = i.sequence[j.start - 1..j.end].to_string();
                let mutablestrand = j.strand.to_string();
                mRNAstruct.push(MRNA {
                    id: mutablename,
                    seq: mutablestring,
                    strand: mutablestrand,
                });
            }
        }
    }

    let mut cdsstruct: Vec<CDS> = Vec::new();
    for i in final_seq.iter_mut() {
        for j in vectorstring.iter_mut() {
            if j.genomefeature == "CDS" {
                let mutablename = j.id.to_string();
                let mutablestring = i.sequence[j.start - 1..j.end].to_string();
                let mutablestrand = j.strand.to_string();
                cdsstruct.push(CDS {
                    id: mutablename,
                    seq: mutablestring,
                    strand: mutablestrand,
                });
            }
        }
    }

    let mut positive: Vec<POSITIVE> = Vec::new();
    for i in final_seq.iter_mut() {
        for j in vectorstring.iter_mut() {
            if j.genomefeature == "CDS" && j.strand == "+" {
                let mutablename = j.id.to_string();
                let mutablestring = i.sequence[j.start - 1..j.end].to_string();
                let mutablestrand = j.strand.to_string();
                positive.push(POSITIVE {
                    id: mutablename,
                    seq: mutablestring,
                    strand: mutablestrand,
                });
            }
        }
    }

    let mut negative: Vec<NEGATIVE> = Vec::new();
    for i in final_seq.iter_mut() {
        for j in vectorstring.iter_mut() {
            if j.genomefeature == "CDS" && j.strand == "-" {
                let mutablename = j.id.to_string();
                let mutablestring = i.sequence[j.start - 1..j.end].to_string();
                let mutablestrand = j.strand.to_string();
                negative.push(NEGATIVE {
                    id: mutablename,
                    seq: mutablestring,
                    strand: mutablestrand,
                });
            }
        }
    }

    let mut mrna_length: Vec<usize> = Vec::new();
    let mut cds_length: Vec<usize> = Vec::new();
    let mut cds_length_positive: Vec<usize> = Vec::new();
    let mut cds_length_negative: Vec<usize> = Vec::new();

    for i in mRNAstruct.iter_mut() {
        let lengthmut = i.seq.len();
        mrna_length.push(lengthmut);
    }

    for i in cdsstruct.iter_mut() {
        let lengthmut = i.seq.len();
        cds_length.push(lengthmut);
    }

    for i in positive.iter_mut() {
        let lengthmut = i.seq.len();
        cds_length_positive.push(lengthmut);
    }

    for i in negative.iter_mut() {
        let lengthmut = i.seq.len();
        cds_length_negative.push(lengthmut);
    }

    let mut mrna_file = File::create("mRNA.fasta").expect("file not present");
    for i in mRNAstruct.iter_mut() {
        writeln!(mrna_file, ">{}\n{}", i.id, i.seq);
    }

    let mut cds_file = File::create("cds.fasta").expect("file not present");
    for i in cdsstruct.iter_mut() {
        writeln!(cds_file, ">{}\n{}", i.id, i.seq);
    }

    let mut cds_positive = File::create("cds-positive.fasta").expect("file not present");
    for i in positive.iter_mut() {
        writeln!(cds_positive, ">{}\n{}", i.id, i.seq);
    }

    let mut cds_negative = File::create("cds-negative.fasta").expect("file not present");
    for i in negative.iter_mut() {
        writeln!(cds_negative, ">{}\n{}", i.id, i.seq);
    }
    Ok("The panarc has been completed".to_string())
}
