use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "nanoporepacbio",
    version = "1.0",
    about = "nanoporepacbio"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// convert into fasta
    FastaConvert {
        /// provide the path to the pacbiohifi file
        fastqfile: String,
    },
    /// clipping the regions from the fastq
    ClipperAlign {
        /// provide the path to the fastq file
        fastqfile: String,
        /// provide the path to the region file
        regionfile: String,
    },
    /// scans the reads for the motifs single occurence
    Scanner {
        /// provide the path to the fastq file
        fastqfile: String,
        /// motif to be scanned
        motifscan: String,
    },
    /// scans the reads for the selected region and select the upstream and the downstream.
    Motifcatcher {
        /// provide the path to the fastq file
        fastqfile: String,
        /// motif to be scanned
        motifscan: String,
        /// upstream region
        upstream: String,
        /// downstream region
        downstream: String,
    },
    /// selected reads writer
    Selectedreads {
        /// provide the path to the fastq file
        fastqfile: String,
        /// provide the ids that need to be used
        ids: String,
    },
    /// filter the reads prior to the length
    Filterreads {
        /// provide the path to the fastq file
        fastqfile: String,
        /// read length for filtering
        length: String,
    },
}
