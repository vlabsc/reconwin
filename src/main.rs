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
            "cls" | "clear" => {
                clrscr();
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

    //    println!("command              - description ");
        println!("hardware / hw        - probe hardware related information");
        println!("windows / win        - probe windows related informationo (directories, temp files; etc)");
        println!("version / ver / v    - get the version of reconwin program");
        println!("h / help / ?         - get available commands within this section");
        println!("quit / exit / q / e  - quit or exit reconwin program");
        println!("cls / clear          - clears the screen.");
    }
    
fn main_command_version() {
    println!("{}" , format!("recon-windows v0.16 beta"));
}


fn hardware_command_loop() -> i32 {

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
            "cls" | "clear" => {
                clrscr();
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

    println!("basic commands");
    println!("..                   - go back to main menu.");
    println!("version / ver / v    - get the version of reconwin program");
    println!("h / help / ?         - get available commands within this section");
    println!("quit / exit / q / e  - quit or exit reconwin program");
    println!("cls / clear          - clears the screen.");
    println!("\nspecific commands");
    println!("cpu                  - probe cpu information (frequency, number of cores; etc)");
    println!("nw                   - probe network interfaces information (IPv6 IP address, IPv4 IP address; etc)");
    println!("mem                  - probe memory related information (total memory, available free, utilized)");
    println!("disks                - list available disks within windows system.");
    println!("all                  - probe all info - cpu, network and memory.");
}


fn hardware_command_cpu_execute() -> i32 {

    let mut hardware = hardwareprobe::hardwareprobe {
        cpu_info: String::from("cpu info"),
        nw_info: String::from("nw info")
    };

    hardware.get_cpu_logical_cores();

    print!("{}", hardware.cpu_info);
    return 100;
}

fn hardware_command_nw_execute() -> i32 {

    let mut hardware = hardwareprobe::hardwareprobe {
        cpu_info: String::from("cpu info"),
        nw_info: String::from("nw info")
    };

    hardware.get_network_interfaces();

    print!("{}", hardware.nw_info);
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
    print!("{}", disk.disk_info);    
}


fn windows_command_loop() -> i32 {

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
                main_command_version();
            }
            "?" | "help" | "h" => {
                windows_command_help();
            },
            "os" => {
                windows_command_os_execute();
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
            "pwd" => {
                windows_command_pwd_execute();
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
            "process" => {
                let ret = windows_process_command_loop();
                if ret == 0 {
                    return 0;
                }
            },
            "files" => {
                let ret = windows_files_command_loop();
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
                println!("{:?} command not found.", command);
            },
        }
        //println!("command is: {command}");
        command.clear();
    }
}



fn windows_command_help() {

    println!("basic commands");
    println!("..                   - go back to main menu.");
    println!("version / ver / v    - get the version of reconwin program");
    println!("h / help / ?         - get available commands within this section");
    println!("quit / exit / q / e  - quit or exit reconwin program");
    println!("cls / clear          - clears the screen.");
    println!("\nspecific commands");
    println!("os                   - probe operating system related information.");
    println!("users                - probe windows account details. list of users within windows system.");
    println!("iapps                - list of installed programs within the windows system. probes through registry.");
    println!("sapps                - list of programs configured to run at startup. probes through registry.");
    println!("cb                   - list clipboard content.");
    println!("pwd                  - display the present working directory.");
    println!("ps                   - list running processes within windows system.");
    println!("ud                   - list windows users directories.");
    println!("cb                   - list clipboard content.");
    println!("time                 - print the current windows time.");
    println!("mem                  - probe memory related information (total memory, available free, utilized)");
    println!("disks                - list available disks within windows system.");
    println!("all                  - probe all info");

    println!("\nsub modules");
    println!("files                - probe files, directories, config paths, temp folders and others within windows fs.");
    println!("events               - probe windows events. ");
    println!("reg                  - . TODO. probe windows registry. TODO . ");
    println!("stasks               - . TODO. probe scheduled tasks. TODO . ");
    println!("services             - . TODO. probe windows services. TODO . ");
    println!("process              - . TODO. probe windows processes. TODO . ");
}

fn windows_command_os_execute() {

    let mut windows = windows::windowsprobe {
        os_info: String::from("mem info"),
        users_info: String::from("users info"),
        installed_applications_info: String::from("applications info"),
        registry_startup_applications_info: String::from("registry startup applications info"),
        clipboard_info: String::from("clipboard info"),
        process_info: String::from("process info"),
        windows_datetime_info: String::from("windows date time info"),
        pwd_info:String::from("present working directory"),
    };

    windows.windows_os_info();
    print!("{}", windows.os_info);
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
        pwd_info:String::from("present working directory"),
    };

    windows.windows_users_info();
    print!("{}", windows.users_info);
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
        pwd_info:String::from("present working directory"),
    };

    windows.windows_process_information();
    print!("{}", windows.process_info);
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
        pwd_info:String::from("present working directory"),
    };

    windows.windows_get_clipboard();
    print!("{}", windows.clipboard_info);
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
        pwd_info:String::from("present working directory"),
    };

    windows.windows_registry_installed_applications_info();
    print!("{}", windows.installed_applications_info);
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
        pwd_info:String::from("present working directory"),
    };

    windows.windows_registry_startup_applications_info();
    print!("{}", windows.registry_startup_applications_info);
}

fn windows_command_pwd_execute() {

    let mut windows = windows::windowsprobe {
        os_info: String::from("mem info"),
        users_info: String::from("users info"),
        installed_applications_info: String::from("applications info"),
        registry_startup_applications_info: String::from("registry startup applications info"),
        clipboard_info: String::from("clipboard info"),
        process_info: String::from("process info"),
        windows_datetime_info: String::from("windows date time info"),
        pwd_info:String::from("present working directory"),
    };

    windows.windows_pwd();
    print!("{}", windows.pwd_info);
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
        pwd_info:String::from("present working directory"),
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
    print!("{}", disk.disk_info);
}

fn windows_command_mem_execute() {
    let mut mem = mem::memprobe {
        mem_info: String::from("mem info"),
    };

    mem.windows_mem_info();
    print!("{}", mem.mem_info);
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
        pwd_info:String::from("present working directory"),
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
    print!("{}", windows.process_info);

}



fn windows_files_command_loop() -> i32 {

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
            "pfiles" => {
                windows_files_pfiles_command_execute();
            },
            "pfiles86" => {
                windows_files_pfiles86_command_execute();
            },
            "windows" => {
                windows_files_windows_command_execute();
            },
            "sys32" => {
                windows_files_windows_system32_command_execute();
            },
            "all" => {
                //windows_all_command_execute();
            },
            "files" => {
                windows_all_command_execute();
            },
            "cls" | "clear" => {
                clrscr();
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

    println!("basic commands");
    println!("..                   - go back to main menu.");
    println!("version / ver / v    - get the version of reconwin program");
    println!("h / help / ?         - get available commands within this section");
    println!("quit / exit / q / e  - quit or exit reconwin program");
    println!("cls / clear          - clears the screen.");
    println!("\nspecific commands");
//    println!("command              - description ");
    println!("ud                   - list windows users directories.");
    println!("appdata              - list files within users %appdata% directory.");
    println!("local                - list files within users %appdata%\\Local directory.");
    println!("localtemp            - list files within users %appdata%\\Local\\Temp directory.");
    println!("desktop              - list files within windows desktop directory.");
    println!("pfiles               - list files within C:\\Program Files\\.");
    println!("pfiles86             - list files within C:\\Program Files(x86)\\.");
    println!("windows              - list files within C:\\Windows\\.");
    println!("sys32                - list files within C:\\Windows\\System32\\.");
    println!("temp                 - list files within windows temp directory.");
    println!("test                 - test.");        
}

fn windows_files_ud_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
        windows_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        windows_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        windows_windows_directory_files_info: String::from("C:\\Winidows\\ info"),
        windows_windows_system32_directory_files_info: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.windows_show_user_directories();
    print!("{}", windowsfiles.user_directories_info);
}


fn windows_files_pfiles_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
        windows_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        windows_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        windows_windows_directory_files_info: String::from("C:\\Winidows\\ info"),
        windows_windows_system32_directory_files_info: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.windows_pfiles_directory_files();
    print!("{}", windowsfiles.windows_pfiles_directory_files);
}

fn windows_files_pfiles86_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
        windows_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        windows_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        windows_windows_directory_files_info: String::from("C:\\Winidows\\ info"),
        windows_windows_system32_directory_files_info: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.windows_pfiles86_directory_files();
    print!("{}", windowsfiles.windows_pfiles86_directory_files);
}

fn windows_files_sys32_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
        windows_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        windows_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        windows_windows_directory_files_info: String::from("C:\\Winidows\\ info"),
        windows_windows_system32_directory_files_info: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.windows_windows_system32_directory_files_probe();
    print!("{}", windowsfiles.windows_windows_system32_directory_files_info);
}

fn windows_files_windows_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
        windows_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        windows_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        windows_windows_directory_files_info: String::from("C:\\Winidows\\ info"),
        windows_windows_system32_directory_files_info: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.windows_windows_directory_files_probe();
    print!("{}", windowsfiles.windows_windows_directory_files_info);
}

fn windows_files_windows_system32_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
        windows_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        windows_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        windows_windows_directory_files_info: String::from("C:\\Winidows\\ info"),
        windows_windows_system32_directory_files_info: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.windows_windows_system32_directory_files_probe();
    print!("{}", windowsfiles.windows_windows_system32_directory_files_info);
}

fn windows_files_appdata_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
        windows_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        windows_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        windows_windows_directory_files_info: String::from("C:\\Winidows\\ info"),
        windows_windows_system32_directory_files_info: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.windows_appdata_roaming_directory_files();
    print!("{}", windowsfiles.appdata_roaming_directory_files);
}

fn windows_files_local_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
        windows_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        windows_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        windows_windows_directory_files_info: String::from("C:\\Winidows\\ info"),
        windows_windows_system32_directory_files_info: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.windows_appdata_local_directory_files();
    print!("{}", windowsfiles.appdata_local_directory_files);
}


fn windows_files_localtemp_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
        windows_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        windows_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        windows_windows_directory_files_info: String::from("C:\\Winidows\\ info"),
        windows_windows_system32_directory_files_info: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.windows_appdata_local_temp_directory_files();
    print!("{}", windowsfiles.appdata_local_temp_directory_files);
}

fn windows_files_temp_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
        windows_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        windows_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        windows_windows_directory_files_info: String::from("C:\\Winidows\\ info"),
        windows_windows_system32_directory_files_info: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.windows_temp_directory_files();
    print!("{}", windowsfiles.windows_temp_directory_files);
}

fn windows_files_desktop_command_execute() {

    let mut windowsfiles = windowsfiles::windowsfilesprobe {
        user_directories_info: String::from("user directories info"),
        appdata_roaming_directory_files: String::from("%AppData% directory info"),
        appdata_local_directory_files: String::from("%AppData%\\Local directory info"),
        appdata_local_temp_directory_files: String::from("%AppData%\\Local\\Temp\\ directory info"),
        windows_temp_directory_files: String::from("\\Windows\\Temp\\ info"),
        windows_desktop_directory_files: String::from("\\desktop\\ info"),
        windows_pfiles_directory_files: String::from("C:\\Program Files\\ info"),
        windows_pfiles86_directory_files: String::from("C:\\Program Files(x86)\\ info"),
        windows_windows_directory_files_info: String::from("C:\\Winidows\\ info"),
        windows_windows_system32_directory_files_info: String::from("C:\\Windows\\System32\\ info"),
    };

    windowsfiles.windows_desktop_directory_files();
    print!("{}", windowsfiles.windows_desktop_directory_files);
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
                main_command_version();
            }
            "?" | "help" | "h" => {
                windows_events_command_help();
            },
            "cls" | "clear" => {
                clrscr();
            },
            "application" | "app"=> {
                windows_events_application_command_execute();
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

fn windows_events_command_help() {

    println!("basic commands");
    println!("..                   - go back to main menu.");
    println!("version / ver / v    - get the version of reconwin program");
    println!("h / help / ?         - get available commands within this section");
    println!("quit / exit / q / e  - quit or exit reconwin program");
    println!("cls / clear          - clears the screen.");
    println!("\nwindows logs");
    println!("application / app      - list application logs.");
    println!("security / sec         - list security logs.");
    println!("setup / set            - list setup logs.");
    println!("system / sys           - list system logs.");
    println!("forwardedevents / fe   - list forwarded events logs.");
}

fn windows_events_application_command_execute() {

    let mut windowsevents = windowsevents::windowseventsprobe {
        appication_logs: String::from("application logs setup"),
        security_logs: String::from("security logs setup"),
        setup_logs: String::from("setup logs setup"),
        system_logs: String::from("system logs setup"),
        forwardedevents_logs: String::from("forwarded events logs setup"),
    };

    windowsevents.windows_events_application_logs();
    print!("{}", windowsevents.appication_logs);
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
                main_command_version();
            }
            "?" | "help" | "h" => {
                windows_process_command_help();
            },
            "cls" | "clear" => {
                clrscr();
            },
            "ps" => {
                windows_command_process_execute();
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

fn windows_process_command_help() {

    println!("basic commands");
    println!("..                   - go back to main menu.");
    println!("version / ver / v    - get the version of reconwin program");
    println!("h / help / ?         - get available commands within this section");
    println!("quit / exit / q / e  - quit or exit reconwin program");
    println!("cls / clear          - clears the screen.");
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