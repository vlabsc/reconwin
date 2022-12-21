// 0.17.1 - windows events implementation

use evtx::EvtxParser;
use std::path::PathBuf;

pub fn tester_main() {
    // Change this to a path of your .evtx sample. 

    let fp = PathBuf::from(format!("Application.evtx"));
    
    let mut parser = EvtxParser::from_path(fp).unwrap();

    println!("count: {}", parser.records().count());
    
    /*
    for record in parser.records() {
        match record {
            Ok(r) => println!("Record {}\n{}", r.event_record_id, r.data),
            Err(e) => eprintln!("{}", e),
        }
    }
    */
}
