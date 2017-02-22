use log_entry::LogEntry;
use chrono::{DateTime, UTC};
use std::collections::{HashMap, HashSet};

pub type OutputMap = HashMap<u16, Vec<OutputEntry> >;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonOutput {
    pub data: OutputMap,
    pub ids: HashSet<String>,
    pub last_updated: DateTime<UTC>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputEntry {
    pub unique_id : String,
    pub timestamp : DateTime<UTC>,
    pub episode_number: u16,
    pub complete_download: bool
}

impl OutputEntry {
    pub fn from(e : &LogEntry) -> OutputEntry {
        OutputEntry {
            unique_id: match e.request_id {
                Some(ref x) => x.clone(),
                None => "".to_string()
            },
            timestamp: e.timestamp.clone(),
            episode_number: 0,
            complete_download: e.was_complete_download()
        }
    }
}
