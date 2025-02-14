# nanopore-pacbio-all
 - rust oxford and nanopore algorithm implementation
 - from alignments to pangenome, metagenome brining all the nanopore and pacbio under one rust binary
 - all skiplist implementation, Btree, Stree, implementation along with the rayon multithreaded. 
 - this code will be updated regularly. 

 ```
 cargo build
 ```
 ```
 14:10:33 gauravsablok@genome nanopore-pacbio-all main ? ./target/debug/nanopore-pacbio-all -h
 nanoporepacbio

 Usage: nanopore-pacbio-all <COMMAND>

 Commands:
  fasta-convert  convert into fasta
  clipper-align  clipping the regions from the fastq
  scanner        scans the reads for the motifs single occurence
  motifcatcher   scans the reads for the selected region and select the upstream and the downstream
  selectedreads  selected reads writer
  filterreads    filter the reads prior to the length
  help           Print this message or the help of the given subcommand(s)

 Options:
  -h, --help     Print help
  -V, --version  Print version
```
 Gaurav Sablok 
