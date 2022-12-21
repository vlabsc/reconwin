// 0.17 - new version

use sysinfo::{Pid, ProcessExt, System, SystemExt, UserExt};
use colored::Colorize;
use std::{env, fs};

use evtx::EvtxParser;
use std::path::PathBuf;
use std::collections::HashMap;


pub struct windowseventsprobe {
    pub application_logs: String,
    pub security_logs: String,
    pub setup_logs: String,
    pub system_logs: String,
    pub forwardedevents_logs: String,
}

pub struct windowseventsprobe_howmany {
    pub info_happlication_logs: String,
    pub info_hsecurity_logs: String,
    pub info_hsetup_logs: String,
    pub info_hsystem_logs: String,
    pub info_hfe_logs: String,
    pub info_htot_logs: String,
}


impl windowseventsprobe {

    pub fn windows_events_application_logs(&mut self) {

        let mut event_level_map: HashMap<i32, &str> = HashMap::new();

        event_level_map.insert (0, "Log Always");
        event_level_map.insert (1, "Critical");
        event_level_map.insert (2, "Error");
        event_level_map.insert (3, "Warning");
        event_level_map.insert (4, "Informational");
        event_level_map.insert (5, "Verbose");
        


        self.application_logs = format!("probing application logs... \n").cyan().bold().to_string();
    
        let fp = PathBuf::from(format!("C:\\Windows\\System32\\winevt\\Logs\\Application.evtx"));
        let mut parser = EvtxParser::from_path(fp).unwrap();

        let mut row = 0;
        self.application_logs.push_str(format!("----------------------------------------------------------------------------------------------------------------------------------------\n").as_str());
        self.application_logs.push_str(format!(" event    source                                        time                          level               task     pid      tid\n").as_str());
        self.application_logs.push_str(format!("----------------------------------------------------------------------------------------------------------------------------------------\n").as_str());
        let mut recordscount = 0;

        for record in parser.records() {
            match record {
                Ok(r) => {
                    self.application_logs.push_str(format!(" {}", r.event_record_id).as_str());
                    let mut totalspaces = 0;

                    let mut eventidstring = format!("{}", r.event_record_id);
                    
                    //logic to print spaces - 9 spaces                    
                    totalspaces = 9 -  (eventidstring.len()) + 1;
                    for x in 0..totalspaces {
                        //print(" ");
                        self.application_logs.push_str(format!(" ").as_str());
                    }


                    // logic to find sourcename
                    let mut startindex = r.data.find("EventSourceName=").unwrap_or(0) + 17;
                    let mut int_string = &r.data[startindex..];
                    let mut endindex = int_string.find("\"").unwrap_or(0);
                    let mut final_string = &int_string[0..endindex];

                    if startindex == 17 {
                        self.application_logs.push_str(format!("No event source found.").as_str());
                        totalspaces = 46 - ("No event source found.".len());
                    } else {
                        self.application_logs.push_str(format!("{}", final_string).as_str());
                        totalspaces = 46 - final_string.len();
                    }

                    //logic to print spaces - 46 spaces
                    for x in 0..totalspaces {
                        //print(" ");
                        self.application_logs.push_str(format!(" ").as_str());
                    }

                    // logic to find timestamp
                    let mut startindex = r.data.find("SystemTime=").unwrap_or(0) + 12;
                    let mut int_string = &r.data[startindex..];
                    let mut endindex = int_string.find("\"").unwrap_or(0);
                    let mut final_string = &int_string[0..endindex];


                    //logic to print spaces - 30 spaces
                    if startindex == 12 {
                        self.application_logs.push_str(format!("No time found.").as_str());
                        totalspaces = 30 - ("No time found.".len());
                    } else {
                        self.application_logs.push_str(format!("{}", final_string).as_str());
                        totalspaces = 30 - (final_string.len());
                    }

                    for x in 0..totalspaces {
                        //print(" ");
                        self.application_logs.push_str(format!(" ").as_str());
                    }

                    /*
                    // logic to find eventid
                    let mut startindex = r.data.find("<EventRecordID>").unwrap_or(0) + 15;
                    let mut int_string = &r.data[startindex..];
                    let mut endindex = int_string.find("</E").unwrap_or(0);
                    let mut final_string = &int_string[0..endindex];

                    if startindex == 15 {
                        self.application_logs.push_str(format!("No event record id.").as_str());
                        totalspaces = 11 - ("No event record id.".len());
                    } else {
                        self.application_logs.push_str(format!("{}", final_string).as_str());
                        totalspaces = 11 - (final_string.len());
                    }


                    //logic to print spaces - 30 spaces
                    for x in 0..totalspaces {
                        self.application_logs.push_str(format!(" ").as_str());
                    }
                    */

                    // logic to find level
                    let mut startindex = r.data.find("<Level>").unwrap_or(0) + 7;
                    let mut int_string = &r.data[startindex..];
                    let mut endindex = int_string.find("</L").unwrap_or(0);
                    
                    let mut finali_string = &int_string[0..endindex];
                    let mut integerval = finali_string.parse::<i32>().unwrap();

                    let mut final_string = &int_string[0..endindex];

                    let mut finall_string = String::new();
                    finall_string = format!("{} ({})", finali_string, event_level_map.get(&integerval).copied().unwrap());

                    //println!("level: {}", finall_string);                    


                    if startindex == 15 {
                        self.application_logs.push_str(format!("No level id.").as_str());
                        totalspaces = 20 - ("No level id.".len());
                    } else {
                        self.application_logs.push_str(format!("{}", finall_string).as_str());
                        totalspaces = 20 - (finall_string.len());
                    }

                    //logic to print spaces - 30 spaces
                    for x in 0..totalspaces {
                        self.application_logs.push_str(format!(" ").as_str());
                    }

                    // logic to find task
                    let mut startindex = r.data.find("<Task>").unwrap_or(0) + 6;
                    let mut int_string = &r.data[startindex..];
                    let mut endindex = int_string.find("</T").unwrap_or(0);
                    let mut final_string = &int_string[0..endindex];

                    if startindex == 15 {
                        self.application_logs.push_str(format!("No task id.").as_str());
                        totalspaces = 9 - ("No task id.".len());
                    } else {
                        self.application_logs.push_str(format!("{}", final_string).as_str());
                        totalspaces = 9 - (final_string.len());
                    }

                    //logic to print spaces - 30 spaces
                    for x in 0..totalspaces {
                        self.application_logs.push_str(format!(" ").as_str());
                    }

                    // logic to find pid
                    let mut startindex = r.data.find("<Execution ProcessID=").unwrap_or(0) + 22;
                    let mut int_string = &r.data[startindex..];
                    let mut endindex = int_string.find("\"").unwrap_or(0);
                    let mut final_string = &int_string[0..endindex];

                    if startindex == 22 {
                        self.application_logs.push_str(format!("No pid.").as_str());
                        totalspaces = 9 - ("No pid.".len());
                    } else {
                        self.application_logs.push_str(format!("{}", final_string).as_str());
                        totalspaces = 9 - (final_string.len());
                    }

                    //logic to print spaces - 30 spaces
                    for x in 0..totalspaces {
                        self.application_logs.push_str(format!(" ").as_str());
                    }

                    // logic to find tid thread id
                    let mut startindex = r.data.find("<ThreadtID=").unwrap_or(0) + 11;
                    let mut int_string = &r.data[startindex..];
                    let mut endindex = int_string.find("\"").unwrap_or(0);
                    let mut final_string = &int_string[0..endindex];

                    if startindex == 11 {
                        self.application_logs.push_str(format!("No tid.").as_str());
                        totalspaces = 9 - ("No tid.".len());
                    } else {
                        self.application_logs.push_str(format!("{}", final_string).as_str());
                        totalspaces = 9 - (final_string.len());
                    }

                    //logic to print spaces - 30 spaces
                    for x in 0..totalspaces {
                        self.application_logs.push_str(format!(" ").as_str());
                    }









                    //let newlevel = 0;
                    //println!("level: {}", event_level_map.get(&newlevel).copied().unwrap());

                    self.application_logs.push_str(format!("\n").as_str());
                    row = row + 1;
                    recordscount = recordscount + 1;

                    //self.application_logs.push_str(format!("len: {} ", totalspaces).as_str());
                    //println!("len: {}", totalspaces);
                    
                    //println!("Original: {}", r.data);

                },
                Err(e) => println!("error in record: {}", e),
            }
            //row = row + 1;
            /*if row > 509 {
                break;
            }*/
            if row > 32 {
                self.application_logs.push_str(format!("----------------------------------------------------------------------------------------------------------------------------------------\n").as_str());
                self.application_logs.push_str(format!(" event    source                                        time                          level               task     pid      tid\n").as_str());
                self.application_logs.push_str(format!("-----------------------------------------------------------------------------------------------------------------------------------------\n").as_str());
                row = 0;
            }
            //println!("");
        }

        self.application_logs.push_str(format!("recordcount: {} \n", recordscount).as_str());
    }    
}

impl windowseventsprobe_howmany {

    pub fn probe_happlication_logs(&mut self) -> usize {

        self.info_happlication_logs = format!("total number of application logs: ").cyan().bold().to_string();

        /*
        let evtx_dir = format!("{}", "C:\\Windows\\System32\\winevt\\Logs\\");
        env::set_current_dir(&evtx_dir);
        let fp = PathBuf::from(format!("Application.evtx"));
        */
        
        let fp = PathBuf::from(format!("C:\\Windows\\System32\\winevt\\Logs\\Application.evtx"));
        let mut parser = EvtxParser::from_path(fp).unwrap();
    
        self.info_happlication_logs.push_str(format!("{} \n", parser.records().count()).as_str());

        parser.records().count()
    }

}

impl windowseventsprobe_howmany {

    pub fn probe_hsecurity_logs(&mut self) -> usize {

        self.info_hsecurity_logs = format!("total number of security logs: ").cyan().bold().to_string();
        
        let fp = PathBuf::from(format!("C:\\Windows\\System32\\winevt\\Logs\\Security.evtx"));
        let mut parser = EvtxParser::from_path(fp).unwrap();
    
        self.info_hsecurity_logs.push_str(format!("{} \n", parser.records().count()).as_str());
        parser.records().count()
    }

}

impl windowseventsprobe_howmany {

    pub fn probe_hsetup_logs(&mut self) -> usize {

        self.info_hsetup_logs = format!("total number of setup logs: ").cyan().bold().to_string();
        
        let fp = PathBuf::from(format!("C:\\Windows\\System32\\winevt\\Logs\\Setup.evtx"));
        let mut parser = EvtxParser::from_path(fp).unwrap();
    
        self.info_hsetup_logs.push_str(format!("{} \n", parser.records().count()).as_str());
        parser.records().count()
    }
}


impl windowseventsprobe_howmany {

    pub fn probe_hsystem_logs(&mut self) -> usize {

        self.info_hsystem_logs = format!("total number of system logs: ").cyan().bold().to_string();
        
        let fp = PathBuf::from(format!("C:\\Windows\\System32\\winevt\\Logs\\System.evtx"));
        let mut parser = EvtxParser::from_path(fp).unwrap();
    
        self.info_hsystem_logs.push_str(format!("{} \n", parser.records().count()).as_str());
        parser.records().count()
    }
}

impl windowseventsprobe_howmany {

    pub fn probe_hfe_logs(&mut self) -> usize {

        return 0;

        self.info_hfe_logs = format!("total number of forwarded events logs: ").cyan().bold().to_string();
        
        let fp = PathBuf::from(format!("C:\\Windows\\System32\\winevt\\Logs\\System.evtx"));
        let mut parser = EvtxParser::from_path(fp).unwrap();
    
        self.info_hfe_logs.push_str(format!("{} \n", parser.records().count()).as_str());
        parser.records().count()
    }
}

impl windowseventsprobe_howmany {

    pub fn probe_htot_logs(&mut self) {

        self.info_htot_logs = format!("total number of events under Windows logs... this may take some time... please wait... \n").cyan().bold().to_string();
        
        let mut retcount = 0;
        let mut count = 0;

        count = count + retcount;
        retcount = 0;

        println!("calculating total number of application logs available ...");
        retcount = self.probe_happlication_logs();
        self.info_htot_logs = format!("application logs: {} \n", retcount);
        count = count + retcount;
        retcount = 0;

        println!("calculating total number of security logs available ...");
        retcount = self.probe_hsecurity_logs();
        self.info_htot_logs.push_str(format!("security logs: {} \n", retcount).as_str());
        count = count + retcount;
        retcount = 0;

        println!("calculating total number of setup logs available ...");
        retcount = self.probe_hsetup_logs();
        self.info_htot_logs.push_str(format!("setup logs: {} \n", retcount).as_str());
        count = count + retcount;
        retcount = 0;

        println!("calculating total number of system logs available ...");
        retcount = self.probe_hsystem_logs();
        self.info_htot_logs.push_str(format!("system logs: {} \n", retcount).as_str());
        count = count + retcount;
        retcount = 0;

        println!("calculating total number of forwarded events logs available ...");
        retcount = self.probe_hfe_logs();
        self.info_htot_logs.push_str(format!("forwarded events logs: {} \n", retcount).as_str());
        count = count + retcount;
        retcount = 0;

        //self.info_htot_logs.pu = format!("total number of events under Windows logs... this may take some time... please wait... \n").cyan().bold().to_string();
        self.info_htot_logs.push_str(format!("total logs: {} \n", count).as_str());
    
        //let fp = PathBuf::from(format!("C:\\Windows\\System32\\winevt\\Logs\\System.evtx"));
        //let mut parser = EvtxParser::from_path(fp).unwrap();
    }
}

