use crate::nanoporepacbio::Alignment;
use cmd_lib::run_cmd;
use cmd_lib::run_fun;
use std::collections::HashSet;
use std::env::set_current_dir;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
/*
  Author Gaurav Sablok
  SLB Potsdam
  Date: 2025-2-17

*/

pub fn pangenome_hifiasm<'a>(
    path: &'a str,
    thread: &'a i32,
    proteinfasta: &'a str,
) -> Result<String, Box<dyn Error>> {
    let _ = fs::create_dir("pangenome_assemble");
    let assemblerpath = Path::new("./pangenome_assemble");
    set_current_dir(assemblerpath).expect("dir not found");
    let _newwrite = std::process::Command::new("git clone https://github.com/chhylp123/hifiasm");
    let _makecmd = std::process::Command::new("cd hifiasm && make");
    let reads = &path.to_string();
    let _genomeassembly = std::process::Command::new("./hifiasm")
        .arg("-o genome")
        .arg("-t")
        .arg(thread.to_string())
        .arg("-f0")
        .arg(&reads)
        .arg("2>")
        .arg("genome-assembly.log")
        .spawn()
        .expect("hifiasm failed to run the assembly");

    let _genomeassembly = run_fun!(
    bash -c "mv genome.bp.p_ctg.gfa assembled-genome.gfa"
     | awk r#"awk '/^S/{print ">"$2;print $3}' assembled-genome.gfa > assembled-genome.fasta"#
     | awk r#"'/^>/ {printf("\n%s\n",$0);next; } { printf("%s",$0);} END {printf("\n");}'
                                          assembled-genome.fasta > final-genome-assembled.fasta"#
    )
    .unwrap();
    println!(
        r#"genome assembly linear fasta failed and the assembled genome
                                     in the graph format is present in the assembled-genome.gfa"#
    );

    let _ = fs::create_dir("./genome_completeness");
    let path_genome = Path::new("./pangenome/genome_completeness");
    let final_assembly =
        String::from("./pangenome/genome_completeness/final-genome-assembled.fasta");
    set_current_dir(path_genome).expect("directory not found");
    let _compleasm = run_fun!(bash -c "wget https://github.com/huangnengCSU/compleasm/releases/download/v0.2.6/compleasm-0.2.6_x64-linux.tar.bz2"
       | bash -c "tar -jxvf compleasm-0.2.6_x64-linux.tar.bz2)"
    );
    set_current_dir("./pangenome/genome_completeness/compleasm").expect("directory not found");
    let _compleasum_run = run_cmd!(
          bash -c "cp -r ../../final-genome-assembled.fasta ./"
    );
    let _genome_assessment = std::process::Command::new("python3")
        .arg("compleasm.py")
        .arg("run")
        .arg("-t")
        .arg("10")
        .arg("-l")
        .arg("eukaryota")
        .arg("-a")
        .arg(&final_assembly)
        .arg("-o")
        .arg("final-genome-completness")
        .spawn()
        .expect("completeness failed");

    let _ = fs::create_dir_all("./pangenome/genome_annotations");
    let annotation_path = Path::new("./pangenome/genome_annotations");
    let _ = set_current_dir(annotation_path);
    run_cmd!(
        bash -c "git clone https://github.com/lh3/miniprot"
        | cd miniprot | make
        | mv miniprot ../
        | echo "miniprot has been installed"
    )
    .unwrap();
    let final_assembly =
        String::from("./pangenome/genome_completeness/final-genome-assembled.fasta");
    let _protein_annotations = std::process::Command::new("miniprot")
        .arg("-t")
        .arg("10")
        .arg("--gff")
        .arg(&final_assembly)
        .arg(&proteinfasta)
        .arg(">")
        .arg("final-genome-annotations.gff")
        .spawn()
        .ok()
        .expect("genome annotations failed");

    let final_assembly =
        String::from("./pangenome/genome_completeness/final-genome-annotations.gff");
    let file = File::open(&final_assembly).expect("file not found");
    let reader = BufReader::new(&file);
    let mut mrna_alignment_vec: Vec<Alignment> = Vec::new();
    for line in reader.lines() {
        let genomeiter = line.expect("line not present").clone();
        if genomeiter.starts_with("#") {
            continue;
        }
        let genomesec = genomeiter.split(" ").collect::<Vec<&str>>();
        if genomesec[2] == "mRNA" {
            mrna_alignment_vec.push(Alignment {
                alignmentregion: genomesec[0].to_string(),
                start: genomesec[3].parse::<usize>().unwrap(),
                end: genomesec[4].parse::<usize>().unwrap(),
            })
        }
    }
    let final_assembly = String::from("./pangenome/genome_completeness/final-genome-assembled.gff");
    let parentid: HashSet<&str> = HashSet::new();
    let file = File::open(&final_assembly).expect("file not present");
    let fileread = BufReader::new(&file);
    for line in fileread.lines() {
        let genomesec = line.expect("file not present");
        let genomepart = genomesec.split(" ").collect::<Vec<&str>>()[2];
        if genomepart == "mRNA" {
            let idhold = &genomesec.split(" ").collect::<Vec<&str>>()[8];
            let finalid = idhold.split(";").collect::<Vec<&str>>()[0];
            let appenid = finalid.split("=").collect::<Vec<&str>>()[1];
            let mut insertid = parentid.clone();
            insertid.insert(appenid);
        }
    }
    Ok("The pangenome along with the genome alignment have been done".to_string())
}
