mod args;
mod clipper;
mod clipseq;
mod estimate;
mod fastaconvert;
mod filesplitpattern;
mod filter;
mod matcher;
mod minimap;
mod motifcatcher;
mod motifone;
mod multiclipseq;
mod multisearch;
mod nanoporepacbio;
mod pafannotate;
mod pafpangenome;
mod panarc;
mod pangenome;
mod pangenomereadsdatabase;
mod precomputed;
mod selectedreads;
mod snatcher;
mod stat;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::clipper::clipperpattern;
use crate::clipseq::clipseqa;
use crate::estimate::harmonicestimate;
use crate::fastaconvert::fastaconvertall;
use crate::filter::readlength;
use crate::matcher::paf_alignments;
use crate::minimap::minimapalignment;
use crate::motifcatcher::motifcatcherupdown;
use crate::motifone::motifsearch;
use crate::multiclipseq::multiclipseqa;
use crate::multisearch::multisearchregex;
use crate::pafannotate::annotatepaf;
use crate::pafpangenome::pangenome_summarize;
use crate::pangenome::pangenome_hifiasm;
use crate::precomputed::precomputealignments;
use crate::selectedreads::selected;
use crate::snatcher::snatcherextract;
use crate::stat::stats;

use clap::Parser;
use panarc::metagenome_annotate;
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
            let command = clipseqa(fastqfile, clippedseq).unwrap();
            println!("The regions have been clipped:{:?}", command);
        }
        Commands::MultiClipSeq {
            fastqfile,
            clipseqfile,
        } => {
            let command = multiclipseqa(fastqfile, clipseqfile).unwrap();
            println!("The file with the clipseq have been clipped:{:?}", command);
        }
        Commands::Minimap {
            fastqfile,
            minimap,
            proteins,
            thread,
        } => {
            let command = minimapalignment(fastqfile, minimap, proteins, thread).unwrap();
            println!("The results of the same are as follows:{:?}", command);
        }
        Commands::Pangenome {
            fastqfile,
            thread,
            proteinfasta,
        } => {
            let command = pangenome_hifiasm(fastqfile, thread, proteinfasta).unwrap();
            println!(
                "The pangenome has been assembled using the pacbiohifi reads:{:?}",
                command
            );
        }
        Commands::Stat { fastqfile } => {
            let command = stats(fastqfile).unwrap();
            println!("The stats for your file is as follows:{:?}", command);
        }
        Commands::PangenomeSummarize { pangenome } => {
            let command = pangenome_summarize(pangenome).unwrap();
            println!("The pangenome has been summarized:{:?}", command);
        }
        Commands::ReadMultisearch {
            readsgenome,
            multisearch,
        } => {
            let command = multisearchregex(readsgenome, multisearch).unwrap();
            println!("The read search have been written:{:?}", command);
        }
        Commands::PafAnnotate {
            pafalignment,
            gfffile,
        } => {
            let command = annotatepaf(pafalignment, gfffile).unwrap();
            println!(
                "The results of the annotate paf have been written:{:?}",
                command
            );
        }
        Commands::Harmonicmean { pafalignment } => {
            let command = harmonicestimate(pafalignment).unwrap();
            println!("The harmonic analysis has been written: {:?}", command);
        }
        Commands::PangenomeMatcher {
            pafgenome1,
            pafgenome2,
        } => {
            let command = paf_alignments(pafgenome1, pafgenome2).unwrap();
            println!(
                "The comparative pangenome files have been written:{:?}",
                command
            );
        }
        Commands::PanArc {
            pafalignment,
            fastafile,
        } => {
            let command = metagenome_annotate(&pafalignment, fastafile).unwrap();
            println!("The panarc has been completed: {:?}", command);
        }
        Commands::Snatcher {
            pafalignment,
            queryfasta,
            referencefasta,
        } => {
            let command = snatcherextract(pafalignment, queryfasta, referencefasta).unwrap();
            println!("The reference snatcher has been completed:{:?}", command);
        }
        Commands::PrecomputedPaf { graphalignment } => {
            let command = precomputealignments(&graphalignment).unwrap();
            println!("The precomputed alignment have been written:{:?}", command);
        }
    }
}
