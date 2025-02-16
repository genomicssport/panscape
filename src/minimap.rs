use crate::filesplitpattern::fastareturn;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

pub fn minimapalignment(
    pathreads: &str,
    pathminimap: &str,
    pathproteins: &str,
    thread: &str
) -> Result<String, Box<dyn Error>> {

    let sequencevector =    fastareturn(pathreads).unwrap();
    let proteinfileopen = File::open(pathproteins).expect("file not present");
    let proteinfileread = BufReader::new(proteinfileopen);
    let mut proteinheader:Vec<String> = Vec::new();
    let mut proteinsequence: Vec<String> = Vec::new();

    for i in proteinfileread.lines(){
        let line = i.expect("line not found");
        if line.starts_with(">"){
            proteinheader.push(line.replace(">", ""));
        } else if !line.starts_with(">"){
            proteinsequence.push(line)
        }
    }

    let command = Command::new  ("minimap").arg("-t").arg(thread).arg(pathreads.to_string()).arg(pathproteins).arg(">").arg("proteinaligned.fasta").output().expect("command failed");
    
    Ok("The results have been sent to the files: {:?}".to_string())
}
