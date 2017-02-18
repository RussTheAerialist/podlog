mod operation;
mod tokenize;

extern crate chrono;

use chrono::prelude::*;
use std::net::Ipv4Addr;
use std::time::Duration;
use std::default::Default;
use operation::operation::Operation;
use tokenize::tokenize::Tokenizer;

static SEPARATORS: &'static [char] = &[' ', ' ', '[', ']', ' ', ' ', ' ', ' ', ' ', ' ',
    '"', '"', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '"', '"', ' ', '"', '"'];

#[derive(Debug)]
struct LogEntry<'a> {
    owner_id: &'a str,
    bucket: &'a str,
    timestamp: DateTime<UTC>,
    ip_address: Ipv4Addr,
    request_id: &'a str,
    requestor_id: &'a str,
    operation: Operation,
    path: &'a str,
    request_uri: &'a str,
    http_status: u16,
    error_message: &'a str, // TODO: Replace with a list of error codes
    bytes_sent: u64,
    object_size: u64,
    total_time: Duration,
    processing_time: Duration,
    referrer: &'a str,
    user_agent: &'a str,
    version_id: &'a str
}

impl<'a> Default for LogEntry<'a> {
    fn default() -> LogEntry<'a> {
        LogEntry {
            owner_id: "-",
            bucket: "-",
            timestamp: Local::now().with_timezone(&UTC),
            ip_address: "127.0.0.1".parse().unwrap(),
            requestor_id: "-",
            request_id: "-",
            operation: "UNKNOWN.UNKNOWN.UNKNOWN".parse().unwrap(),
            path: "-",
            request_uri: "-",
            http_status: 0,
            error_message: "-",
            bytes_sent: 0,
            object_size: 0,
            total_time: Duration::new(0, 0),
            processing_time: Duration::new(0, 0),
            referrer: "-",
            user_agent: "-",
            version_id: "-"
        }
    }
}

fn parse_int(data : &str) -> u64 {
    match data.parse::<u64>() {
        Ok(x) => x,
        _ => 0
    }
}

impl<'a> LogEntry<'a> {
    fn was_complete_download(&self) -> bool {
        self.bytes_sent == self.object_size
    }

    fn from_str(line: &'a str) -> LogEntry {
        let mut new_entry = LogEntry::default();
        let mut tokenizer = Tokenizer::new(SEPARATORS, line);
        new_entry.owner_id = tokenizer.next().unwrap();
        new_entry.bucket = tokenizer.next().unwrap();
        tokenizer.next(); // Skip open '['
        new_entry.timestamp = DateTime::parse_from_str(tokenizer.next().unwrap(), "%d/%b/%Y:%H:%M:%S %z").unwrap().with_timezone(&UTC);
        tokenizer.next(); // Skip ending space
        new_entry.ip_address = tokenizer.next().unwrap().parse().unwrap();
        new_entry.requestor_id = tokenizer.next().unwrap();
        new_entry.request_id = tokenizer.next().unwrap();
        new_entry.operation = tokenizer.next().unwrap().parse().unwrap();
        new_entry.path = tokenizer.next().unwrap();
        tokenizer.next(); // Skip open '"'
        new_entry.request_uri = tokenizer.next().unwrap();
        tokenizer.next(); // Skip ending space
        new_entry.http_status = tokenizer.next().unwrap().parse().unwrap();
        new_entry.error_message = tokenizer.next().unwrap();
        new_entry.bytes_sent = parse_int(tokenizer.next().unwrap());
        new_entry.object_size = parse_int(tokenizer.next().unwrap());

        new_entry.total_time = Duration::from_millis(parse_int(tokenizer.next().unwrap()));
        new_entry.processing_time = Duration::from_millis(parse_int(tokenizer.next().unwrap()));
        tokenizer.next(); // Skip open '"'
        new_entry.referrer = tokenizer.next().unwrap();
        tokenizer.next(); // Skip ending space
        tokenizer.next(); // Skip open '"'
        new_entry.user_agent = tokenizer.next().unwrap();
        new_entry.version_id = tokenizer.next().unwrap();

        new_entry
    }
}


fn main() {
    let test_line = "8263874e0c9548cbd23b87b8618c316a2ffb6d29e72aaef03a8b275cd11cb23a episodes.batmanslittlebird.com [20/Oct/2016:20:35:11 +0000] 69.25.143.32 - F5C37873583CA70C WEBSITE.GET.OBJECT index.html \"GET / HTTP/1.1\" 403 AccessDenied 303 - 297 - \"-\" \"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.11; rv:49.0) Gecko/20100101 Firefox/49.0\"";
    let log = LogEntry::from_str(test_line);
    println!("{:?}", log);
    match log.was_complete_download() {
        true => println!("Complete Download"),
        false => println!("Incomplete Download")
    }
}
