// 0.17 - new version

use sysinfo::{System, SystemExt};
use colored::Colorize;

pub struct memprobe {
    pub mem_info: String,
}

impl memprobe {

    pub fn windows_mem_info(&mut self) {

        let sys = System::new_all();

        self.mem_info = format!("memory ... \n").cyan().bold().to_string();

        self.mem_info.push_str(format!("total: {} GB\n", sys.total_memory() / (1024 * 1024 * 1024)).as_str());
        self.mem_info.push_str(format!("free : {} GB\n", sys.free_memory() / (1024 * 1024 * 1024)).as_str());
        self.mem_info.push_str(format!("used : {} GB\n", sys.used_memory() / (1024 * 1024 * 1024)).as_str());
        self.mem_info.push_str(format!("total swap: {} GB\n", sys.total_swap() / (1024 * 1024 * 1024)).as_str());
        self.mem_info.push_str(format!("used swap: {} GB\n", sys.used_swap() / (1024 * 1024 * 1024)).as_str());
    }
}