mod operation;
mod tokenize;
mod log_entry;
mod output_entry;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate chrono;
extern crate clap;

use log_entry::LogEntry;
use operation::OperationSource;
use output_entry::{JsonOutput, OutputEntry, OutputMap};
use clap::{Arg, App};
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::collections::{HashMap, HashSet};
use chrono::UTC;

fn main() {
    let matches = App::new("Podlog")
                      .version("0.1")
                      .arg(Arg::with_name("DIRECTORY")
                               .required(true)
                               .index(1))
                      .get_matches();

    let directory = Path::new(matches.value_of("DIRECTORY").unwrap());
    if !directory.is_dir() {
        panic!("{} isn't a directory", directory.to_str().unwrap());
    }

    let mut results: OutputMap = HashMap::new();
    let mut ids: HashSet<String> = HashSet::new();

    let directory_entries = directory.read_dir().ok().unwrap();
    for file in directory_entries {
        let entry = file.ok().unwrap();
        process_log_file(&entry.path(), &mut results, &mut ids);
    }

    let output = JsonOutput {
        data: results,
        ids: ids,
        last_updated: UTC::now(),
    };

    let storage = serde_json::to_string_pretty(&output).unwrap();
    println!("{}", storage);
}

fn process_log_file(path: &Path, results: &mut OutputMap, ids: &mut HashSet<String>) -> () {
    let file = BufReader::new(File::open(&path).unwrap());
    let lines = file.lines().filter_map(|result| result.ok()); // Filter out bad rows
    let entries = lines.map(|x| LogEntry::from_str(&x))
                       .filter(|x| x.operation.source == OperationSource::WEBSITE)
                       .filter(|ref x| x.is_audio_file());


    for entry in entries {
        let output_entry = OutputEntry::from(&entry);

        if !ids.contains(entry.request_id.as_ref().unwrap()) {
            if !results.contains_key(&output_entry.episode_number) {
                results.insert(output_entry.episode_number, Vec::new());
            } else {
                let mut x = results.get_mut(&output_entry.episode_number);
                ids.insert(entry.request_id.clone().unwrap());
                if x.is_some() {
                    x.as_mut().unwrap().push(output_entry);
                }
            }
        }
    }
}
