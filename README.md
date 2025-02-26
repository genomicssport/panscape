# panscape
 - from reads to alignments to pangenome, metagenome inclduing the genome annotation using the nanopore and pacbio under one rust binary
 - this will also creates the pangenome database and also the pangenome reads database for rest api. 
 - this code will be updated regularly. 

 ```
 cargo build
 ```

 ```
 gauravsablok@genome panscape main ? ./target/debug/panscape -h
 panscape: analyzing pangenomes from reads to stats

 Usage: panscape <COMMAND>

 Commands:
  fasta-convert           convert into fasta
  clipper-align           clipping the regions from the fastq
  scanner                 scans the reads for the motifs single occurence
  motifcatcher            motif plus upstream and the downstream
  selectedreads           selected reads writer
  filterreads             filter the reads prior to the length
  clip-seq                remove the clip regions from the reads
  multi-clip-seq          remove the multitags for the fastqfile
  pangenome               assemble pangenome
  minimap                 annotate reads
  stat                    annotated stats for your file
  pangenome-summarize     pangenome pre-computed alignment
  read-multisearch        multisearch reads across the reads
  paf-annotate            annotate your pangenome paf alignment using gtf
  harmonicmean            estimate the harmonic mean from the pangenome
  pangenome-matcher       pangenome matcher
  pan-arc                 pangenome annotator
  snatcher                extract specific region from paf alignment
  precomputed-paf         generate stats from precomputed paf
  precompute-cds          extract the coding regions from the precomputed pangenome
  graph                   graph analyzer
  pangenome-bed           Pangenome bed constructor
  intergenic-noncoding    Intergenic extractor
  pan-reads-database      pangenome database
  bedtool-ancestral       analyze pangenome from the bedtools alignment to ancestral state
  vcf-aanalyze            analyze pangenome vcffiles
  pangenome-single-merge  merge single pangenome bedfile
  help                    Print this message or the help of the given subcommand(s)

 Options:
  -h, --help     Print help
  -V, --version  Print version
 ```
 Gaurav Sablok
