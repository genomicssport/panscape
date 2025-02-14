use crate::filesplitpattern::fastareturn;
use crate::nanoporepacbio::Motifcatcher;
use crate::nanoporepacbio::Sequence;
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn motifcatcherupdown(
    path: &str,
    motif: &str,
    upstream: &str,
    downstream: &str,
) -> Result<String, Box<dyn Error>> {
    let unpack: Vec<Sequence> = fastareturn(path).unwrap();
    let upstreamadd: usize = upstream.parse::<usize>().unwrap();
    let downstreamadd: usize = downstream.parse::<usize>().unwrap();

    let mut sequencesearch: Vec<Motifcatcher> = Vec::new();
    for i in unpack.iter() {
        sequencesearch.push(Motifcatcher {
            name: i.header.clone(),
            sequence: i.sequence.clone(),
            start: i.sequence.find(motif).unwrap(),
            end: i.sequence.find(motif).unwrap() + motif.len(),
            upstream: i.sequence
                [i.sequence.find(motif).unwrap() - upstreamadd..i.sequence.find(motif).unwrap()]
                .to_string(),
            downstream: i.sequence[i.sequence.find(motif).unwrap() + motif.len()..downstreamadd]
                .to_string(),
        });
    }

    let mut filewrite = File::create("motipattern-up-downstream.txt").expect("file not present");
    for i in sequencesearch.iter() {
        writeln!(
            filewrite,
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
            i.name, i.start, i.end, i.upstream, i.downstream
        )
        .expect("file not found");
    }

    Ok("The results have been written".to_string())
}
