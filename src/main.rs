mod args;
mod clipper;
mod clipseq;
mod fastaconvert;
mod filesplitpattern;
mod filter;
mod motifcatcher;
mod motifone;
mod multiclipseq;
mod nanoporepacbio;
mod selectedreads;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::clipper::clipperpattern;
use crate::clipseq::clipseqa;
use crate::fastaconvert::fastaconvertall;
use crate::filter::readlength;
use crate::motifcatcher::motifcatcherupdown;
use crate::motifone::motifsearch;
use crate::multiclipseq::multiclipseqa;
use crate::selectedreads::selected;
use clap::Parser;

/*
 Author Gaurav Sablok
 SLB Potsdam
 Date: 2025-2-14

*/

fn main() {
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::FastaConvert { fastqfile } => {
            let command = fastaconvertall(fastqfile).unwrap();
            println!("The file has been converted: {:?}", command);
        }
        Commands::ClipperAlign {
            fastqfile,
            regionfile,
        } => {
            let command = clipperpattern(fastqfile, regionfile).unwrap();
            println!("The regions have been clipped: {:?}", command);
        }
        Commands::Scanner {
            fastqfile,
            motifscan,
        } => {
            let command = motifsearch(fastqfile, motifscan).unwrap();
            println!("The scanned motif files have been written:{:?}", command);
        }
        Commands::Motifcatcher {
            fastqfile,
            motifscan,
            upstream,
            downstream,
        } => {
            let command = motifcatcherupdown(fastqfile, motifscan, upstream, downstream).unwrap();
            println!(
                "The results of the motifcatcher have been written:{:?}",
                command
            );
        }
        Commands::Filterreads { fastqfile, length } => {
            let command = readlength(fastqfile, length).unwrap();
            println!("The reads have been filtered: {:?}", command);
        }
        Commands::Selectedreads { fastqfile, ids } => {
            let command = selected(fastqfile, ids).unwrap();
            println!(
                "The file with the selected ids have been written:{:?}",
                command
            );
        }
        Commands::ClipSeq {
            fastqfile,
            clippedseq,
        } => {
            let command = clipseqa(fastqfile, &clippedseq).unwrap();
            println!("The regions have been clipped:{:?}", command);
        }
        Commands::MultiClipSeq {
            fastqfile,
            clipseqfile,
        } => {
            let command = multiclipseqa(fastqfile, clipseqfile).unwrap();
            println!("The file with the clipseq have been clipped:{:?}", command);
        }
    }
}
