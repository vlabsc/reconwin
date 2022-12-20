// 0.16.2 - naming ...

use num_cpus;
use colored::Colorize;
use sysinfo::{CpuExt, System, SystemExt};

use local_ip_address::list_afinet_netifas;



pub struct hardwareprobe {
    pub info_cpu: String,
    //pub info_nw: String,
}

impl Default for hardwareprobe {
    fn default() -> Self {
        hardwareprobe {
            info_cpu: String::from("cpu info"),
            //info_nw: String::from("nw info")
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
/*
impl hardwareprobe {

    pub fn get_network_interfaces(&mut self) {

        self.info_nw = format!("{}" , format!("hardware probe - network interfaces ...\n").cyan().bold());

        let network_interfaces = list_afinet_netifas().unwrap();

        for (name, ip) in network_interfaces.iter() {
            
            if ip.is_ipv4() {
                self.info_nw.push_str(format!("{}: IPv4 - {:?}\n", name, ip).as_str());
            } else if ip.is_ipv6() {
                self.info_nw.push_str(format!("{}: IPv6 - {:?}\n", name, ip).as_str());
            }
        }
    }
}
*/