extern crate chrono;

use chrono::prelude::*;
use std::net::Ipv4Addr;
use std::time::Duration;

#[derive(Debug)]
enum OperationSource {
    UNKNOWN(String),
    SOAP,
    REST,
    WEBSITE,
    BATCH
}

impl OperationSource {
    fn from_str(data : &str) -> OperationSource {
        match data {
            "SOAP" => OperationSource::SOAP,
            "REST" => OperationSource::REST,
            "WEBSITE" => OperationSource::WEBSITE,
            "BATCH" => OperationSource::BATCH,
            _ => OperationSource::UNKNOWN(data.to_string())
        }
    }
}

#[derive(Debug)]
enum OperationMethod {
    UNKNOWN(String),
    GET,
    PUT,
    DELETE
}

impl OperationMethod {
    fn from_str(data : &str) -> OperationMethod {
        match data {
            "GET" => OperationMethod::GET,
            "PUT" => OperationMethod::PUT,
            "DELETE" => OperationMethod::DELETE,
            _ => OperationMethod::UNKNOWN(data.to_string())
        }
    }
}

#[derive(Debug)]
enum OperationResourceType {
    UNKNOWN(String),
    OBJECT
}

impl OperationResourceType {
    fn from_str(data : &str) -> OperationResourceType {
        match data {
            "OBJECT" => OperationResourceType::OBJECT,
            _ => OperationResourceType::UNKNOWN(data.to_string())
        }
    }
}

#[derive(Debug)]
struct Operation {
    source : OperationSource,
    method : OperationMethod,
    resource_type: OperationResourceType
}

impl Operation {
    fn from_str(data : &str) -> Result<Operation, &str> {
        let mut parts = data.split(".");
        let source_option = parts.next();
        let method_option = parts.next();
        let resource_type_option = parts.next();

        let source = match source_option {
            Some(s) => s,
            None => return Err("Unable to find source")
        };

        let method = match method_option {
            Some(s) => s,
            None => return Err("Unable to find match")
        };

        let resource_type = match resource_type_option {
            Some(s) => s,
            None => return Err("Unable to find resource type")
        };

        Ok(Operation {
            source: OperationSource::from_str(source),
            method: OperationMethod::from_str(method),
            resource_type: OperationResourceType::from_str(resource_type)
        })
    }
}

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
