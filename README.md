# nanopore-pacbio-all
 - from reads to alignments to pangenome, metagenome inclduing the genome annotation using the nanopore and pacbio under one rust binary
 - this code will be updated regularly. 

 ```
 cargo build
 ```

 ```
 12:29:37 gauravsablok@genome nanopore-pacbio-all main ? ./target/debug/nanopore-pacbio-all -h
 nanoporepacbio

 Usage: nanopore-pacbio-all <COMMAND>

 Commands:
  fasta-convert        convert into fasta
  clipper-align        clipping the regions from the fastq
  scanner              scans the reads for the motifs single occurence
  motifcatcher         motif plus upstream and the downstream
  selectedreads        selected reads writer
  filterreads          filter the reads prior to the length
  clip-seq             remove the clip regions from the reads
  multi-clip-seq       remove the multitags for the fastqfile
  pangenome            assemble pangenome
  minimap              annotate reads
  stat                 annotated stats for your file
  pangenome-summarize  pangenome pre-computed alignment
  help                 Print this message or the help of the given subcommand(s)

 Options:
  -h, --help     Print help
  -V, --version  Print version
 ```
 Gaurav Sablok
