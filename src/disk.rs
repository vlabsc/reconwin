// 0.17.1 - windows events implementation

use sysinfo::{DiskExt, System, SystemExt};
use colored::Colorize;
//use std::time::Instant;

pub struct diskprobe {
    pub disk_info: String,
}


impl diskprobe {
    
    pub fn windows_disk_info(&mut self) {

        //let start = Instant::now();

        let sys = System::new_all();

        self.disk_info = format!("disks ... \n").cyan().bold().to_string();
        for disk in sys.disks() {
            self.disk_info.push_str(format!("label: {}, fs: ", disk.name().to_string_lossy()).as_str());

            for x in disk.file_system() {
                self.disk_info.push_str(format!("{}", *x as char).as_str());
            }

            self.disk_info.push_str(format!(", mount: {}", disk.mount_point().display()).as_str());
            self.disk_info.push_str(format!(", removable: ").as_str());
            if disk.is_removable()  {
                self.disk_info.push_str(format!(" yes").as_str());
            } else {
                self.disk_info.push_str(format!(" no" ).as_str());
            }

            self.disk_info.push_str(
                format!(
                ", total capacity: {} GB, free-space: {} GB", disk.total_space() / (1024 * 1024 * 1024), disk.available_space() / (1024 * 1024 * 1024)
                ).as_str()
            );
            self.disk_info.push_str(format!("\n").as_str());
        }

        //println!("total time elapsed: {:?}", start.elapsed());
        //println!("total time elapsed: {a}", a=5);

    }
}
