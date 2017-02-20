mod operation;
mod tokenize;
mod log_entry;

extern crate chrono;
extern crate clap;

use log_entry::log_entry::LogEntry;
use clap::{Arg, App};
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

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
    let entries = lines.map(|x| LogEntry::from_str(&x)); // Turn them into log entries

    // Create a hashmap based on path name
}
