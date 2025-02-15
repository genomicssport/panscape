use crate::filesplitpattern::fastareturn;
use crate::nanoporepacbio::Sequence;
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn clipseqa(path: &str, clipseq: &str) -> Result<String, Box<dyn Error>> {
    let sequenceunpack: Vec<Sequence> = fastareturn(path).unwrap();

    let mut clipseqa: Vec<Sequence> = Vec::new();
    for i in sequenceunpack.iter() {
        let seqlinestart = i.sequence.find(clipseq).unwrap();
        let seqlineend = i.sequence.find(clipseq).unwrap() + clipseq.len();
        let clippedregion = i.sequence[seqlinestart..seqlineend].to_string();
        if clippedregion.len() == 0usize {
            continue;
        } else if !clippedregion.len() == 0usize {
            clipseqa.push(Sequence {
                header: i.header.clone(),
                sequence: clippedregion,
            });
        }
    }

    let mut filewrite = File::create("clippedseq-file.fasta").expect("file not found");
    for i in clipseqa.iter() {
        writeln!(filewrite, ">{}\n{}", i.header, i.sequence).expect("file not present");
    }

    Ok("The clipseq has been finished".to_string())
}
