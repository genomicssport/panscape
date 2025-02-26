use crate::nanoporepacbio::Fasta;
use crate::nanoporepacbio::MergeBed;
use crate::nanoporepacbio::MergeFasta;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*

 Author Gaurav Sablok
 SLB Potsdam
 Date 2025-2-26

bedtools single merge for the pangenome bed files. A tuple based approach to anchor the first iter and then look for the next iter and then use the comparison to anchor the merge.

*/

pub fn pangenome_merge(path1: &str, path2: &str) -> Result<String, Box<dyn Error>> {
    let bedfile = File::open(path1).expect("File not found");
    let bedreader = BufReader::new(bedfile);
    let mut tuple_vec: Vec<(String, usize, usize, String)> = Vec::new();
    for i in bedreader.lines() {
        let line = i.expect("Could not read line");
        let lineread = line
            .split("\t")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        let tuple_iter: (String, usize, usize, String) = (
            lineread[0].to_string(),
            lineread[1].parse::<usize>().unwrap(),
            lineread[2].parse::<usize>().unwrap(),
            lineread[3].to_string(),
        );
        tuple_vec.push(tuple_iter);
    }

    let mut tuple_merge: Vec<MergeBed> = Vec::new();
    for i in 0..tuple_vec.len() - 1 {
        if tuple_vec[i + 1].1 > tuple_vec[i].1
            && tuple_vec[i + 1].1 < tuple_vec[i].2
            && tuple_vec[i + 1].2 > tuple_vec[i].2
        {
            let mut tuplevechold: Vec<String> = vec![];
            tuplevechold.push(tuple_vec[i].3.to_string());
            tuplevechold.push(tuple_vec[i + 1].3.to_string());
            tuple_merge.push(MergeBed {
                bed1: tuple_vec[i].0.clone(),
                start1: tuple_vec[i].1,
                end1: tuple_vec[i + 1].2,
                bedseq: tuplevechold,
            });
        } else if tuple_vec[i].1 < tuple_vec[i + 1].1
            && tuple_vec[i].2 < tuple_vec[i + 1].1
            && tuple_vec[i].2 < tuple_vec[i + 1].2
        {
            let mut tuplevechold: Vec<String> = vec![];
            tuplevechold.push(tuple_vec[i].3.to_string());
            tuple_merge.push(MergeBed {
                bed1: tuple_vec[i + 1].0.clone(),
                start1: tuple_vec[i + 1].1,
                end1: tuple_vec[i + 1].2,
                bedseq: tuplevechold,
            });
        }
    }

    let fastaunclone: Vec<Fasta> = fasta_estimate(path2)?;

    let mut tuple_unmerge = File::create("merged_cordinates.txt").expect("file not found");

    let mut tuple_final_write: Vec<MergeFasta> = Vec::new();

    for i in fastaunclone.iter() {
        for j in tuple_merge.iter() {
            if i.header == j.bed1 {
                tuple_final_write.push(MergeFasta {
                    bed: i.header.clone(),
                    start1: j.start1,
                    end1: j.end1,
                    mergedseq: i.sequence[j.start1..j.end1].to_string(),
                })
            }
        }
    }

    for i in tuple_final_write.iter() {
        writeln!(
            tuple_unmerge,
            "{}\t{}\t{}\t{}",
            i.bed, i.start1, i.end1, i.mergedseq
        )
        .expect("line not present");
    }

    Ok("pangenome merge has been written".to_string())
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
