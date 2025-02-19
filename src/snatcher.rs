use crate::nanoporepacbio::{QuerypafView, ReferencepafView};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
 Author Gaurav Sablok
 SLB Potsdam
 Date: 2025-2-19
reading the pafalignments and then extracting the aligned paf regions from the reference as well as the query genome.
*/

pub fn snatcherextract(
    pathpaf: &str,
    queryfasta: &str,
    referencefasta: &str,
) -> Result<String, Box<dyn Error>> {
    let pafaligner = File::open(pathpaf).expect("file not present");
    let pafreader = BufReader::new(pafaligner);
    let mut pafquery: Vec<QuerypafView> = Vec::new();
    let mut pafreference: Vec<ReferencepafView> = Vec::new();
    for i in pafreader.lines() {
        let line = i.expect("line not present");
        let linecheck = line.split("\t").collect::<Vec<_>>();
        pafquery.push(QuerypafView {
            query: linecheck[0].to_string(),
            length: linecheck[1].parse::<usize>().unwrap(),
            start: linecheck[2].parse::<usize>().unwrap(),
            end: linecheck[3].parse::<usize>().unwrap(),
            strand: linecheck[4].to_string(),
        });
        pafreference.push(ReferencepafView {
            reference: linecheck[5].to_string(),
            length: linecheck[6].parse::<usize>().unwrap(),
            start: linecheck[7].parse::<usize>().unwrap(),
            end: linecheck[8].parse::<usize>().unwrap(),
            residuematch: linecheck[9].parse::<usize>().unwrap(),
            alignmentblock: linecheck[10].parse::<usize>().unwrap(),
        })
    }

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    struct FastaEncoder {
        header: String,
        sequence: String,
    }

    let query_fasta = File::open(queryfasta).expect("file not present");
    let reference_fasta = File::open(referencefasta).expect("file not present");
    let mut query_combined: Vec<FastaEncoder> = Vec::new();
    let mut reference_combined: Vec<FastaEncoder> = Vec::new();

    let mut query_header = Vec::new();
    let mut query_sequence = Vec::new();
    let mut reference_header = Vec::new();
    let mut reference_sequence = Vec::new();

    let queryread = BufReader::new(query_fasta);
    for i in queryread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            query_header.push(line.replace(">", ""))
        } else {
            query_sequence.push(line)
        }
    }

    let referenceread = BufReader::new(reference_fasta);
    for i in referenceread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            reference_header.push(line.replace(">", ""))
        } else {
            reference_sequence.push(line)
        }
    }

    for i in 0..query_header.len() {
        query_combined.push(FastaEncoder {
            header: query_header[i].clone(),
            sequence: query_sequence[i].clone(),
        })
    }

    for i in 0..reference_header.len() {
        reference_combined.push(FastaEncoder {
            header: reference_header[i].clone(),
            sequence: reference_sequence[i].clone(),
        })
    }

    let mut splice_query: Vec<FastaEncoder> = Vec::new();
    let mut splice_reference: Vec<FastaEncoder> = Vec::new();

    for i in pafquery.iter() {
        for j in query_combined.iter() {
            if i.query.to_string() == j.header.to_string() {
                splice_query.push(FastaEncoder {
                    header: i.query.clone(),
                    sequence: j.sequence[i.start..i.end].to_string(),
                })
            }
        }
    }

    for r1 in pafreference.iter() {
        for r2 in reference_combined.iter() {
            if r1.reference.to_string() == r2.header.to_string() {
                splice_reference.push(FastaEncoder {
                    header: r2.header.to_string(),
                    sequence: r2.sequence[r1.start..r1.end].to_string(),
                })
            }
        }
    }

    let mut query_write = File::create("query-aligned.fasta").expect("file not present");
    let mut reference_write = File::create("reference-aligned.fasta").expect("file not present");
    for i in splice_query.iter() {
        write!(query_write, ">{}\n{}\n", i.header, i.sequence).expect("query not found");
    }

    for i in splice_reference.iter() {
        write!(reference_write, ">{}\n{}\n", i.header, i.sequence).expect("line not present");
    }

    Ok(
        "The paf analyzer has finished and all the pangenome fasta files have been written:"
            .to_string(),
    )
}
