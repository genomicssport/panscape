use crate::nanoporepacbio::BedtoolsRange;
use crate::nanoporepacbio::Fasta;
use crate::nanoporepacbio::Vecstorage;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

/*

 Author Gaurav Sablok
 SLB Potsdam
 Date 2025-2-26

 bedtools arthimetic for the pangenome. rust ancestral state construction using the bed tools alignments.given a bed alignment and the corresponding fasta file will extract and filter the pangenome alignment above that threshold. You can map using any aligner and convert to the bed format for the same.

*/

pub fn pangenome_longest_alignment(
    pathalignment: &str,
    pathfasta: &str,
    length: usize,
    pathprank: &str,
) -> Result<String, Box<dyn Error>> {
    let bedtools_open = File::open(pathalignment).expect("file not present");
    let bedtools_read = BufReader::new(bedtools_open);
    let mut bedtoolshold: Vec<BedtoolsRange> = Vec::new();
    let mut bedtoolsstart: Vec<usize> = Vec::new();
    let mut bedtoolsend: Vec<usize> = Vec::new();
    for i in bedtools_read.lines() {
        let line = i.expect("line not present");
        let linevec: Vec<_> = line
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        bedtoolshold.push(BedtoolsRange {
            alignedref: linevec[0].to_string(),
            alignedstart: linevec[1].parse::<usize>().unwrap(),
            alignedend: linevec[2].parse::<usize>().unwrap(),
            score: linevec[4].parse::<usize>().unwrap(),
            strand: linevec[5].to_string(),
            difference: linevec[2].parse::<usize>().unwrap() - linevec[1].parse::<usize>().unwrap(),
        });
        bedtoolsstart.push(linevec[1].parse::<usize>().unwrap());
        bedtoolsend.push(linevec[2].parse::<usize>().unwrap());
    }
    // calling the longest alignment function here.
    let estimate = estimate_longest_alignment(bedtoolsstart, bedtoolsend).unwrap();
    let mut writeestimate = File::create("alignment_length.txt").expect("file not found");
    writeln!(
        writeestimate,
        "The length estimates prior to the filtering along with the differences are given below"
    )
    .expect("line not found");
    writeln!(writeestimate, "start\tend\tdifference\t").expect("line not found");
    for i in estimate.iter() {
        writeln!(writeestimate, "{}\t{}\t{}\n", i.start, i.end, i.difference)
            .expect("file not found");
    }

    let fastahold: Vec<Fasta> = fasta_estimate(pathfasta).unwrap();
    let mut fastawrite = File::create("ancestral.fasta").expect("file not found");
    for i in bedtoolshold.iter() {
        for j in fastahold.iter() {
            if i.alignedref == j.header && i.difference >= length {
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

pub fn estimate_longest_alignment(
    inputvecstart: Vec<usize>,
    inputvecend: Vec<usize>,
) -> Result<Vec<Vecstorage>, Box<dyn Error>> {
    let vec_analyze_start = inputvecstart;
    let vec_analyze_end = inputvecend;
    let mut vecdiff: Vec<Vecstorage> = Vec::new();

    for i in 0..vec_analyze_start.len() {
        vecdiff.push(Vecstorage {
            start: vec_analyze_start[i],
            end: vec_analyze_end[i],
            difference: vec_analyze_end[i] - vec_analyze_start[i],
        })
    }

    Ok(vecdiff)
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
