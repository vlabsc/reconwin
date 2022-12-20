// 0.16.2 - naming ...

use num_cpus;
use colored::Colorize;
use sysinfo::{CpuExt, System, SystemExt};

use local_ip_address::list_afinet_netifas;

use ipconfig;


pub struct networkprobe {
    pub info_get_network_interfaces: String,
    pub info_get_dns_for_network_interfaces: String,
}

impl Default for networkprobe {
    fn default() -> Self {
        networkprobe {
            info_get_network_interfaces: String::from("nw info"),
            info_get_dns_for_network_interfaces: String::from("dns info")
        }
     }
}




impl networkprobe {

    pub fn probe_get_network_interfaces(&mut self) {

        self.info_get_network_interfaces = format!("{}" , format!("network probe - network interfaces ...\n").cyan().bold());

        let network_interfaces = list_afinet_netifas().unwrap();

        for (name, ip) in network_interfaces.iter() {
            
            if ip.is_ipv4() {
                self.info_get_network_interfaces.push_str(format!("{}: IPv4 - {:?}\n", name, ip).as_str());
            } else if ip.is_ipv6() {
                self.info_get_network_interfaces.push_str(format!("{}: IPv6 - {:?}\n", name, ip).as_str());
            }
        }
    }
}

impl networkprobe {

    pub fn probe_get_dns_for_network_interfaces(&mut self) {

        let mut adapters = ipconfig::get_adapters().unwrap();
        adapters.sort_by(|ip1, ip2| ip1.ipv4_metric().cmp(&ip2.ipv4_metric()));

        for adapter in adapters {

            //let new_ipaddress = adapter.ip_addresses();

            for new_ipaddress in adapter.ip_addresses() {
                if new_ipaddress.is_ipv4() {
                    println!(
                        "DNS: {:?} -> For IP: {:?}, Type: {}.",
                        adapter.dns_servers(),
                        new_ipaddress,
                        adapter.friendly_name()
                    )
                }
            }

                

                /*
                println!(
                    "DNS: {:?} -> For IP {:?}, Type {}.",
                    adapter.dns_servers(),
                    adapter.ip_addresses(),
                    adapter.IPv4(),
                    adapter.friendly_name()
                )
                */
        }
        
    }
}


