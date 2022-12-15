// 0.16.1 - windowsevents

use sysinfo::{Pid, ProcessExt, System, SystemExt, UserExt};
use colored::Colorize;
use std::{env, fs};

pub struct windowseventsprobe {
    pub appication_logs: String,
    pub security_logs: String,
    pub setup_logs: String,
    pub system_logs: String,
    pub forwardedevents_logs: String,
}


impl windowseventsprobe {

    pub fn windows_events_application_logs(&mut self) {

        self.appication_logs = format!("probing application logs... \n").cyan().bold().to_string();
    }

}
