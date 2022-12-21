// 0.17.1 - windows events implementation

use num_cpus;
use colored::Colorize;
use sysinfo::{CpuExt, System, SystemExt};

use local_ip_address::list_afinet_netifas;



pub struct hardwareprobe {
    pub info_cpu: String,
}

impl Default for hardwareprobe {
    fn default() -> Self {
        hardwareprobe {
            info_cpu: String::from("cpu info"),
        }
     }
}


impl hardwareprobe {

    pub fn get_cpu_logical_cores(&mut self) {

        let sys = System::new_all();

        self.info_cpu = format!("hardware probe - cpu ...\n").cyan().bold().to_string();
        self.info_cpu.push_str(format!("number of logical cores: {}\n", num_cpus::get()).as_str());

        for cpu in sys.cpus() {
            self.info_cpu.push_str(format!("{}% - {} Mhz - {} - {} - {}\n", cpu.cpu_usage(), cpu.frequency(), cpu.vendor_id(), cpu.name(), cpu.brand()).as_str());
        }
    }
}
