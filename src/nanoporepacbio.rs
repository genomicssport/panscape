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
    query: String,
    query_length: usize,
    query_start: usize,
    query_end: usize,
    strand: String,
    target: String,
    target_length: usize,
    target_start: usize,
    target_end: usize,
    residue_matches: usize,
    alignment_length: usize,
}
