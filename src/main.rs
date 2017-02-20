mod operation;
mod tokenize;
mod log_entry;

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

fn main() {
    let matches = App::new("Podlog")
        .version("0.1")
        .arg(Arg::with_name("FILENAME")
                .required(true)
                .index(1))
    .get_matches();

    let path = Path::new(matches.value_of("FILENAME").unwrap());
    let file = BufReader::new(File::open(&path).unwrap());
    let lines = file.lines().filter_map(|result| result.ok()); // Filter out bad rows
    let entries = lines.map(|x| LogEntry::from_str(&x))
                       .filter(|x| x.operation.source == OperationSource::WEBSITE)
                       .filter(|ref x| x.is_audio_file());

    let mut results : HashMap<String, Vec<&str> > = HashMap::new();
    for entry in entries {
        let path = entry.path.clone().unwrap();
        if !results.contains_key(&path) {
            results.insert(path, Vec::new());
        } else {
            match results.get_mut(&path) {
                Some(x) => x.push("foo"),
                None => ()
            }
        }
    }
    println!("{:?}", results);
}

// Output Format:
// UniqueID, DateTime, Episode, Was Complete Download
