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
