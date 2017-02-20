pub mod log_entry {
    use chrono::prelude::*;
    use std::net::Ipv4Addr;
    use std::time::Duration;
    use std::default::Default;
    use operation::operation::Operation;
    use tokenize::tokenize::Tokenizer;

    static SEPARATORS: &'static [char] = &[' ', ' ', '[', ']', ' ', ' ', ' ', ' ', ' ', ' ',
        '"', '"', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '"', '"', ' ', '"', '"'];

    #[derive(Debug)]
    pub struct LogEntry {
        pub owner_id: Option<String>,
        pub bucket: Option<String>,
        pub timestamp: DateTime<UTC>,
        pub ip_address: Ipv4Addr,
        pub request_id: Option<String>,
        pub requestor_id: Option<String>,
        pub operation: Operation,
        pub path: Option<String>,
        pub request_uri: Option<String>,
        pub http_status: u16,
        pub error_message: Option<String>,
        // TODO: Replace with a list of error codes
        pub bytes_sent: u64,
        pub object_size: u64,
        pub total_time: Duration,
        pub processing_time: Duration,
        pub referrer: Option<String>,
        pub user_agent: Option<String>,
        pub version_id: Option<String>
    }

    impl Default for LogEntry {
        fn default() -> LogEntry {
            LogEntry {
                owner_id: None,
                bucket: None,
                timestamp: Local::now().with_timezone(&UTC),
                ip_address: "127.0.0.1".parse().unwrap(),
                requestor_id: None,
                request_id: None,
                operation: "UNKNOWN.UNKNOWN.UNKNOWN".parse().unwrap(),
                path: None,
                request_uri: None,
                http_status: 0,
                error_message: None,
                bytes_sent: 0,
                object_size: 0,
                total_time: Duration::new(0, 0),
                processing_time: Duration::new(0, 0),
                referrer: None,
                user_agent: None,
                version_id: None
            }
        }
    }

    fn parse_int(data: &str) -> u64 {
        match data.parse::<u64>() {
            Ok(x) => x,
            _ => 0
        }
    }

    macro_rules! get_next_token {
        ($a:ident) => (Some($a.next().unwrap().to_string()))
    }

    macro_rules! get_next_parsed_token {
        ($a:ident) => ($a.next().unwrap().parse().unwrap())
    }

    impl LogEntry {
        pub fn was_complete_download(&self) -> bool {
            self.bytes_sent == self.object_size
        }

        pub fn from_str<'a>(line: &'a str) -> LogEntry {
            let mut new_entry = LogEntry::default();
            let mut tokenizer = Tokenizer::new(SEPARATORS, line);
            new_entry.owner_id = get_next_token!(tokenizer);
            new_entry.bucket = get_next_token!(tokenizer);
            tokenizer.next(); // Skip open '['
            new_entry.timestamp = DateTime::parse_from_str(tokenizer.next().unwrap(), "%d/%b/%Y:%H:%M:%S %z").unwrap().with_timezone(&UTC);
            tokenizer.next(); // Skip ending space
            new_entry.ip_address = get_next_parsed_token!(tokenizer);
            new_entry.requestor_id = get_next_token!(tokenizer);
            new_entry.request_id = get_next_token!(tokenizer);
            new_entry.operation = get_next_parsed_token!(tokenizer);
            new_entry.path = get_next_token!(tokenizer);
            tokenizer.next(); // Skip open '"'
            new_entry.request_uri = get_next_token!(tokenizer);
            tokenizer.next(); // Skip ending space
            new_entry.http_status = get_next_parsed_token!(tokenizer);
            new_entry.error_message = get_next_token!(tokenizer);
            new_entry.bytes_sent = parse_int(tokenizer.next().unwrap());
            new_entry.object_size = parse_int(tokenizer.next().unwrap());

            new_entry.total_time = Duration::from_millis(parse_int(tokenizer.next().unwrap()));
            new_entry.processing_time = Duration::from_millis(parse_int(tokenizer.next().unwrap()));
            tokenizer.next(); // Skip open '"'
            new_entry.referrer = get_next_token!(tokenizer);
            tokenizer.next(); // Skip ending space
            tokenizer.next(); // Skip open '"'
            new_entry.user_agent = get_next_token!(tokenizer);
            new_entry.version_id = get_next_token!(tokenizer);

            new_entry
        }
    }
}
