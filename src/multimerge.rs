use crate::nanoporepacbio::BedtoolsRangeMulti;
use crate::nanoporepacbio::Fasta;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

/*
 Author Gaurav Sablok
 SLB Potsdam
 Date 2025-2-26

 bedtools arthimetic for the pangenome. rust ancestral state construction using the bed tools alignments.given a bed alignment and the corresponding fasta file will extract and filter the pangenome alignment above that threshold. You can map using any aligner and convert to the bed format for the same.

 This allows for the same alignment of the sequence across a multitude of genomes and then extracting the longest of the alignment for each of the alignment and making the ancestral tags.

 algorithmic implementation says: make a shahset of the unique alignments and then using those alignments then search the value for the alignment in the hashset and if the value matches then store the value into the tuple and before inseting into the final tuple, sort the tuple and take the first vector of the tuple.

*/

pub fn pangenomemultialignment(
    pathalignment: &str,
    pathfasta: &str,
    pathprank: &str,
) -> Result<String, Box<dyn Error>> {
    let bedtools_open = File::open(pathalignment).expect("file not present");
    let bedtools_read = BufReader::new(bedtools_open);
    let mut bedtoolsend: Vec<String> = Vec::new();
    let mut uniquealignment: HashSet<String> = HashSet::new();
    for i in bedtools_read.lines() {
        let line = i.expect("line not present");
        let linevec: Vec<_> = line
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        uniquealignment.insert(linevec[0].to_string());
        bedtoolsend.push(line);
    }

    let mut uniquealignment: HashSet<String> = HashSet::new();
    let mut filestring: Vec<String> = Vec::new();
    for i in bedtoolsend.iter() {
        let line = i;
        let linevec: Vec<_> = line
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        uniquealignment.insert(linevec[0].to_string());
        filestring.push(line.to_string());
    }
    let mut final_tuple_vec: Vec<(String, usize, usize, usize)> = Vec::new();
    for j in filestring.iter() {
        for i in uniquealignment.iter() {
            let line = j;
            let linevec: Vec<_> = line
                .split(" ")
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            if linevec[0] == i {
                let tupleiter: (String, usize, usize, usize) = (
                    linevec[0].to_string(),
                    linevec[1].parse::<usize>().unwrap(),
                    linevec[2].parse::<usize>().unwrap(),
                    linevec[2].parse::<usize>().unwrap() - linevec[1].parse::<usize>().unwrap(),
                );
                final_tuple_vec.push(tupleiter);
            }
        }
    }
    let mut last_tuple_hold: Vec<_> = Vec::new();
    for i in uniquealignment.iter() {
        let mut submatch: Vec<_> = Vec::new();
        for j in final_tuple_vec.iter_mut() {
            if *i == j.0 {
                submatch.push(j);
            }
        }
        submatch.sort_by(|a, b| b.3.cmp(&a.3));
        last_tuple_hold.push(submatch[0].clone());
    }

    let mut writeable_longest: Vec<BedtoolsRangeMulti> = Vec::new();

    for i in last_tuple_hold.iter() {
        writeable_longest.push(BedtoolsRangeMulti {
            alignedref: i.0.clone(),
            alignedstart: i.1,
            alignedend: i.2,
            difference: i.3,
        })
    }

    let fastahold: Vec<Fasta> = fasta_estimate(pathfasta).unwrap();
    let mut fastawrite = File::create("ancestral.fasta").expect("file not found");
    for i in writeable_longest.iter() {
        for j in fastahold.iter() {
            if i.alignedref == j.header {
                write!(
                    fastawrite,
                    ">{}\n{}\n",
                    i.alignedref,
                    &j.sequence[i.alignedstart..i.alignedend]
                )
                .expect("file not found");
            }
        }
    }

    let _aligned = Command::new(pathprank)
        .arg("-d=ancestral.fasta")
        .arg("-o=ancestral-aligned.fasta")
        .arg("-showanc")
        .output()
        .expect("failed to execute");

    Ok("Results file has been written".to_string())
}

pub fn fasta_estimate(path: &str) -> Result<Vec<Fasta>, Box<dyn Error>> {
    let fastaopen = File::open(path).expect("file not present");
    let fastaread = BufReader::new(fastaopen);
    let mut fastaholder: Vec<Fasta> = Vec::new();
    let mut fastaheader: Vec<String> = Vec::new();
    let mut fastasequence: Vec<String> = Vec::new();
    for i in fastaread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            fastaheader.push(line.replace(">", ""));
        } else {
            fastasequence.push(line);
        }
    }

    for i in 0..fastaheader.len() {
        fastaholder.push(Fasta {
            header: fastaheader[i].clone(),
            sequence: fastasequence[i].clone(),
        })
    }

    Ok(fastaholder)
}
