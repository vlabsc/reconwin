// 0.17.1 - windows events implementation


use colored::Colorize;
use std::io;
use std::io::*;
use std::process;

use battery;


extern crate winreg;

mod probe_hardware;
mod disk;
mod mem;
mod probe_windows;
mod probe_windows_files;
mod probe_windows_events;
mod probe_network;

mod tester;

fn main() {

    hello_world();

    
    //tester::tester_main();
    //test();
    //return;
    

    //let ret = command_main_loop();
    //let ret = command_network_loop();
    //let mut ret = windows_events_setup_command_execute();
    let mut returne = windows_events_command_loop();

    print_end_of_program();
}




fn test () {
    

}
fn hello_world() {

    println!("");
    println!("------------------------------------------------------------------");
    println!("{}" , format!("recon-windows v0.16 beta").bold().green());
    println!("");

}


fn command_main_loop() -> i32 {

    let mut command: String = String::new();
    let stdin = io::stdin();

    
    'commandmain: loop {
        println!("");
        print!("rewin> ");

        let _=stdout().flush();
        stdin.read_line(&mut command);
        let _=stdout().flush();
        command.make_ascii_lowercase();

        if let Some('\n') = command.chars().next_back() {
            command.pop();
        }
        if let Some('\r') = command.chars().next_back() {
            command.pop();
        }

        match command.as_str() {
            "quit" | "exit" | "q" | "e"  | ".." => { 
                return 1;
                //break 'commandmain;
             },
            "hardware" | "hw" => {
                command.clear();
                let ret = command_hardware_loop();
                if ret == 0 {
                    return 0;
                }
            },
            "win" | "windows" => {
                command.clear();
                let ret = command_windows_loop();
                if ret == 0 {
                    return 0;
                }
            },
            "network" | "nw" => {
                command.clear();
                let ret = command_network_loop();
                if ret == 0 {
                    return 0;
                }
            },
            "cls" | "clear" => {
                clrscr();
            },
            "v" | "version" | "ver" => {
                command_main_version_execute();
            }
            "?" | "help" | "h" => {
                command_main_help_execute();
            },
            _ => {
                if command.is_empty() {
                    // do nothing
                }
                else {
                    println!("{command} command not found.");
                }
            },
        }
        //println!("command is: {command}");
        command.clear();
    }
}

fn command_main_help_execute() {
        println!("basic commands");
        println!("..                   - go back to previous menu.");
        println!("version / ver / v    - get the version of reconwin program");
        println!("h / help / ?         - get available commands within this section");
        println!("quit / exit / q / e  - quit or exit reconwin program");
        println!("cls / clear          - clears the screen.");
        println!("credits              - credits to cargo packages used, developers and others who directly or in-directly supported.");
        println!("\nmodules");
        println!("hardware / hw        - probe hardware related information");
        println!("windows / win        - probe windows related informationo (directories, temp files; etc)");
        println!("network / nw         - . TODO . probe network related informationo (adapters, IP address, DNS servers; etc)");
        println!("hunt                 - . TODO . the threat hunting module. specific to hunt threats and indicators within processes, registry, filesystem and across windows.");
    }
    
fn command_main_version_execute() {
    println!("{}" , format!("recon-windows v0.17.1 beta"));
}


fn command_hardware_loop() -> i32 {

    let mut command: String = String::new();
    let stdin = io::stdin();
    
    'commandmain: loop {
        println!("");
        print!("rewin> hardware> ");

        let _=stdout().flush();
        stdin.read_line(&mut command);
        command.make_ascii_lowercase();

        if let Some('\n') = command.chars().next_back() {
            command.pop();
        }
        if let Some('\r') = command.chars().next_back() {
            command.pop();
        }

        match command.as_str() {
            "quit" | "exit" | "q" | "e" =>
            {
                return 0;
            },
            "v" | "version" | "ver" => {
                command_main_version_execute();
            }
            "?" | "help" | "h" => {
                command_hardware_help();
            },
            "cpu" => {
                command_hardware_cpu_execute();
            },
            "nis" => {
                command_network_nis_execute();
            },    
            "disks" => {
                command_windows_disks_execute();
            },
            "mem" => {
                command_windows_mem_execute();
            },
            "all" => {
                command_hardware_all_execute();
            },
            "cls" | "clear" => {
                clrscr();
            },
            ".." => {
                return 55;
            },
            _ => {
                if command.is_empty() {
                    // do nothing
                }
                else {
                    println!("{command} command not found.");
                }
            },
        }
        command.clear();
    }
}

fn command_hardware_help() {

    println!("basic commands");
    println!("..                   - go back to previous menu.");
    println!("version / ver / v    - get the version of reconwin program");
    println!("h / help / ?         - get available commands within this section");
    println!("quit / exit / q / e  - quit or exit reconwin program");
    println!("cls / clear          - clears the screen.");
    println!("credits              - credits to cargo packages used, developers and others who directly or in-directly supported.");
    println!("\nhardware module - specific commands");
    println!("cpu       - probe cpu information (frequency, number of cores; etc)");
    println!("nis       - probe network interfaces information (IPv6 IP address, IPv4 IP address; etc)");
    println!("mem       - probe memory related information (total memory, available free, utilized)");
    println!("disks     - list available disks within windows system.");
    println!("battery   - . TODO . probe battery related information.");
    println!("all       - probe all info - cpu, network and memory.");
}


fn command_hardware_cpu_execute() -> i32 {

    let mut hardware = probe_hardware::hardwareprobe {
        info_cpu: String::from("cpu info"),
        //info_nw: String::from("nw info")
    };

    hardware.get_cpu_logical_cores();

    print!("{}", hardware.info_cpu);
    return 100;
}


fn command_hardware_all_execute() {
    
    let mut hardware = probe_hardware::hardwareprobe {
        info_cpu: String::from("cpu info"),
        //info_nw: String::from("nw info")
    };

    let mut network = probe_network::networkprobe {
        info_get_network_interfaces: String::from("nw info"),
        info_get_dns_for_network_interfaces: String::from("dns info")
    };

    let mut disk = disk::diskprobe {
        disk_info: String::from("disk info"),
    };

    let mut mem = mem::memprobe {
        mem_info: String::from("mem info"),
    };

    hardware.get_cpu_logical_cores();
    //hardware.get_network_interfaces();
    network.probe_get_network_interfaces();

    mem.windows_mem_info();
    disk.windows_disk_info();

    println!("{}", hardware.info_cpu);
    //println!("{}", hardware.info_nw);
    println!("{}", network.info_get_network_interfaces);
    println!("{}", mem.mem_info);
    print!("{}", disk.disk_info);    
}

fn command_network_loop() -> i32 {

    let mut command: String = String::new();
    let stdin = io::stdin();
    
    'commandmain: loop {
        println!("");
        print!("rewin> network> ");

        let _=stdout().flush();
        stdin.read_line(&mut command);
        command.make_ascii_lowercase();

        if let Some('\n') = command.chars().next_back() {
            command.pop();
        }
        if let Some('\r') = command.chars().next_back() {
            command.pop();
        }

        match command.as_str() {
            "quit" | "exit" | "q" | "e" =>
            {
                return 0;
            },
            "v" | "version" | "ver" => {
                command_main_version_execute();
            }
            "?" | "help" | "h" => {
                command_network_help();
            },
            "dns" => {
                // do nothing
                command_network_dns_execute();
            },
            "nis" => {
                command_network_nis_execute();
            },
            "cls" | "clear" => {
                clrscr();
            },
            ".." => {
                return 55;
            },
            _ => {
                if command.is_empty() {
                    // do nothing
                }
                else {
                    println!("{command} command not found.");
                }
            },
        }
        command.clear();
    }
}


fn command_network_help() {

    println!("basic commands");
    println!("..                   - go back to previous menu.");
    println!("version / ver / v    - get the version of reconwin program");
    println!("h / help / ?         - get available commands within this section");
    println!("quit / exit / q / e  - quit or exit reconwin program");
    println!("cls / clear          - clears the screen.");
    println!("credits              - credits to cargo packages used, developers and others who directly or in-directly supported.");
    println!("\nnetwork module - specific commands");
    println!("nis     - probe network interfaces information (IPv6 IP address, IPv4 IP address; etc)");
    println!("dns     - probe assigned DNS servers for each adapter.");
}

fn command_network_nis_execute() -> i32 {

    let mut network = probe_network::networkprobe {
        info_get_network_interfaces: String::from("nw info"),
        info_get_dns_for_network_interfaces: String::from("dns info")
    };

    /*
    let mut hardware = probe_hardware::hardwareprobe {
        info_cpu: String::from("cpu info"),
        info_nw: String::from("nw info")
    };
    */
    network.probe_get_network_interfaces();
    print!("{}", network.info_get_network_interfaces);

    //hardware.get_network_interfaces();
    //print!("{}", hardware.info_nw);
    return 100;
}

fn command_network_dns_execute() -> i32 {

    let mut network = probe_network::networkprobe {
        info_get_network_interfaces: String::from("nw info"),
        info_get_dns_for_network_interfaces: String::from("dns info")
    };

    /*
    let mut hardware = probe_hardware::hardwareprobe {
        info_cpu: String::from("cpu info"),
        info_nw: String::from("nw info")
    };
    */
    network.probe_get_dns_for_network_interfaces();
    //print!("{}", network.info_get_network_interfaces);

    //hardware.get_network_interfaces();
    //print!("{}", hardware.info_nw);
    return 100;
}


fn command_windows_loop() -> i32 {

    let mut command: String = String::new();
    let stdin = io::stdin();
    
    'commandmain: loop {
        println!("");
        print!("rewin> windows> ");

        let _=stdout().flush();
        stdin.read_line(&mut command);
        command.make_ascii_lowercase();

        if let Some('\n') = command.chars().next_back() {
            command.pop();
        }
        if let Some('\r') = command.chars().next_back() {
            command.pop();
        }

        match command.as_str() {
            "quit" | "exit" | "q" | "e" =>
            {
                return 0;
            },
            "v" | "version" | "ver" => {
                command_main_version_execute();
            }
            "?" | "help" | "h" => {
                command_windows_help();
            },
            "os" => {
                command_windows_os_execute();
            },
            "users" => {
                command_windows_users_execute();
            },
            "iapps" => {
                command_windows_iapps_execute();
            },
            "sapps" => {
                command_windows_sapps_execute();
            },
            "pwd" => {
                command_windows_pwd_execute();
            },
            "cb" => {
                command_windows_cb_execute();
            },    
            "disks" => {
                command_windows_disks_execute();
            },
            "mem" => {
                command_windows_mem_execute();
            },
            "ps" => {
                command_windows_get_process_list_execute();
            },
            "ud" => {
                command_windows_files_ud_execute();
            },
            "time" => {
                command_windows_datetime_execute();
            },
            "all" => {
                command_windows_all_execute();
            },
            "process" => {
                let ret = windows_process_command_loop();
                if ret == 0 {
                    return 0;
                }
            },
            "isadmin" => {
                windows_isadmin_command_execute();
            },
            "files" => {
                let ret = command_windows_files_loop();
                if ret == 0 {
                    return 0;
                }
            },
            "events" => {
                let ret = windows_events_command_loop();
                if ret == 0 {
                    return 0;
                }
            },
            ".." => {
                return 55;
            }
            "cls" | "clear" => {
                clrscr();
            },
            _ => {
                if command.is_empty() {
                    // do nothing
                }
                else {
                    println!("{command} command not found.");
                }
            },
        }
        command.clear();
    }
}



fn command_windows_help() {

    println!("basic commands");
    println!("..                   - go back to previous menu.");
    println!("version / ver / v    - get the version of reconwin program");
    println!("h / help / ?         - get available commands within this section");
    println!("quit / exit / q / e  - quit or exit reconwin program");
    println!("cls / clear          - clears the screen.");
    println!("credits              - credits to cargo packages used, developers and others who directly or in-directly supported.");
    println!("\nwindows module - specific commands");
    println!("os                   - probe operating system related details.");
    println!("ud                   - list windows users directories.");
    println!("cb                   - list clipboard content.");
    println!("ps                   - list running processes within windows system.");
    println!("pwd                  - display the present working directory.");
    println!("mem                  - probe memory related information (total memory, available free, utilized)");
    println!("time                 - print the current windows time.");
    println!("users                - probe windows account details. list of users within windows system.");
    println!("iapps                - list of installed programs within the windows system. probes through registry.");
    println!("sapps                - list of programs configured to run at startup. probes through registry.");
    println!("isadmin              - is the current context of the user is admin or not.");
    println!("disks                - list available disks within windows system.");
    println!("all                  - probe all info");

    println!("\nwindows module - sub modules");
    println!("files       - probe files, directories, config paths, temp folders and others within windows fs.");
    println!("events      - probe windows events. ");
    println!("reg         - . TODO. probe windows registry. TODO . ");
    println!("stasks      - . TODO. probe scheduled tasks. TODO . ");
    println!("services    - . TODO. probe windows services. TODO . ");
    println!("process     - . TODO. probe windows processes. TODO . ");
}


fn command_windows_os_execute() {

    let mut windows = probe_windows::windowsprobe {
        info_windows_osdetails: String::from("mem info"),
        info_windows_users: String::from("users info"),
        info_windows_installed_applications: String::from("applications info"),
        info_windows_startup_applications: String::from("registry startup applications info"),
        info_windows_get_clipboard: String::from("clipboard info"),
        info_windows_get_process_list: String::from("process info"),
        info_windows_datetime: String::from("windows date time info"),
        info_windows_pwd:String::from("present working directory"),
        info_windows_isadmin:String::from("not probed"),
    };

    windows.probe_windows_osdetails();
    print!("{}", windows.info_windows_osdetails);
}


fn command_windows_users_execute() {

    let mut windows = probe_windows::windowsprobe {
        info_windows_osdetails: String::from("mem info"),
        info_windows_users: String::from("users info"),
        info_windows_installed_applications: String::from("applications info"),
        info_windows_startup_applications: String::from("registry startup applications info"),
        info_windows_get_clipboard: String::from("clipboard info"),
        info_windows_get_process_list: String::from("process info"),
        info_windows_datetime: String::from("windows date time info"),
        info_windows_pwd:String::from("present working directory"),
        info_windows_isadmin:String::from("not probed"),
    };

    windows.probe_windows_users();
    print!("{}", windows.info_windows_users);
}

fn command_windows_get_process_list_execute() {

    let mut windows = probe_windows::windowsprobe {
        info_windows_osdetails: String::from("mem info"),
        info_windows_users: String::from("users info"),
        info_windows_installed_applications: String::from("applications info"),
        info_windows_startup_applications: String::from("registry startup applications info"),
        info_windows_get_clipboard: String::from("clipboard info"),
        info_windows_get_process_list: String::from("process info"),
        info_windows_datetime: String::from("windows date time info"),
        info_windows_pwd:String::from("present working directory"),
        info_windows_isadmin:String::from("not probed"),
    };

    windows.probe_windows_get_process_list();
    print!("{}", windows.info_windows_get_process_list);
}



fn command_windows_cb_execute() {

    let mut windows = probe_windows::windowsprobe {
        info_windows_osdetails: String::from("mem info"),
        info_windows_users: String::from("users info"),
        info_windows_installed_applications: String::from("applications info"),
        info_windows_startup_applications: String::from("registry startup applications info"),
        info_windows_get_clipboard: String::from("clipboard info"),
        info_windows_get_process_list: String::from("process info"),
        info_windows_datetime: String::from("windows date time info"),
        info_windows_pwd:String::from("present working directory"),
        info_windows_isadmin:String::from("not probed"),
    };

    windows.probe_windows_get_clipboard();
    print!("{}", windows.info_windows_get_clipboard);
}


fn command_windows_iapps_execute() {

    let mut windows = probe_windows::windowsprobe {
        info_windows_osdetails: String::from("mem info"),
        info_windows_users: String::from("users info"),
        info_windows_installed_applications: String::from("applications info"),
        info_windows_startup_applications: String::from("registry startup applications info"),
        info_windows_get_clipboard: String::from("clipboard info"),
        info_windows_get_process_list: String::from("process info"),
        info_windows_datetime: String::from("windows date time info"),
        info_windows_pwd:String::from("present working directory"),
        info_windows_isadmin:String::from("not probed"),
    };

    windows.probe_windows_installed_applications();
    print!("{}", windows.info_windows_installed_applications);
}

fn command_windows_sapps_execute() {

    let mut windows = probe_windows::windowsprobe {
        info_windows_osdetails: String::from("mem info"),
        info_windows_users: String::from("users info"),
        info_windows_installed_applications: String::from("applications info"),
        info_windows_startup_applications: String::from("registry startup applications info"),
        info_windows_get_clipboard: String::from("clipboard info"),
        info_windows_get_process_list: String::from("process info"),
        info_windows_datetime: String::from("windows date time info"),
        info_windows_pwd:String::from("present working directory"),
        info_windows_isadmin:String::from("not probed"),
    };

    windows.probe_windows_startup_applications();
    print!("{}", windows.info_windows_startup_applications);
}

fn command_windows_pwd_execute() {

    let mut windows = probe_windows::windowsprobe {
        info_windows_osdetails: String::from("mem info"),
        info_windows_users: String::from("users info"),
        info_windows_installed_applications: String::from("applications info"),
        info_windows_startup_applications: String::from("registry startup applications info"),
        info_windows_get_clipboard: String::from("clipboard info"),
        info_windows_get_process_list: String::from("process info"),
        info_windows_datetime: String::from("windows date time info"),
        info_windows_pwd:String::from("present working directory"),
        info_windows_isadmin:String::from("not probed"),
    };

    windows.probe_windows_pwd();
    print!("{}", windows.info_windows_pwd);
}

fn command_windows_datetime_execute() {

    let mut windows = probe_windows::windowsprobe {
        info_windows_osdetails: String::from("mem info"),
        info_windows_users: String::from("users info"),
        info_windows_installed_applications: String::from("applications info"),
        info_windows_startup_applications: String::from("registry startup applications info"),
        info_windows_get_clipboard: String::from("clipboard info"),
        info_windows_get_process_list: String::from("process info"),
        info_windows_datetime: String::from("windows date time info"),
        info_windows_pwd:String::from("present working directory"),
        info_windows_isadmin:String::from("not probed"),
    };

    windows.probe_windows_datetime();
    print!("{}", windows.info_windows_datetime);

    //probe_windows::windows_test();

}

fn windows_isadmin_command_execute() {

    let mut windows = probe_windows::windowsprobe {
        info_windows_osdetails: String::from("mem info"),
        info_windows_users: String::from("users info"),
        info_windows_installed_applications: String::from("applications info"),
        info_windows_startup_applications: String::from("registry startup applications info"),
        info_windows_get_clipboard: String::from("clipboard info"),
        info_windows_get_process_list: String::from("process info"),
        info_windows_datetime: String::from("windows date time info"),
        info_windows_pwd:String::from("present working directory"),
        info_windows_isadmin:String::from("not probed"),
    };

    windows.probe_windows_isadmin();
    print!("{}", windows.info_windows_isadmin);
}

fn command_windows_disks_execute() {
    let mut disk = disk::diskprobe {
        disk_info: String::from("disk info"),
    };

    disk.windows_disk_info();
    print!("{}", disk.disk_info);
}

fn command_windows_mem_execute() {
    let mut mem = mem::memprobe {
        mem_info: String::from("mem info"),
    };

    mem.windows_mem_info();
    print!("{}", mem.mem_info);
}


fn command_windows_all_execute() {

    let mut disk = disk::diskprobe {
        disk_info: String::from("disk info"),
    };

    let mut mem = mem::memprobe {
        mem_info: String::from("mem info"),
    };

    let mut windows = probe_windows::windowsprobe {
        info_windows_osdetails: String::from("mem info"),
        info_windows_users: String::from("users info"),
        info_windows_installed_applications: String::from("applications info"),
        info_windows_startup_applications: String::from("registry startup applications info"),
        info_windows_get_clipboard: String::from("clipboard info"),
        info_windows_get_process_list: String::from("process info"),
        info_windows_datetime: String::from("windows date time info"),
        info_windows_pwd:String::from("present working directory"),
        info_windows_isadmin:String::from("not probed"),
    };


    disk.windows_disk_info();
    println!("{}", disk.disk_info);

    mem.windows_mem_info();
    println!("{}", mem.mem_info);

    windows.probe_windows_osdetails();
    println!("{}", windows.info_windows_osdetails);

    windows.probe_windows_users();
    println!("{}", windows.info_windows_users);

    windows.probe_windows_installed_applications();
    println!("{}", windows.info_windows_installed_applications);

    windows.probe_windows_startup_applications();
    println!("{}", windows.info_windows_startup_applications);

    windows.probe_windows_get_clipboard();
    println!("{}", windows.info_windows_get_clipboard);

    windows.probe_windows_get_process_list();
    print!("{}", windows.info_windows_get_process_list);
}



fn command_windows_files_loop() -> i32 {

    let mut command: String = String::new();
    let stdin = io::stdin();

    'commandmain: loop {
        println!("");
        print!("rewin> windows> files> ");

        let _=stdout().flush();
        stdin.read_line(&mut command);
        command.make_ascii_lowercase();

        if let Some('\n') = command.chars().next_back() {
            command.pop();
        }
        if let Some('\r') = command.chars().next_back() {
            command.pop();
        }

        match command.as_str() {
            "quit" | "exit" | "q" | "e" =>
            {
                return 0;
            },
            "v" | "version" | "ver" => {
                command_main_version_execute();
            }
            "?" | "help" | "h" => {
                command_windows_files_help();
            },
            "ud" => {
                command_windows_files_ud_execute();
            },
            "appdata" => {
                command_windows_files_appdata_execute();
            },
            "local" => {
                command_windows_files_local_execute();
            },
            "localtemp" => {
                command_windows_files_localtemp_execute();
            },
            "temp" => {
                command_windows_files_temp_execute();
            },
            "desktop" => {
                command_windows_files_desktop_execute();
            },
            "pfiles" => {
                command_windows_files_pfiles_execute();
            },
            "pfiles86" => {
                command_windows_files_pfiles86_execute();
            },
            "windows" => {
                command_windows_files_windows_execute();
            },
            "sys32" => {
                command_windows_files_windows_system32_execute();
            },
            "all" => {
                //windows_all_command_execute();
            },
            "files" => {
                //command_windows_all_execute();
            },
            "cls" | "clear" => {
                clrscr();
            },
            ".." => {
                return 55;
            },
            _ => {
                if command.is_empty() {
                    // do nothing
                }
                else {
                    println!("{command} command not found.");
                }
            },
        }
        command.clear();
    }
}

fn command_windows_files_help() {

    println!("basic commands");
    println!("..                   - go back to previous menu.");
    println!("version / ver / v    - get the version of reconwin program");
    println!("h / help / ?         - get available commands within this section");
    println!("quit / exit / q / e  - quit or exit reconwin program");
    println!("cls / clear          - clears the screen.");
    println!("credits              - credits to cargo packages used, developers and others who directly or in-directly supported.");
    println!("\nwindows files - specific commands");
    println!("ud             - list windows users directories.");
    println!("temp           - list files within windows temp directory.");
    println!("test           - test.");        
    println!("sys32          - list files within C:\\Windows\\System32\\.");
    println!("local          - list files within users %appdata%\\Local directory.");
    println!("pfiles         - list files within C:\\Program Files\\.");
    println!("appdata        - list files within users %appdata% directory.");
    println!("desktop        - list files within windows desktop directory.");
    println!("windows        - list files within C:\\Windows\\.");
    println!("pfiles86       - list files within C:\\Program Files(x86)\\.");
    println!("localtemp      - list files within users %appdata%\\Local\\Temp directory.");
}

fn command_windows_files_ud_execute() {

    let mut windowsfiles = probe_windows_files::probewindowsfiles {
        info_windows_files_list_user_directories: String::from("user directories info"),
        info_windows_files_list_appdata_roaming_directory_files: String::from("%AppData% directory info"),
        info_windows_files_list_appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        info_windows_files_list_appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        info_windows_files_list_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        info_windows_files_list_desktop_directory_files: String::from("\\desktop\\ info"),
        info_windows_files_list_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        info_windows_files_list_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        info_windows_files_list_windows_directory_files: String::from("C:\\Winidows\\ info"),
        info_windows_files_list_system32_directory_files: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.probe_windows_files_list_user_directories();
    print!("{}", windowsfiles.info_windows_files_list_user_directories);
}


fn command_windows_files_pfiles_execute() {

    let mut windowsfiles = probe_windows_files::probewindowsfiles {
        info_windows_files_list_user_directories: String::from("user directories info"),
        info_windows_files_list_appdata_roaming_directory_files: String::from("%AppData% directory info"),
        info_windows_files_list_appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        info_windows_files_list_appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        info_windows_files_list_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        info_windows_files_list_desktop_directory_files: String::from("\\desktop\\ info"),
        info_windows_files_list_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        info_windows_files_list_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        info_windows_files_list_windows_directory_files: String::from("C:\\Winidows\\ info"),
        info_windows_files_list_system32_directory_files: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.probe_windows_files_list_pfiles_directory_files();
    print!("{}", windowsfiles.info_windows_files_list_pfiles_directory_files);
}

fn command_windows_files_pfiles86_execute() {

    let mut windowsfiles = probe_windows_files::probewindowsfiles {
        info_windows_files_list_user_directories: String::from("user directories info"),
        info_windows_files_list_appdata_roaming_directory_files: String::from("%AppData% directory info"),
        info_windows_files_list_appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        info_windows_files_list_appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        info_windows_files_list_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        info_windows_files_list_desktop_directory_files: String::from("\\desktop\\ info"),
        info_windows_files_list_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        info_windows_files_list_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        info_windows_files_list_windows_directory_files: String::from("C:\\Winidows\\ info"),
        info_windows_files_list_system32_directory_files: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.probe_windows_files_list_pfiles86_directory_files();
    print!("{}", windowsfiles.info_windows_files_list_pfiles86_directory_files);
}

fn command_windows_files_sys32_execute() {

    let mut windowsfiles = probe_windows_files::probewindowsfiles {
        info_windows_files_list_user_directories: String::from("user directories info"),
        info_windows_files_list_appdata_roaming_directory_files: String::from("%AppData% directory info"),
        info_windows_files_list_appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        info_windows_files_list_appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        info_windows_files_list_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        info_windows_files_list_desktop_directory_files: String::from("\\desktop\\ info"),
        info_windows_files_list_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        info_windows_files_list_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        info_windows_files_list_windows_directory_files: String::from("C:\\Winidows\\ info"),
        info_windows_files_list_system32_directory_files: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.probe_windows_files_list_system32_directory_files();
    print!("{}", windowsfiles.info_windows_files_list_system32_directory_files);
}

fn command_windows_files_windows_execute() {

    let mut windowsfiles = probe_windows_files::probewindowsfiles {
        info_windows_files_list_user_directories: String::from("user directories info"),
        info_windows_files_list_appdata_roaming_directory_files: String::from("%AppData% directory info"),
        info_windows_files_list_appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        info_windows_files_list_appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        info_windows_files_list_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        info_windows_files_list_desktop_directory_files: String::from("\\desktop\\ info"),
        info_windows_files_list_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        info_windows_files_list_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        info_windows_files_list_windows_directory_files: String::from("C:\\Winidows\\ info"),
        info_windows_files_list_system32_directory_files: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.probe_windows_files_list_windows_directory_files();
    print!("{}", windowsfiles.info_windows_files_list_windows_directory_files);
}

fn command_windows_files_windows_system32_execute() {

    let mut windowsfiles = probe_windows_files::probewindowsfiles {
        info_windows_files_list_user_directories: String::from("user directories info"),
        info_windows_files_list_appdata_roaming_directory_files: String::from("%AppData% directory info"),
        info_windows_files_list_appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        info_windows_files_list_appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        info_windows_files_list_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        info_windows_files_list_desktop_directory_files: String::from("\\desktop\\ info"),
        info_windows_files_list_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        info_windows_files_list_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        info_windows_files_list_windows_directory_files: String::from("C:\\Winidows\\ info"),
        info_windows_files_list_system32_directory_files: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.probe_windows_files_list_system32_directory_files();
    print!("{}", windowsfiles.info_windows_files_list_system32_directory_files);
}

fn command_windows_files_appdata_execute() {

    let mut windowsfiles = probe_windows_files::probewindowsfiles {
        info_windows_files_list_user_directories: String::from("user directories info"),
        info_windows_files_list_appdata_roaming_directory_files: String::from("%AppData% directory info"),
        info_windows_files_list_appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        info_windows_files_list_appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        info_windows_files_list_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        info_windows_files_list_desktop_directory_files: String::from("\\desktop\\ info"),
        info_windows_files_list_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        info_windows_files_list_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        info_windows_files_list_windows_directory_files: String::from("C:\\Winidows\\ info"),
        info_windows_files_list_system32_directory_files: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.probe_windows_files_list_appdata_roaming_directory_files();
    print!("{}", windowsfiles.info_windows_files_list_appdata_roaming_directory_files);
}

fn command_windows_files_local_execute() {

    let mut windowsfiles = probe_windows_files::probewindowsfiles {
        info_windows_files_list_user_directories: String::from("user directories info"),
        info_windows_files_list_appdata_roaming_directory_files: String::from("%AppData% directory info"),
        info_windows_files_list_appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        info_windows_files_list_appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        info_windows_files_list_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        info_windows_files_list_desktop_directory_files: String::from("\\desktop\\ info"),
        info_windows_files_list_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        info_windows_files_list_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        info_windows_files_list_windows_directory_files: String::from("C:\\Winidows\\ info"),
        info_windows_files_list_system32_directory_files: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.probe_windows_files_list_appdata_local_directory_files();
    print!("{}", windowsfiles.info_windows_files_list_appdata_local_directory_files);
}


fn command_windows_files_localtemp_execute() {

    let mut windowsfiles = probe_windows_files::probewindowsfiles {
        info_windows_files_list_user_directories: String::from("user directories info"),
        info_windows_files_list_appdata_roaming_directory_files: String::from("%AppData% directory info"),
        info_windows_files_list_appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        info_windows_files_list_appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        info_windows_files_list_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        info_windows_files_list_desktop_directory_files: String::from("\\desktop\\ info"),
        info_windows_files_list_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        info_windows_files_list_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        info_windows_files_list_windows_directory_files: String::from("C:\\Winidows\\ info"),
        info_windows_files_list_system32_directory_files: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.probe_windows_files_list_appdata_local_temp_directory_files();
    print!("{}", windowsfiles.info_windows_files_list_appdata_local_temp_directory_files);
}

fn command_windows_files_temp_execute() {

    let mut windowsfiles = probe_windows_files::probewindowsfiles {
        info_windows_files_list_user_directories: String::from("user directories info"),
        info_windows_files_list_appdata_roaming_directory_files: String::from("%AppData% directory info"),
        info_windows_files_list_appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        info_windows_files_list_appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        info_windows_files_list_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        info_windows_files_list_desktop_directory_files: String::from("\\desktop\\ info"),
        info_windows_files_list_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        info_windows_files_list_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        info_windows_files_list_windows_directory_files: String::from("C:\\Winidows\\ info"),
        info_windows_files_list_system32_directory_files: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.probe_windows_files_list_temp_directory_files();
    print!("{}", windowsfiles.info_windows_files_list_temp_directory_files);
}

fn command_windows_files_desktop_execute() {

    let mut windowsfiles = probe_windows_files::probewindowsfiles {
        info_windows_files_list_user_directories: String::from("user directories info"),
        info_windows_files_list_appdata_roaming_directory_files: String::from("%AppData% directory info"),
        info_windows_files_list_appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        info_windows_files_list_appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        info_windows_files_list_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        info_windows_files_list_desktop_directory_files: String::from("\\desktop\\ info"),
        info_windows_files_list_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        info_windows_files_list_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        info_windows_files_list_windows_directory_files: String::from("C:\\Winidows\\ info"),
        info_windows_files_list_system32_directory_files: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.probe_windows_files_list_desktop_directory_files();
    print!("{}", windowsfiles.info_windows_files_list_desktop_directory_files);
}


fn windows_events_command_loop() -> i32 {

    let mut command: String = String::new();
    let stdin = io::stdin();

    'commandmain: loop {
        println!("");
        print!("rewin> windows> events> ");

        let _=stdout().flush();
        stdin.read_line(&mut command);
        command.make_ascii_lowercase();

        if let Some('\n') = command.chars().next_back() {
            command.pop();
        }
        if let Some('\r') = command.chars().next_back() {
            command.pop();
        }

        match command.as_str() {
            "quit" | "exit" | "q" | "e" =>
            {
                return 0;
            },
            "v" | "version" | "ver" => {
                command_main_version_execute();
            }
            "?" | "help" | "h" => {
                windows_events_command_help();
            },
            "cls" | "clear" => {
                clrscr();
            },
            "application" | "app" => {
                windows_events_application_command_execute();
            },
            "security" | "sec" => {
                windows_events_security_command_execute();
            },
            "setup" | "set" => {
                windows_events_setup_command_execute();
            },
            "system" | "sys" => {
                windows_events_system_command_execute();
            },
            "happ" => {
                windows_events_happ_command_execute();
            },
            "hsec" => {
                windows_events_hsec_command_execute();
            },
            "hset" => {
                windows_events_hset_command_execute();
            },
            "hsys" => {
                windows_events_hsys_command_execute();
            },
            "hfe" => {
                windows_events_hfe_command_execute();
            },
            "htot" => {
                windows_events_htot_command_execute();
            },
            ".." => {
                return 55;
            },
            _ => {
                if command.is_empty() {
                    // do nothing
                }
                else {
                    println!("{command} command not found.");
                }
            },
        }
        command.clear();
    }
}

fn windows_events_command_help() {

    println!("basic commands");
    println!("..                   - go back to previous menu.");
    println!("version / ver / v    - get the version of reconwin program");
    println!("h / help / ?         - get available commands within this section");
    println!("quit / exit / q / e  - quit or exit reconwin program");
    println!("cls / clear          - clears the screen.");
    println!("credits              - credits to cargo packages used, developers and others who directly or in-directly supported.");
    println!("\nwindows logs");
    println!("application / app      - list application logs.");
    println!("security / sec         - list security logs.");
    println!("setup / set            - list setup logs.");
    println!("system / sys           - list system logs.");
    println!("forwardedevents / fe   - list forwarded events logs.");
    println!("happ                   - how many application event records available.");
    println!("hsec                   - how many security event records available.");
    println!("hset                   - how many setup event records available.");
    println!("hsys                   - how many system event records available.");
    println!("htot                   - how many total events available under Windows Logs.");

}

fn windows_events_application_command_execute() {

    let mut windowsevents = probe_windows_events::windowseventsprobe {
        application_logs: String::from("application logs setup"),
        security_logs: String::from("security logs setup"),
        setup_logs: String::from("setup logs setup"),
        system_logs: String::from("system logs setup"),
        forwardedevents_logs: String::from("forwarded events logs setup"),
    };

    windowsevents.windows_events_application_logs();
    print!("{}", windowsevents.application_logs);
}

fn windows_events_security_command_execute() {

    let mut windowsevents = probe_windows_events::windowseventsprobe {
        application_logs: String::from("application logs setup"),
        security_logs: String::from("security logs setup"),
        setup_logs: String::from("setup logs setup"),
        system_logs: String::from("system logs setup"),
        forwardedevents_logs: String::from("forwarded events logs setup"),
    };

    windowsevents.windows_events_security_logs();
    print!("{}", windowsevents.security_logs);
}

fn windows_events_setup_command_execute() {

    let mut windowsevents = probe_windows_events::windowseventsprobe {
        application_logs: String::from("application logs setup"),
        security_logs: String::from("security logs setup"),
        setup_logs: String::from("setup logs setup"),
        system_logs: String::from("system logs setup"),
        forwardedevents_logs: String::from("forwarded events logs setup"),
    };

    windowsevents.windows_events_setup_logs();
    print!("{}", windowsevents.setup_logs);
}

fn windows_events_system_command_execute() {

    let mut windowsevents = probe_windows_events::windowseventsprobe {
        application_logs: String::from("application logs setup"),
        security_logs: String::from("security logs setup"),
        setup_logs: String::from("setup logs setup"),
        system_logs: String::from("system logs setup"),
        forwardedevents_logs: String::from("forwarded events logs setup"),
    };

    windowsevents.windows_events_system_logs();
    print!("{}", windowsevents.system_logs);
}

fn windows_events_happ_command_execute() {

    let mut windowseventsprobehowmany = probe_windows_events::windowseventsprobe_howmany {
        info_happlication_logs: String::from("number"),
        info_hsecurity_logs: String::from("number"),
        info_hsetup_logs: String::from("number"),
        info_hsystem_logs: String::from("number"),
        info_hfe_logs: String::from("number"),
        info_htot_logs: String::from("number"),
    };

    println!("calculating total number of application logs available ...");
    windowseventsprobehowmany.probe_happlication_logs();
    print!("{}", windowseventsprobehowmany.info_happlication_logs);
}

fn windows_events_hsec_command_execute() {

    let mut windowseventsprobehowmany = probe_windows_events::windowseventsprobe_howmany {
        info_happlication_logs: String::from("number"),
        info_hsecurity_logs: String::from("number"),
        info_hsetup_logs: String::from("number"),
        info_hsystem_logs: String::from("number"),
        info_hfe_logs: String::from("number"),
        info_htot_logs: String::from("number"),
    };

    println!("calculating total number of security logs available ...");
    windowseventsprobehowmany.probe_hsecurity_logs();
    print!("{}", windowseventsprobehowmany.info_hsecurity_logs);
}

fn windows_events_hset_command_execute() {

    let mut windowseventsprobehowmany = probe_windows_events::windowseventsprobe_howmany {
        info_happlication_logs: String::from("number"),
        info_hsecurity_logs: String::from("number"),
        info_hsetup_logs: String::from("number"),
        info_hsystem_logs: String::from("number"),
        info_hfe_logs: String::from("number"),
        info_htot_logs: String::from("number"),
    };

    println!("calculating total number of setup logs available ...");
    windowseventsprobehowmany.probe_hsetup_logs();
    print!("{}", windowseventsprobehowmany.info_hsetup_logs);
}


fn windows_events_hsys_command_execute() {

    let mut windowseventsprobehowmany = probe_windows_events::windowseventsprobe_howmany {
        info_happlication_logs: String::from("number"),
        info_hsecurity_logs: String::from("number"),
        info_hsetup_logs: String::from("number"),
        info_hsystem_logs: String::from("number"),
        info_hfe_logs: String::from("number"),
        info_htot_logs: String::from("number"),
    };

    println!("calculating total number of system logs available ...");
    windowseventsprobehowmany.probe_hsystem_logs();
    print!("{}", windowseventsprobehowmany.info_hsystem_logs);
}

fn windows_events_hfe_command_execute() {

    let mut windowseventsprobehowmany = probe_windows_events::windowseventsprobe_howmany {
        info_happlication_logs: String::from("number"),
        info_hsecurity_logs: String::from("number"),
        info_hsetup_logs: String::from("number"),
        info_hsystem_logs: String::from("number"),
        info_hfe_logs: String::from("number"),
        info_htot_logs: String::from("number"),
    };

    println!("calculating total number of forwarded events logs available ...");
    windowseventsprobehowmany.probe_hfe_logs();
    print!("{}", windowseventsprobehowmany.info_hfe_logs);
}

fn windows_events_htot_command_execute() {

    let mut windowseventsprobehowmany = probe_windows_events::windowseventsprobe_howmany {
        info_happlication_logs: String::from("number"),
        info_hsecurity_logs: String::from("number"),
        info_hsetup_logs: String::from("number"),
        info_hsystem_logs: String::from("number"),
        info_hfe_logs: String::from("number"),
        info_htot_logs: String::from("number"),
    };

    println!("total number of events under Windows logs... this may take some time... please wait...");
    windowseventsprobehowmany.probe_htot_logs();
    print!("{}", windowseventsprobehowmany.info_htot_logs);
}

fn windows_process_command_loop() -> i32 {

    let mut command: String = String::new();
    let stdin = io::stdin();

    'commandmain: loop {
        println!("");
        print!("rewin> windows> process> ");

        let _=stdout().flush();
        stdin.read_line(&mut command);
        command.make_ascii_lowercase();

        if let Some('\n') = command.chars().next_back() {
            command.pop();
        }
        if let Some('\r') = command.chars().next_back() {
            command.pop();
        }

        match command.as_str() {
            "quit" | "exit" | "q" | "e" =>
            {
                return 0;
            },
            "v" | "version" | "ver" => {
                command_main_version_execute();
            }
            "?" | "help" | "h" => {
                windows_process_command_help();
            },
            "cls" | "clear" => {
                clrscr();
            },
            "ps" => {
                command_windows_get_process_list_execute();
            },
            ".." => {
                return 55;
            },
            _ => {
                if command.is_empty() {
                    // do nothing
                }
                else {
                    println!("{command} command not found.");
                }
            },
        }
        command.clear();
    }
}

fn windows_process_command_help() {

    println!("basic commands");
    println!("..                   - go back to previous menu.");
    println!("version / ver / v    - get the version of reconwin program");
    println!("h / help / ?         - get available commands within this section");
    println!("quit / exit / q / e  - quit or exit reconwin program");
    println!("cls / clear          - clears the screen.");
    println!("credits              - credits to cargo packages used, developers and others who directly or in-directly supported.");
    println!("\nwindows processes");
    println!("ps         - list running processes within windows system.");
    println!("which      - . TODO . which locates executable of running process. TODO .");

}


fn exitthis() {

    process::exit(1);
    
}


fn print_end_of_program() {

    println!("------------------------------------------------------------------");
}

fn clrscr() {
    println!("{}[2J{esc}[1;1H", esc = 27 as char);
}