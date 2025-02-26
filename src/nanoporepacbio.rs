use tabled::Tabled;

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct Sequence {
    pub header: String,
    pub sequence: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct Search {
    pub name: String,
    pub sequence: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct Motifcatcher {
    pub name: String,
    pub sequence: String,
    pub start: usize,
    pub end: usize,
    pub upstream: String,
    pub downstream: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct Filesnatch {
    pub name: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct Alignment {
    pub alignmentregion: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct Code {
    pub parent_id: String,
    pub cds_start: String,
    pub cds_end: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Readlength {
    pub five: usize,
    pub hundred: usize,
    pub hundredfifty: usize,
    pub twohundred: usize,
    pub morethan: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]

pub struct Pafanalyzer {
    pub query: String,
    pub query_length: usize,
    pub query_start: usize,
    pub query_end: usize,
    pub strand: String,
    pub target: String,
    pub target_length: usize,
    pub target_start: usize,
    pub target_end: usize,
    pub residue_matches: usize,
    pub alignment_length: usize,
    pub quality: usize,
}

#[derive(Debug, Tabled)]
pub struct Tableinformation {
    pub information: &'static str,
    pub query: usize,
    pub residue: usize,
    pub alignment: usize,
    pub target: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct Multiplepattern<'a> {
    pub name: &'a String,
    pub collectionvec: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct PAFAnnotate {
    pub target: String,
    pub target_gff: String,
    pub target_start: usize,
    pub target_end: usize,
    pub target_gff_start: usize,
    pub target_gff_end: usize,
    pub annotategene: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct QuerypafView {
    pub query: String,
    pub length: usize,
    pub start: usize,
    pub end: usize,
    pub strand: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ReferencepafView {
    pub reference: String,
    pub length: usize,
    pub start: usize,
    pub end: usize,
    pub residuematch: usize,
    pub alignmentblock: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct QueryComparativeHolder {
    pub query1: String,
    pub length1: usize,
    pub start1: usize,
    pub end1: usize,
    pub strand1: String,
    pub query2: String,
    pub length2: usize,
    pub start2: usize,
    pub end2: usize,
    pub strand2: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RefComparativeHolder {
    pub reference1: String,
    pub length1: usize,
    pub start1: usize,
    pub end1: usize,
    pub reference2: String,
    pub length2: usize,
    pub start2: usize,
    pub end2: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ResidueAlignment {
    pub residuematch1: usize,
    pub residuematch2: usize,
    pub alignmentblock1: usize,
    pub alignmentblock2: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AlignmentGFF {
    pub id: String,
    pub genomefeature: String,
    pub start: usize,
    pub end: usize,
    pub strand: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Positive {
    pub id: String,
    pub genomefeature: String,
    pub start: usize,
    pub end: usize,
    pub strand: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Negative {
    pub id: String,
    pub genomefeature: String,
    pub start: usize,
    pub end: usize,
    pub strand: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Sequenceadd {
    pub id: String,
    pub sequence: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CaptureSeq {
    pub id: String,
    pub seq: String,
    pub strand: String,
}
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MrnaAnnotate {
    pub id: String,
    pub seq: String,
    pub strand: String,
}
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct CdsAnnotate {
    pub id: String,
    pub seq: String,
    pub strand: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct PositiveAnnotate {
    pub id: String,
    pub seq: String,
    pub strand: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct NegativeAnnotate {
    pub id: String,
    pub seq: String,
    pub strand: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CdsExtract<'a> {
    pub header: &'a String,
    pub cdsordinate: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CdsExtractSeq<'a> {
    pub header: &'a String,
    pub cdsordinate: Vec<(usize, usize)>,
    pub cdsextract: Vec<String>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Segments {
    pub segment: String,
    pub id: String,
    pub seq: String,
    pub tag: String,
    pub aligntag: String,
    pub alignmenttag: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Links {
    pub link: String,
    pub id1: String,
    pub strand1: String,
    pub id2: String,
    pub strand2: String,
    pub tag: String,
    pub arc: String,
}

#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]
pub struct FinalWrite {
    pub number_segment: usize,
    pub number_links: usize,
    pub number_arc: usize,
    pub max_rank: usize,
    pub total_segment_length: usize,
    pub average_segment_length: usize,
    pub sum_0_segment_length: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Segment {
    pub segment: String,
    pub id: String,
    pub seq: String,
    pub connection: String,
    pub tag: String,
}

#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]
pub struct Analyze {
    pub tag: String,
    pub start: usize,
    pub end: usize,
    pub oritag: String,
    pub rank: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct BedtoolsRange {
    pub alignedref: String,
    pub alignedstart: usize,
    pub alignedend: usize,
    pub score: usize,
    pub strand: String,
    pub difference: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Vecstorage {
    pub start: usize,
    pub end: usize,
    pub difference: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct VCFRange {
    pub name: String,
    pub start: usize,
    pub end: usize,
    pub delorig: String,
    pub deltype: String,
    pub threshold: Box<f64>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Endpointcompare {
    pub name: String,
    pub start1: usize,
    pub end1: usize,
    pub start2: usize,
    pub end2: usize,
    pub delorig1: String,
    pub deltype1: String,
    pub threshold1: Box<f64>,
    pub delorig2: String,
    pub deltype2: String,
    pub threshold2: Box<f64>,
    pub difference: Box<usize>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Startpointcompare {
    pub name: String,
    pub start1: usize,
    pub end1: usize,
    pub start2: usize,
    pub end2: usize,
    pub delorig1: String,
    pub deltype1: String,
    pub threshold1: Box<f64>,
    pub delorig2: String,
    pub deltype2: String,
    pub threshold2: Box<f64>,
    pub difference: Box<usize>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Common {
    pub name: String,
    pub start1: usize,
    pub end1: usize,
    pub start2: usize,
    pub end2: usize,
    pub delorig1: String,
    pub deltype1: String,
    pub delorig2: String,
    pub deltype2: String,
    pub threshold1: f64,
    pub threshold2: f64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Fasta {
    pub header: String,
    pub sequence: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Fastasnatcher {
    pub name: String,
    pub start1: usize,
    pub end1: usize,
    pub start2: usize,
    pub end2: usize,
    pub delorig1: String,
    pub deltype1: String,
    pub threshold1: Box<f64>,
    pub delorig2: String,
    pub deltype2: String,
    pub threshold2: Box<f64>,
    pub sequenceadd: String,
    pub sequenceregion1: String,
    pub sequenceregion2: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Commonsnatcher {
    pub name: String,
    pub start1: usize,
    pub end1: usize,
    pub start2: usize,
    pub end2: usize,
    pub delorig1: String,
    pub deltype1: String,
    pub threshold1: Box<f64>,
    pub delorig2: String,
    pub deltype2: String,
    pub threshold2: Box<f64>,
    pub sequenceregion1: String,
    pub sequenceregion2: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MergeBed {
    pub bed1: String,
    pub start1: usize,
    pub end1: usize,
    pub bedseq: Vec<String>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MergeFasta {
    pub bed: String,
    pub start1: usize,
    pub end1: usize,
    pub mergedseq: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct BedtoolsRangeMulti {
    pub alignedref: String,
    pub alignedstart: usize,
    pub alignedend: usize,
    pub difference: usize,
}
