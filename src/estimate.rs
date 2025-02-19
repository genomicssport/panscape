use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn harmonicestimate(harmonicfile: &str) -> Result<String, Box<dyn Error>> {
    /*
         Author Gaurav Sablok
         SLB Potsdam
         Date: 2025-2-19
        estimate the harmonic mean of the same query aligned across the different genomes with less divergence so that the estimated divergence value can be linked wot the harmonic mean of the pangenome covered.

    */
    let fileopen1 = File::open(harmonicfile).expect("file not present");
    let fileread1 = BufReader::new(fileopen1);
    let mut harmonic_capture_final: Vec<usize> = Vec::new();
    for i in fileread1.lines() {
        let line = i.expect("line not present");
        let linevector = line
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut harmonic_capture_inverse: Vec<usize> = Vec::new();
        for i in 0..linevector.len() {
            harmonic_capture_inverse.push(1 / linevector[i]);
        }
        let finalharmonic: usize = linevector.len() / sumharmonic(linevector).unwrap();
        harmonic_capture_final.push(finalharmonic);
    }

    let mut harmonic_file = File::create("harmonic-analysis.txt").expect("file not present");
    for i in 0..harmonic_capture_final.len() {
        write!(harmonic_file, "{}\n", harmonic_capture_final[i]).expect("line not present");
    }

    Ok("capture harmonic mean for each same query coordinate have ben writen".to_string())
}

pub fn sumharmonic(inputvec: Vec<usize>) -> Result<usize, Box<dyn Error>> {
    let readarr = inputvec;
    let mut harmonicholder = 0usize;
    for i in readarr.iter() {
        harmonicholder += *i;
    }
    Ok(harmonicholder)
}
