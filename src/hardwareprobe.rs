// 0.15

use num_cpus;
use colored::Colorize;
use sysinfo::{CpuExt, System, SystemExt};

use local_ip_address::list_afinet_netifas;



pub struct hardwareprobe {
    pub cpu_info: String,
    pub nw_info: String,
}

impl hardwareprobe {

    pub fn get_cpu_logical_cores(&mut self) {

        let sys = System::new_all();

        self.cpu_info = format!("hardware probe - cpu ...\n").cyan().bold().to_string();
        self.cpu_info.push_str(format!("number of logical cores: {}\n", num_cpus::get()).as_str());

        for cpu in sys.cpus() {
            self.cpu_info.push_str(format!("{}% - {} Mhz - {} - {} - {}\n", cpu.cpu_usage(), cpu.frequency(), cpu.vendor_id(), cpu.name(), cpu.brand()).as_str());
        }
    }
}

impl hardwareprobe {

    pub fn get_network_interfaces(&mut self) {

        self.nw_info = format!("{}" , format!("hardware probe - network interfaces ...\n").cyan().bold());

        let network_interfaces = list_afinet_netifas().unwrap();

        for (name, ip) in network_interfaces.iter() {
            
            if ip.is_ipv4() {
                self.nw_info.push_str(format!("{}: IPv4 - {:?}\n", name, ip).as_str());
            } else if ip.is_ipv6() {
                self.nw_info.push_str(format!("{}: IPv6 - {:?}\n", name, ip).as_str());
            }
        }
    }
}