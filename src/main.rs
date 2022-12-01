// 0.16


use colored::Colorize;
use std::io;
use std::io::*;
use std::process;


extern crate winreg;

mod hardwareprobe;
mod disk;
mod mem;
mod windows;
mod windowsfiles;
mod windowsevents;



fn main() {

    hello_world();
    
    //get_all_hardware_info();
    //get_all_windows_info();

    let ret = main_command_loop();
    //let ret = windows_command_loop();

    print_end_of_program();
}

fn hello_world() {

    println!("");
    println!("------------------------------------------------------------------");
    println!("{}" , format!("recon-windows v0.16 beta").bold().green());
    println!("");

}



fn main_command_loop() -> i32 {

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
            "quit" | "exit" | "q" | "e" => { 
                return 1;
                //break 'commandmain;
             },
            "hardware" | "hw" => {
                command.clear();
                let ret = hardware_command_loop();
                if ret == 0 {
                    return 0;
                }
            },
            "win" | "windows" => {
                command.clear();
                let ret = windows_command_loop();
                if ret == 0 {
                    return 0;
                }
            },
            "v" | "version" | "ver" => {
                main_command_version();
            }
            "?" | "help" | "h" => {
                main_command_help();
            },
            //_ => print!("what command is this {command}"),
            _ => { 
                println!("{command} command not found.");
            },
        }
        //println!("command is: {command}");
        command.clear();
    }
}

fn main_command_help() {

    println!("command              - description ");
    println!("hardware / hw        - probe hardware related information");
    println!("windows / win        - probe windows related informationo (directories, temp files; etc)");
    println!("version / ver / v    - get the version of reconwin program");
    println!("h / help / ?         - get available commands within this section");
    println!("quit / exit / q / e  - quit or exit reconwin program");

}

fn main_command_version() {

    println!("{}" , format!("recon-windows v0.16 beta"));
}

fn hardware_command_loop() -> i32 {

    let mut command: String = String::new();
    let stdin = io::stdin();
    
    'commandmain: loop {
        println!("");
        print!("hardware> ");
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
                main_command_version();
            }
            "?" | "help" | "h" => {
                hardware_command_help();
            },
            "cpu" => {
                hardware_command_cpu_execute();
            },
            "nw" | "network" => {
                hardware_command_nw_execute();
            },    
            "disks" => {
                windows_command_disks_execute();
            },
            "mem" => {
                windows_command_mem_execute();
            },
            "all" => {
                hardware_command_all_execute();
            },
            ".." => {
                return 55;
            }
            _ => {
                println!("{:?} command not found.", command);
            },
        }
        //println!("command is: {command}");
        command.clear();
    }
}


fn hardware_command_help() {

    println!("command              - description ");
    println!("cpu                  - probe cpu information (frequency, number of cores; etc)");
    println!("nw                   - probe network interfaces information (IPv6 IP address, IPv4 IP address; etc)");
    println!("mem                  - probe memory related information (total memory, available free, utilized)");
    println!("disks                - list available disks within windows system.");
    println!("all                  - probe all info - cpu, network and memory.");
    println!("..                   - go back to main menu.");
    println!("version / ver / v    - get the version of reconwin program");
    println!("h / help / ?         - get available commands within this section");
    println!("quit / exit / q / e  - quit or exit reconwin program");
}


fn hardware_command_cpu_execute() -> i32 {

    let mut hardware = hardwareprobe::hardwareprobe {
        cpu_info: String::from("cpu info"),
        nw_info: String::from("nw info")
    };

    hardware.get_cpu_logical_cores();

    println!("{}", hardware.cpu_info);
    return 100;
}

fn hardware_command_nw_execute() -> i32 {

    let mut hardware = hardwareprobe::hardwareprobe {
        cpu_info: String::from("cpu info"),
        nw_info: String::from("nw info")
    };

    hardware.get_network_interfaces();

    println!("{}", hardware.nw_info);
    return 100;
}

fn hardware_command_all_execute() {

    //let mut cpu_info = hardware::hardware_cpu{info: "cpu info".to_string()};
    
    let mut hardware = hardwareprobe::hardwareprobe {
        cpu_info: String::from("cpu info"),
        nw_info: String::from("nw info")
    };
    let mut disk = disk::diskprobe {
        disk_info: String::from("disk info"),
    };


let mut mem = mem::memprobe {
        mem_info: String::from("mem info"),
    };

    hardware.get_cpu_logical_cores();
    hardware.get_network_interfaces();
    mem.windows_mem_info();
    disk.windows_disk_info();

    println!("{}", hardware.cpu_info);
    println!("{}", hardware.nw_info);
    println!("{}", mem.mem_info);
    println!("{}", disk.disk_info);    
}


fn windows_command_loop() -> i32 {

    let mut command: String = String::new();
    let stdin = io::stdin();
    
    'commandmain: loop {
        println!("");
        print!("windows> ");
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
                main_command_version();
            }
            "?" | "help" | "h" => {
                windows_command_help();
            },
            "os" => {
                windows_command__os_execute();
            },
            "users" => {
                windows_command_users_execute();
            },
            "iapps" => {
                windows_command_iapps_execute();
            },
            "sapps" => {
                windows_command_sapps_execute();
            },
            "cb" => {
                windows_command_cb_execute();
            },    
            "disks" => {
                windows_command_disks_execute();
            },
            "mem" => {
                windows_command_mem_execute();
            },
            "ps" => {
                windows_command_process_execute();
            },
            "ud" => {
                windows_files_ud_command_execute();
            },
            "time" => {
                windows_command_datetime_execute();
            },
            "all" => {
                windows_all_command_execute();
            },
            "files" => {
                let ret = windows_files_command_loop();
                if ret == 0 {
                    return 0;
                }
            },
            ".." => {
                return 55;
            }
            _ => {
                println!("{:?} command not found.", command);
            },
        }
        //println!("command is: {command}");
        command.clear();
    }
}



fn windows_command_help() {

    println!("command              - description ");
    println!("os                   - probe operating system related information.");
    println!("users                - probe windows account details. list of users within windows system.");
    println!("iapps                - list of installed programs within the windows system.");
    println!("sapps                - list of programs configured to run at startup. probes through registry.");
    println!("cb                   - list clipboard content.");
    println!("ps                   - list running processes within windows system.");
    println!("ud                   - list windows users directories.");
    println!("cb                   - list clipboard content.");
    println!("files                - list files within specific windows directory.");
    println!("time                 - print the current windows time.");
    
    println!("mem                  - probe memory related information (total memory, available free, utilized)");
    println!("disks                - list available disks within windows system.");
    println!("all                  - probe all info");

    
        
    println!("..                   - go back to main menu.");
    println!("version / ver / v    - get the version of reconwin program");
    println!("h / help / ?         - get available commands within this section");
    println!("quit / exit / q / e  - quit or exit reconwin program");
}

fn windows_command__os_execute() {

    let mut windows = windows::windowsprobe {
        os_info: String::from("mem info"),
        users_info: String::from("users info"),
        installed_applications_info: String::from("applications info"),
        registry_startup_applications_info: String::from("registry startup applications info"),
        clipboard_info: String::from("clipboard info"),
        process_info: String::from("process info"),
        windows_datetime_info: String::from("windows date time info"),
    };

    windows.windows_os_info();
    println!("{}", windows.os_info);
}


fn windows_command_users_execute() {

    let mut windows = windows::windowsprobe {
        os_info: String::from("mem info"),
        users_info: String::from("users info"),
        installed_applications_info: String::from("applications info"),
        registry_startup_applications_info: String::from("registry startup applications info"),
        clipboard_info: String::from("clipboard info"),
        process_info: String::from("process info"),
        windows_datetime_info: String::from("windows date time info"),
    };

    windows.windows_users_info();
    println!("{}", windows.users_info);
}

fn windows_command_process_execute() {

    let mut windows = windows::windowsprobe {
        os_info: String::from("mem info"),
        users_info: String::from("users info"),
        installed_applications_info: String::from("applications info"),
        registry_startup_applications_info: String::from("registry startup applications info"),
        clipboard_info: String::from("clipboard info"),
        process_info: String::from("process info"),
        windows_datetime_info: String::from("windows date time info"),
    };

    windows.windows_process_information();
    println!("{}", windows.process_info);
}



fn windows_command_cb_execute() {

    let mut windows = windows::windowsprobe {
        os_info: String::from("mem info"),
        users_info: String::from("users info"),
        installed_applications_info: String::from("applications info"),
        registry_startup_applications_info: String::from("registry startup applications info"),
        clipboard_info: String::from("clipboard info"),
        process_info: String::from("process info"),
        windows_datetime_info: String::from("windows date time info"),
    };

    windows.windows_get_clipboard();
    println!("{}", windows.clipboard_info);
}


fn windows_command_iapps_execute() {

    let mut windows = windows::windowsprobe {
        os_info: String::from("mem info"),
        users_info: String::from("users info"),
        installed_applications_info: String::from("applications info"),
        registry_startup_applications_info: String::from("registry startup applications info"),
        clipboard_info: String::from("clipboard info"),
        process_info: String::from("process info"),
        windows_datetime_info: String::from("windows date time info"),
    };

    windows.windows_registry_installed_applications_info();
    println!("{}", windows.installed_applications_info);
}

fn windows_command_sapps_execute() {

    let mut windows = windows::windowsprobe {
        os_info: String::from("mem info"),
        users_info: String::from("users info"),
        installed_applications_info: String::from("applications info"),
        registry_startup_applications_info: String::from("registry startup applications info"),
        clipboard_info: String::from("clipboard info"),
        process_info: String::from("process info"),
        windows_datetime_info: String::from("windows date time info"),
    };

    windows.windows_registry_startup_applications_info();
    println!("{}", windows.registry_startup_applications_info);
}

fn windows_command_datetime_execute() {

    let mut windows = windows::windowsprobe {
        os_info: String::from("mem info"),
        users_info: String::from("users info"),
        installed_applications_info: String::from("applications info"),
        registry_startup_applications_info: String::from("registry startup applications info"),
        clipboard_info: String::from("clipboard info"),
        process_info: String::from("process info"),
        windows_datetime_info: String::from("windows date time info"),
    };

    windows.windows_datetime();
    print!("{}", windows.windows_datetime_info);

    //windows::windows_test();

}

fn windows_command_disks_execute() {
    let mut disk = disk::diskprobe {
        disk_info: String::from("disk info"),
    };

    disk.windows_disk_info();
    println!("{}", disk.disk_info);
}

fn windows_command_mem_execute() {
    let mut mem = mem::memprobe {
        mem_info: String::from("mem info"),
    };

    mem.windows_mem_info();
    println!("{}", mem.mem_info);
}


fn windows_all_command_execute() {

    let mut disk = disk::diskprobe {
        disk_info: String::from("disk info"),
    };

    let mut mem = mem::memprobe {
        mem_info: String::from("mem info"),
    };

    let mut windows = windows::windowsprobe {
        os_info: String::from("mem info"),
        users_info: String::from("users info"),
        installed_applications_info: String::from("applications info"),
        registry_startup_applications_info: String::from("registry startup applications info"),
        clipboard_info: String::from("clipboard info"),
        process_info: String::from("process info"),
        windows_datetime_info: String::from("windows date time info"),
    };


    disk.windows_disk_info();
    println!("{}", disk.disk_info);

    mem.windows_mem_info();
    println!("{}", mem.mem_info);

    
    windows.windows_os_info();
    println!("{}", windows.os_info);

    windows.windows_users_info();
    println!("{}", windows.users_info);

    windows.windows_registry_installed_applications_info();
    println!("{}", windows.installed_applications_info);
        

    windows.windows_registry_startup_applications_info();
    println!("{}", windows.registry_startup_applications_info);

    windows.windows_get_clipboard();
    println!("{}", windows.clipboard_info);

    windows.windows_process_information();
    println!("{}", windows.process_info);
    
    /*
    windows.windows_show_user_directories();
    println!("{}", windows.user_directories_info);
    
    windows.windows_appdata_roaming_directory_files();
    println!("{}", windows.appdata_roaming_directory_files);

    windows.windows_appdata_local_directory_files();
    println!("{}", windows.appdata_local_directory_files);

    windows.windows_appdata_local_temp_directory_files();
    println!("{}", windows.appdata_local_temp_directory_files);

    windows.windows_temp_directory_files();
    println!("{}", windows.windows_temp_directory_files);
    */


    /*
    windows::windows_test();
    println!("");

    //windowsevents::parse_events();
    //println!("");
    */

}



fn windows_files_command_loop() -> i32 {

    let mut command: String = String::new();
    let stdin = io::stdin();

    'commandmain: loop {
        println!("");
        print!("windows> files> ");
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
                main_command_version();
            }
            "?" | "help" | "h" => {
                windows_files_command_help();
            },
            "ud" => {
                windows_files_ud_command_execute();
            },
            "appdata" => {
                windows_files_appdata_command_execute();
            },
            "local" => {
                windows_files_local_command_execute();
            },
            "localtemp" => {
                windows_files_localtemp_command_execute();
            },
            "temp" => {
                windows_files_temp_command_execute();
            },
            "desktop" => {
                windows_files_desktop_command_execute();
            },
            "all" => {
                windows_all_command_execute();
            },
            "files" => {
                windows_all_command_execute();
            },
            ".." => {
                return 55;
            }
            _ => {
                println!("{:?} command not found.", command);
            },
        }
        //println!("command is: {command}");
        command.clear();
    }
}

fn windows_files_command_help() {

    println!("command              - description ");
    println!("ud                   - list windows users directories.");
    println!("appdata              - list files within users %appdata% directory.");
    println!("local                - list files within users %appdata%\\Local directory.");
    println!("localtemp            - list files within users %appdata%\\Local\\Temp directory.");
    println!("desktop              - list files within windows desktop directory.");
    println!("temp                 - list files within windows temp directory.");
    println!("test                 - test.");        
    println!("..                   - go back to main menu.");
    println!("version / ver / v    - get the version of reconwin program");
    println!("h / help / ?         - get available commands within this section");
    println!("quit / exit / q / e  - quit or exit reconwin program");
}

fn windows_files_ud_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
    };

    windowsfiles.windows_show_user_directories();
    println!("{}", windowsfiles.user_directories_info);
}


fn windows_files_appdata_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
    };

    windowsfiles.windows_appdata_roaming_directory_files();
    println!("{}", windowsfiles.appdata_roaming_directory_files);
}

fn windows_files_local_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
    };

    windowsfiles.windows_appdata_local_directory_files();
    println!("{}", windowsfiles.appdata_local_directory_files);
}


fn windows_files_localtemp_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
    };

    windowsfiles.windows_appdata_local_temp_directory_files();
    println!("{}", windowsfiles.appdata_local_temp_directory_files);
}

fn windows_files_temp_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
    };

    windowsfiles.windows_temp_directory_files();
    println!("{}", windowsfiles.windows_temp_directory_files);
}

fn windows_files_desktop_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
    };

    windowsfiles.windows_desktop_directory_files();
    println!("{}", windowsfiles.windows_desktop_directory_files);
}


fn exitthis() {

    process::exit(1);
    
}


fn print_end_of_program() {

    println!("------------------------------------------------------------------");
}