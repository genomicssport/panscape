use crate::filesplitpattern::fastareturn;
use crate::nanoporepacbio::Search;
use crate::nanoporepacbio::Sequence;
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn motifsearch(path: &str, motif: &str) -> Result<String, Box<dyn Error>> {
    let unpack: Vec<Sequence> = fastareturn(path).unwrap();

    let mut sequencesearch: Vec<Search> = Vec::new();
    for i in unpack.iter() {
        sequencesearch.push(Search {
            name: i.header.clone(),
            sequence: i.sequence.clone(),
            start: i.sequence.find(motif).unwrap(),
            end: i.sequence.find(motif).unwrap() + motif.len(),
        });
    }

    let mut filewrite = File::create("motipattern.txt").expect("file not present");
    for i in sequencesearch.iter() {
        writeln!(filewrite, "{:?}\t{:?}\t{:?}", i.name, i.start, i.end).expect("file not found");
    }

    Ok("The results have been written".to_string())
}
