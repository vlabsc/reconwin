// 0.15
// parse events

use evtx::EvtxParser;
use std::path::PathBuf;
use colored::Colorize;


use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};




fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

pub fn parse_events() {
    

    let fp = PathBuf::from(format!("C:\\windows\\System32\\winevt\\Logs\\Application.evtx")); 
    
    let mut count = 1;

    let mut parser = EvtxParser::from_path(fp).unwrap();

    
    for record in parser.records() {
        /*
        match record {
            //Ok(r) => println!("Record {}\n{}", r.event_record_id, r.data),
            Ok(r) => line = r.data,
            Err(e) => eprintln!("{}", e),
        }
        */

        //let line = "hello Provider Name = Data .ends"; //record.unwrap();
        //let start = line.find("Provider Name").unwrap_or(0);
        //let ends = line.find(".").unwrap_or(line.len());
        //let result = &line[start..ends];

        //println!("data again: {:?} {:?} - {:?} => {:?} ", line, start, ends, result);
        //let line = record.unwrap().to_str();
        //let start = line.find("Provider Name").unwrap_or(0);


        //println!("{:?}", line);
        

        //println!("{:?}", record);
        

        if count > 2 {
            break;
        } else {
            count = count + 1;
        }
    }

    /*
    let file = File::open("C:\\windows\\System32\\winevt\\Logs\\Application.evtx").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    */
}