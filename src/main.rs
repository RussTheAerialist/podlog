mod operation;

extern crate chrono;

use chrono::prelude::*;
use std::net::Ipv4Addr;
use std::time::Duration;
use operation::operation::Operation;


#[derive(Debug)]
struct LogEntry<'a> {
    owner_id: &'a str,
    bucket: &'a str,
    timestamp: DateTime<UTC>,
    ip_address: Ipv4Addr,
    request_id: &'a str,
    requestor_id: &'a str,
    operation: Operation,
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

impl<'a> LogEntry<'a> {
    fn was_complete_download(&self) -> bool {
        self.bytes_sent == self.object_size
    }

    fn from_str(line: &'a str) -> LogEntry {
        LogEntry {
            owner_id: "OwnerId",
            bucket: "Bucket",
            timestamp: Local::now().with_timezone(&UTC),
            ip_address: Ipv4Addr::new(127,0,0,1),
            requestor_id: "-",
            request_id: "1234",
            operation: Operation::from_str("WEBSITE.GET.OBJECT").unwrap(),
            request_uri: "/index.html",
            http_status: 404,
            error_message: "AccessDenied",
            bytes_sent: 100,
            object_size: 101,
            total_time: Duration::from_millis(70),
            processing_time: Duration::from_millis(42),
            referrer: "-",
            user_agent: "-",
            version_id: "-"
        }
    }
}

fn main() {
    let log = LogEntry::from_str("foo");
    println!("Hello, world! {:?}", log);
    match log.was_complete_download() {
        true => println!("Complete Download"),
        false => println!("Incomplete Download")
    }
}
