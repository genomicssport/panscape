use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "panscape",
    version = "1.0",
    about = "panscape: analyzing pangenomes from reads to stats"
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
    /// motif plus upstream and the downstream.
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
    /// remove the clip regions from the reads
    ClipSeq {
        /// provide the path to the fastq file
        fastqfile: String,
        /// provide the sequences to be clipped.
        clippedseq: String,
    },
    /// remove the multitags for the fastqfile
    MultiClipSeq {
        /// provide the path to the fastq file
        fastqfile: String,
        /// provide the path to the text file
        clipseqfile: String,
    },
    /// assemble pangenome
    Pangenome {
        /// provide the path to the pacbiohifi reads
        fastqfile: String,
        /// provde the thread
        thread: i32,
        /// provide the path to the protein file
        proteinfasta: String,
    },
    /// annotate reads
    Minimap {
        /// provide the path to the fastq file
        fastqfile: String,
        /// provide the path to the proteins
        proteins: String,
        /// provide the path the minimap
        minimap: String,
        /// provide the number of the threads
        thread: String,
    },
    /// annotated stats for your file
    Stat {
        /// path to the fastq file
        fastqfile: String,
    },
    /// pangenome pre-computed alignment
    PangenomeSummarize {
        /// path to the pangenome alignment
        pangenome: String,
    },
    /// multisearch reads across the reads
    ReadMultisearch {
        /// path to the sequenced reads or genome file.
        readsgenome: String,
        /// search pattern for the multi search
        multisearch: String,
    },
    /// annotate your pangenome paf alignment using gtf
    PafAnnotate {
        /// path to the pafalignment
        pafalignment: String,
        /// path to the gtf/gff file
        gfffile: String,
    },
    /// estimate the harmonic mean from the pangenome
    Harmonicmean {
        /// path to the pangenome alignment
        pafalignment: String,
    },
    /// pangenome matcher
    PangenomeMatcher {
        /// path to the first pangenomefile
        pafgenome1: String,
        /// path to the second pangenomefile
        pafgenome2: String,
    },
    /// pangenome annotator
    PanArc {
        /// path to the pangenome alignment
        pafalignment: String,
        /// path to the fasta file,
        fastafile: String,
    },
    /// extract specific region from paf alignment
    Snatcher {
        /// pafalignment file
        pafalignment: String,
        /// query fasta file
        queryfasta: String,
        /// reference fasta
        referencefasta: String,
    },
    /// generate stats from precomputed paf
    PrecomputedPaf {
        /// path to the pre computed alignment
        graphalignment: String,
    },
}
