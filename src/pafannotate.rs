use crate::nanoporepacbio::{PAFAnnotate, Pafanalyzer};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
 Author Gaurav Sablok
 SLB Potsdam
 Date: 2025-2-19

*/

pub fn annotatepaf(pathpaf: &str, pathgtf: &str) -> Result<String, Box<dyn Error>> {
    Ok("The results have been written".to_string())
}
