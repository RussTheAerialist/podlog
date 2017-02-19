mod operation;
mod tokenize;
mod log_entry;

extern crate chrono;

use log_entry::log_entry::LogEntry;

fn main() {
    let test_line = "8263874e0c9548cbd23b87b8618c316a2ffb6d29e72aaef03a8b275cd11cb23a episodes.batmanslittlebird.com [20/Oct/2016:20:35:11 +0000] 69.25.143.32 - F5C37873583CA70C WEBSITE.GET.OBJECT index.html \"GET / HTTP/1.1\" 403 AccessDenied 303 - 297 - \"-\" \"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.11; rv:49.0) Gecko/20100101 Firefox/49.0\"";
    let log = LogEntry::from_str(test_line);
    println!("{:?}", log);
    match log.was_complete_download() {
        true => println!("Complete Download"),
        false => println!("Incomplete Download")
    }
}
