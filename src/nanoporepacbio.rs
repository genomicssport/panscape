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
