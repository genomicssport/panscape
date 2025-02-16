use crate::filesplitpattern::fastareturn;
use crate::nanoporepacbio::Sequence;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

pub fn multiclipseqa(path: &str, clipseq: &str) -> Result<String, Box<dyn Error>> {
    let sequenceunpack: Vec<Sequence> = fastareturn(path).unwrap();

    let fileopen = File::open(clipseq).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut clipregion: Vec<String> = Vec::new();

    for i in fileread.lines() {
        let line = i.expect("file not present");
        clipregion.push(line.to_string());
    }

    let mut clipseqa: Vec<Sequence> = Vec::new();
    for i in sequenceunpack.iter() {
        for j in 0..clipregion.len() {
            let seqlinestart = i.sequence.find(clipregion[j].as_str()).unwrap();
            let seqlineend =
                i.sequence.to_string().find(clipregion[j].as_str()).unwrap() + clipregion[j].len();
            let clippedregion = i.sequence[seqlinestart..seqlineend].to_string();
            let nextclippedregion_start = clippedregion.find(clipregion[j + 1].as_str()).unwrap();
            let nextclippedreg_end =
                clippedregion.find(clipregion[j + 1].as_str()).unwrap() + clipregion[j + 1].len();
            let finalnextclipped =
                clippedregion[nextclippedregion_start..nextclippedreg_end].to_string();
            let finalclippedstart = finalnextclipped.find(clipregion[j + 2].as_str()).unwrap();
            let finalclippedend = finalnextclipped.find(clipregion[j + 2].as_str()).unwrap()
                + clipregion[j + 2].len();
            let finalclip = finalnextclipped[finalclippedstart..finalclippedend].to_string();
            let writeclipstart = finalclip.find(clipregion[j + 3].as_str()).unwrap();
            let writeclipend =
                finalclip.find(clipregion[j + 3].as_str()).unwrap() + clipregion[j + 3].len();
            let writeseq = finalclip[writeclipstart..writeclipend].to_string();
            if clippedregion.len() == 0usize {
                continue;
            } else if !clippedregion.len() == 0usize {
                clipseqa.push(Sequence {
                    header: i.header.clone(),
                    sequence: writeseq,
                });
            }
        }
    }

    let mut filewrite = File::create("clippedseq-file.fasta").expect("file not found");
    for i in clipseqa.iter() {
        writeln!(filewrite, ">{}\n{}", i.header, i.sequence).expect("file not present");
    }

    Ok("The clipseq has been finished".to_string())
}
