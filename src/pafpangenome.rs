use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::nanoporepacbio::Pafanalyzer;

pub fn pangenome_summarize(path: &str) -> Result<String, Box<dyn Error>> {

    let pafopen = File::open(path).expect("file not found");
    let pafread = BufReader::new(pafopen);
    let pafstruct: V
    
    Ok("The pangenome has been summarized".to_string())
}
