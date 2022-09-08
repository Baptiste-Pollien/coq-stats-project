use crate::file_analysis::stats_coq::StatsCoq;


#[derive(Debug)]
pub struct StatsFile {
    pub name: String,
    pub lines: u64,
    pub blanks: u64,
    pub code: u64,
    pub comments: u64,
    pub coq_stats: StatsCoq,
}

impl StatsFile {
    pub fn new(file_name: &String) -> Self {
        Self {
            name: String::from(file_name),
            lines: 0,
            blanks: 0,
            code: 0,
            comments: 0,
            coq_stats: StatsCoq::new(),
        }
    }
}