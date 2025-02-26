use crate::nanoporepacbio::Common;
use crate::nanoporepacbio::Commonsnatcher;
use crate::nanoporepacbio::Endpointcompare;
use crate::nanoporepacbio::Fasta;
use crate::nanoporepacbio::Fastasnatcher;
use crate::nanoporepacbio::Startpointcompare;
use crate::nanoporepacbio::VCFRange;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*

 Author Gaurav Sablok
 SLB Potsdam
 Date 2025-2-26

pangenomeic vcf compare. Compare two vcf files for the pangenome alignment and profile all the snp diversity for the pangenome linearization. storing the vcf as a btreemap so that it can by searched through the mapping key index and this allows for the faster sorting of the keymaps based on the string.
algorithmic implementation: to minimize the time comparsion, it first looks for the start or the end point and then tabulates the vcf snps so the difference of the length doesnt have to be calculated in real time. If there is a difference in the start or the end coordinate then will stash the stash coordinate, do a reverse slice and then will map the reverse slice iteration into a join vector. Even reports a single nucleotide
based difference.

*/

pub fn vcf_compare(
    path1: &str,
    path2: &str,
    pathfastaunwrap: &str,
) -> Result<String, Box<dyn Error>> {
    let vcf1_open = File::open(path1).expect("file not found");
    let vcf2_open = File::open(path2).expect("file not found");
    let vcf1_read = BufReader::new(vcf1_open);
    let vcf2_read = BufReader::new(vcf2_open);
    let mut vcf1: Vec<VCFRange> = Vec::new();
    let mut vcf2: Vec<VCFRange> = Vec::new();

    for i in vcf1_read.lines() {
        let line = i.expect("line not present");
        if line.starts_with("#") {
            continue;
        } else if !line.starts_with("#") {
            let linevec = line
                .split("\t")
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            vcf1.push(VCFRange {
                name: linevec[0].to_string(),
                start: linevec[1].parse::<usize>().unwrap(),
                end: linevec[2].parse::<usize>().unwrap(),
                delorig: linevec[3].to_string(),
                deltype: linevec[4].to_string().replace(">", "").replace("<", ""),
                threshold: Box::new(linevec[5].parse::<f64>().unwrap()),
            });
        }
    }

    for i in vcf2_read.lines() {
        let line = i.expect("line not present");
        if line.starts_with("#") {
            continue;
        } else if !line.starts_with("#") {
            let linevec = line
                .split("\t")
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            vcf2.push(VCFRange {
                name: linevec[0].to_string(),
                start: linevec[1].parse::<usize>().unwrap(),
                end: linevec[2].parse::<usize>().unwrap(),
                delorig: linevec[3].to_string(),
                deltype: linevec[4].to_string().replace(">", "").replace("<", ""),
                threshold: Box::new(linevec[5].parse::<f64>().unwrap()),
            });
        }
    }

    let mut end_point_compare: Vec<Endpointcompare> = Vec::new();

    for j in vcf1.iter() {
        for value in vcf2.iter() {
            if j.end == value.end && j.start > value.start {
                end_point_compare.push(Endpointcompare {
                    name: j.name.clone(),
                    start1: j.start,
                    end1: j.end,
                    start2: value.start,
                    end2: value.end,
                    delorig1: j.delorig.clone(),
                    deltype1: j.deltype.clone(),
                    threshold1: Box::new(*j.threshold),
                    delorig2: value.delorig.clone(),
                    deltype2: value.deltype.clone(),
                    threshold2: Box::new(*value.threshold),
                    difference: Box::new(j.start - value.start),
                });
            } else if j.end == value.end && value.start > j.start {
                end_point_compare.push(Endpointcompare {
                    name: j.name.clone(),
                    start1: j.start,
                    end1: j.end,
                    start2: value.start,
                    end2: value.end,
                    delorig1: j.delorig.clone(),
                    deltype1: j.deltype.clone(),
                    threshold1: Box::new(*j.threshold),
                    delorig2: value.delorig.clone(),
                    deltype2: value.deltype.clone(),
                    threshold2: Box::new(*value.threshold),
                    difference: Box::new(value.start - j.start),
                })
            }
        }
    }

    let mut start_point_compare: Vec<Startpointcompare> = Vec::new();

    for j in vcf1.iter() {
        for value in vcf2.iter() {
            if j.start == value.start && j.end > value.end {
                start_point_compare.push(Startpointcompare {
                    name: j.name.clone(),
                    start1: j.start,
                    start2: value.start,
                    end1: j.end,
                    end2: value.end,
                    delorig1: j.delorig.clone(),
                    deltype1: j.deltype.clone(),
                    threshold1: Box::new(*j.threshold),
                    delorig2: value.delorig.clone(),
                    deltype2: value.deltype.clone(),
                    threshold2: Box::new(*value.threshold),
                    difference: Box::new(j.end - value.end),
                });
            } else if j.start == value.start && value.end > j.end {
                start_point_compare.push(Startpointcompare {
                    name: j.name.clone(),
                    start1: j.start,
                    start2: value.start,
                    end1: j.end,
                    end2: value.end,
                    delorig1: j.delorig.clone(),
                    deltype1: j.deltype.clone(),
                    threshold1: Box::new(*j.threshold),
                    delorig2: value.delorig.clone(),
                    deltype2: value.deltype.clone(),
                    threshold2: Box::new(*value.threshold),
                    difference: Box::new(value.end - j.end),
                })
            }
        }
    }

    let mut common_pangenome_variants: Vec<Common> = Vec::new();

    for j in vcf1.iter() {
        for value in vcf2.iter() {
            if j.start == value.start && j.end == value.end {
                common_pangenome_variants.push(Common {
                    name: j.name.clone(),
                    start1: j.start,
                    start2: value.start,
                    end1: j.end,
                    end2: value.end,
                    delorig1: j.delorig.clone(),
                    deltype1: j.deltype.clone(),
                    delorig2: value.delorig.clone(),
                    deltype2: value.deltype.clone(),
                    threshold1: *j.threshold,
                    threshold2: *value.threshold,
                })
            }
        }
    }

    let fasta_unload: Vec<Fasta> = fasta_estimate(pathfastaunwrap).unwrap();

    let mut endpointsnatcher: Vec<Fastasnatcher> = Vec::new();
    let mut startpointsnatcher: Vec<Fastasnatcher> = Vec::new();
    let mut commonpointvariant: Vec<Commonsnatcher> = Vec::new();

    let mut endpointcompare = File::create("endpoint_variants.txt").expect("file not present");
    let mut startpointcompare = File::create("startpoint_variants.txt").expect("file not present");
    let mut common_point = File::create("common_variant.txt").expect("file not present");

    for i in common_pangenome_variants.iter() {
        for j in fasta_unload.iter() {
            if i.name == j.header {
                commonpointvariant.push(Commonsnatcher {
                    name: i.name.clone(),
                    start1: i.start1,
                    end1: i.end1,
                    start2: i.start2,
                    end2: i.end2,
                    deltype1: i.deltype1.clone(),
                    delorig1: i.delorig1.clone(),
                    deltype2: i.deltype2.clone(),
                    delorig2: i.delorig2.clone(),
                    threshold1: Box::new(i.threshold1),
                    threshold2: Box::new(i.threshold2),
                    sequenceregion1: j.sequence[i.start1..i.end1].to_string(),
                    sequenceregion2: j.sequence[i.start2..i.end2].to_string(),
                });
            }
        }
    }

    for i in commonpointvariant.iter() {
        writeln!(
            common_point,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            i.name,
            i.start1,
            i.end1,
            i.start2,
            i.end2,
            i.delorig1,
            i.deltype1,
            i.delorig2,
            i.deltype2,
            i.threshold1,
            i.threshold2,
            i.sequenceregion1,
            i.sequenceregion2
        )
        .expect("file not present");
    }

    // implemented reverse slicing when the coorindate are not range compatible.

    for i in end_point_compare.iter() {
        for j in fasta_unload.iter() {
            if i.name == j.header && i.end1 == i.end2 && i.start1 > i.start2 {
                endpointsnatcher.push(Fastasnatcher {
                    name: i.name.clone(),
                    start1: i.start1,
                    end1: i.end1,
                    start2: i.start2,
                    end2: i.end2,
                    delorig1: i.delorig1.clone(),
                    deltype1: i.deltype1.clone(),
                    delorig2: i.delorig2.clone(),
                    deltype2: i.deltype2.clone(),
                    threshold1: i.threshold1.clone(),
                    threshold2: i.threshold2.clone(),
                    sequenceadd: j.sequence[i.start2..i.start1]
                        .to_string()
                        .chars()
                        .rev()
                        .map(|x| String::from(x))
                        .collect::<Vec<_>>()
                        .join(""),
                    sequenceregion1: j.sequence[i.start1..i.end1].to_string(),
                    sequenceregion2: j.sequence[i.start2..i.end2].to_string(),
                })
            } else if i.name == j.header && i.end1 == i.end2 && i.start1 < i.start2 {
                endpointsnatcher.push(Fastasnatcher {
                    name: i.name.clone(),
                    start1: i.start1,
                    end1: i.end1,
                    start2: i.start2,
                    end2: i.end2,
                    delorig1: i.delorig1.clone(),
                    deltype1: i.deltype1.clone(),
                    delorig2: i.delorig2.clone(),
                    deltype2: i.deltype2.clone(),
                    threshold1: i.threshold1.clone(),
                    threshold2: i.threshold2.clone(),
                    sequenceadd: j.sequence[i.start1..i.start2].to_string(),
                    sequenceregion1: j.sequence[i.start1..i.end1].to_string(),
                    sequenceregion2: j.sequence[i.start2..i.end2].to_string(),
                })
            }
        }
    }

    // implemented the reverse slicing when the range parameters are not in the range.

    for i in start_point_compare.iter() {
        for j in fasta_unload.iter() {
            if i.name == j.header && i.start1 == i.start2 && i.end1 > i.end2 {
                startpointsnatcher.push(Fastasnatcher {
                    name: i.name.clone(),
                    start1: i.start1,
                    end1: i.end1,
                    start2: i.start2,
                    end2: i.end2,
                    deltype1: i.deltype1.clone(),
                    delorig1: i.delorig1.clone(),
                    deltype2: i.deltype2.clone(),
                    delorig2: i.delorig2.clone(),
                    threshold1: i.threshold1.clone(),
                    threshold2: i.threshold2.clone(),
                    sequenceadd: j.sequence[i.end2..i.end1]
                        .to_string()
                        .chars()
                        .rev()
                        .map(|x| String::from(x))
                        .collect::<Vec<_>>()
                        .join(""),
                    sequenceregion1: j.sequence[i.start1..i.end1].to_string(),
                    sequenceregion2: j.sequence[i.start2..i.end2].to_string(),
                })
            } else if i.name == j.header && i.start1 == i.start2 && i.end1 < i.end2 {
                startpointsnatcher.push(Fastasnatcher {
                    name: i.name.clone(),
                    start1: i.start1,
                    end1: i.end1,
                    start2: i.start2,
                    end2: i.end2,
                    delorig1: i.delorig1.clone(),
                    deltype1: i.deltype1.clone(),
                    delorig2: i.delorig2.clone(),
                    deltype2: i.deltype2.clone(),
                    threshold1: i.threshold1.clone(),
                    threshold2: i.threshold2.clone(),
                    sequenceadd: j.sequence[i.end1..i.end2].to_string(),
                    sequenceregion1: j.sequence[i.start1..i.end1].to_string(),
                    sequenceregion2: j.sequence[i.start2..i.end2].to_string(),
                })
            }
        }
    }

    for i in endpointsnatcher.iter() {
        writeln!(
            endpointcompare,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            i.name,
            i.start1,
            i.end1,
            i.start2,
            i.end2,
            i.delorig1,
            i.deltype1,
            i.delorig2,
            i.deltype2,
            i.sequenceadd,
            i.sequenceregion1,
            i.sequenceregion2
        )
        .expect("line not present");
    }

    for i in startpointsnatcher.iter() {
        writeln!(
            startpointcompare,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            i.name,
            i.start1,
            i.end1,
            i.start2,
            i.end2,
            i.delorig1,
            i.deltype1,
            i.delorig2,
            i.deltype2,
            i.sequenceadd,
            i.sequenceregion1,
            i.sequenceregion2
        )
        .expect("line not present");
    }

    Ok("VF file have been compared for the pangenomes".to_string())
}

pub fn fasta_estimate(pathfasta: &str) -> Result<Vec<Fasta>, Box<dyn Error>> {
    let fastaopen = File::open(pathfasta).expect("file not present");
    let fastaread = BufReader::new(fastaopen);
    let mut fastaholder: Vec<Fasta> = Vec::new();
    let mut fastaheader: Vec<String> = Vec::new();
    let mut fastasequence: Vec<String> = Vec::new();
    for i in fastaread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            fastaheader.push(line.replace(">", ""));
        } else {
            fastasequence.push(line);
        }
    }

    for i in 0..fastaheader.len() {
        fastaholder.push(Fasta {
            header: fastaheader[i].clone(),
            sequence: fastasequence[i].clone(),
        })
    }

    Ok(fastaholder)
}
