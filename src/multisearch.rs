use crate::filesplitpattern::fastareturn;
use crate::nanoporepacbio::Multiplepattern;
use crate::nanoporepacbio::Sequence;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::Write;

/*
 Author Gaurav Sablok
 SLB Potsdam
 Date: 2025-2-19

*/

pub fn multisearchregex(path: &str, searchstr: &str) -> Result<String, Box<dyn Error>> {
    let sequencevec: Vec<Sequence> = fastareturn(path).unwrap();

    let searchregex = Regex::new(searchstr).unwrap();
    let mut collectionvec: Vec<Multiplepattern> = Vec::new();

    for i in sequencevec.iter() {
        let mut vecor: Vec<(usize, usize)> = Vec::new();
        for j in searchregex.captures_iter(i.sequence.as_str()) {
            let tuplecord: (usize, usize) = (j.get(0).unwrap().start(), j.get(0).unwrap().end());
            vecor.push(tuplecord);
        }

        collectionvec.push(Multiplepattern {
            name: &i.header,
            collectionvec: vecor,
        });
    }

    let mut filewrite = File::create("multi-searchpattern.txt").expect("file not found");

    for i in collectionvec.iter() {
        for j in i.collectionvec.iter() {
            writeln!(filewrite, "{}\t{}\t{}", i.name, j.0, j.1).expect("line not found");
        }
    }

    Ok("The search results for the multisearch have been written".to_string())
}
