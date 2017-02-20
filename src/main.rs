mod operation;
mod tokenize;
mod log_entry;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate chrono;
extern crate clap;

use log_entry::log_entry::LogEntry;
use operation::operation::OperationSource;
use clap::{Arg, App};
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use chrono::{DateTime, UTC};

#[derive(Debug, Serialize, Deserialize)]
struct OutputEntry {
    unique_id : String,
    timestamp : DateTime<UTC>,
    episode_number: u16,
    complete_download: bool
}

impl OutputEntry {
    fn from(e : &LogEntry) -> OutputEntry {
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

fn main() {
    let matches = App::new("Podlog")
        .version("0.1")
        .arg(Arg::with_name("FILENAME")
            .required(true)
            .index(1))
        .get_matches();

    let filename = matches.value_of("FILENAME").unwrap();
    foo(filename);
}

fn foo(filename : &str) -> () {

    let path = Path::new(filename);
    let file = BufReader::new(File::open(&path).unwrap());
    let lines = file.lines().filter_map(|result| result.ok()); // Filter out bad rows
    let entries = lines.map(|x| LogEntry::from_str(&x))
                       .filter(|x| x.operation.source == OperationSource::WEBSITE)
                       .filter(|ref x| x.is_audio_file());

    let mut results : HashMap<String, Vec<OutputEntry> > = HashMap::new();
    for entry in entries {
        let path = entry.path.clone().unwrap();
        if !results.contains_key(&path) {
            results.insert(path, Vec::new());
        } else {
            let mut x = results.get_mut(&path);
            if x.is_some() {
                x.as_mut().unwrap().push(OutputEntry::from(&entry));
            }
        }
    }
    let storage = serde_json::to_string_pretty(&results).unwrap();
    println!("{}", storage);
}

// Output Format:
// UniqueID, DateTime, Episode, Was Complete Download
