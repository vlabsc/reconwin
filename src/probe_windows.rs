// 0.16

use sysinfo::{Pid, ProcessExt, System, SystemExt, UserExt};
use colored::Colorize;
use winreg::enums::*;
use cli_clipboard::{ClipboardContext};
use std::{env, fs};
use directories::UserDirs;
use walkdir::WalkDir;
use chrono::prelude::*;
use is_root::is_root;

pub struct windowsprobe {
    pub info_windows_osdetails: String,
    pub info_windows_users: String,
    pub info_windows_installed_applications: String,
    pub info_windows_startup_applications: String,
    pub info_windows_get_clipboard: String,
    pub info_windows_get_process_list: String,
    pub info_windows_datetime: String,
    pub info_windows_pwd: String,
    pub info_windows_isadmin: String,
}

impl windowsprobe {

    pub fn probe_windows_osdetails(&mut self) {

        let sys = System::new_all();

        self.info_windows_osdetails = format!("os ... \n").cyan().bold().to_string();

        self.info_windows_osdetails.push_str(format!("name: {}\n", sys.name().as_deref().unwrap()).as_str());
        self.info_windows_osdetails.push_str(format!("kernel version: {}\n", sys.kernel_version().as_deref().unwrap()).as_str());
        self.info_windows_osdetails.push_str(format!("os version: {}\n", sys.os_version().as_deref().unwrap()).as_str());
        self.info_windows_osdetails.push_str(format!("os version (long): {}\n", sys.long_os_version().as_deref().unwrap()).as_str());
        self.info_windows_osdetails.push_str(format!("host name: {}\n", sys.host_name().as_deref().unwrap()).as_str());
        
        let mut uptime_inseconds = sys.uptime();
        let mut uptime_inminutes = sys.uptime() / 60;
        let mut uptime_inhours = sys.uptime() / 3600;

        self.info_windows_osdetails.push_str(format!("uptime: {} hours, {} minutes, {} seconds\n", uptime_inhours, uptime_inminutes, uptime_inseconds).as_str());

        if uptime_inminutes > 59{
            uptime_inminutes = uptime_inminutes % 60;
        }

        self.info_windows_osdetails.push_str(format!("boot time: {}\n", sys.boot_time()).as_str());
        self.info_windows_osdetails.push_str(format!("host name: {}\n", sys.host_name().as_deref().unwrap()).as_str());
        
        let current_dir = env::current_dir();
        if current_dir.is_ok() {
            match current_dir {
                Ok(curdir) => self.info_windows_osdetails.push_str(format!("current directory: {}\n", curdir.into_os_string().into_string().unwrap()).as_str()),
                Err(e) => self.info_windows_osdetails.push_str(format!("error while reading current directory {:?}\n", e).as_str()),
            }
        }

        if is_root() {
            self.info_windows_osdetails.push_str(format!("is administrator: yes\n").as_str());
        }
        else {
            self.info_windows_osdetails.push_str(format!("is administrator: no\n").as_str());
        }
    }
}


impl windowsprobe {

    pub fn probe_windows_isadmin(&mut self) {

        let sys = System::new_all();

        self.info_windows_isadmin = format!("is administrator? ").to_string();

        if is_root() {
            self.info_windows_isadmin.push_str(format!("yes\n").as_str());
        }
        else {
            self.info_windows_isadmin.push_str(format!("no\n").as_str());
        }
    }
}

impl windowsprobe {

    pub fn probe_windows_pwd(&mut self) {

        let sys = System::new_all();

        self.info_windows_pwd = format!("present working directory ... \n").cyan().bold().to_string();
        let current_dir = env::current_dir();
        if current_dir.is_ok() {
            match current_dir {
                Ok(curdir) => self.info_windows_pwd.push_str(format!("-> {}\n", curdir.into_os_string().into_string().unwrap()).as_str()),
                Err(e) => self.info_windows_pwd.push_str(format!("error while reading present working directory {:?}\n", e).as_str()),
            }
        }
    }
}


impl windowsprobe {

    pub fn probe_windows_users(&mut self) {

        let sys = System::new_all();

        self.info_windows_users = format!("windows user accounts ... \n").cyan().bold().to_string();
        
        for user in sys.users() {
            self.info_windows_users.push_str(format!("user: {}, user id: {}, gid: {}, in groups: {}\n"
            , user.name(), &user.id().to_string(), *user.group_id(), user.groups().len()).as_str());
        }
    }
}


impl windowsprobe {

    pub fn probe_windows_installed_applications(&mut self) {

        self.info_windows_installed_applications = format!("installed applications - windows registry ... \n").cyan().bold().to_string();

        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey =
        hklm.open_subkey_with_flags(r#"Software\Microsoft\Windows\CurrentVersion\Uninstall"#,
                                    KEY_READ).expect("Failed to open Registry");

        for key in subkey.enum_keys() {
            let _ = key.and_then(|key| subkey.open_subkey_with_flags(key, KEY_READ))
                    .and_then(|program_key| program_key.get_value("DisplayName"))
                    .and_then(|name: String| {
                                self.info_windows_installed_applications.push_str(format!("{}\n", name).as_str());
                                Ok(())
                            });
        }
    }
}


impl windowsprobe {

    pub fn probe_windows_startup_applications(&mut self) {

        self.info_windows_startup_applications = format!(
            "startup applications - windows registry (SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run) ... \n"
        ).cyan().bold().to_string();

        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey =
        hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\Windows\CurrentVersion\Run"#,
                                    KEY_READ).expect("Failed to open Registry");

        for (name, value) in subkey.enum_values().map(|x| x.unwrap()) {
            self.info_windows_startup_applications.push_str(format!("{} = {}\n", name, value).as_str());
        }

        self.info_windows_startup_applications.push_str(format!("\n").as_str());
        self.info_windows_startup_applications.push_str(
            format!("startup applications - windows registry (SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\RunOnce)\n"
        ).cyan().bold().to_string().as_str()
        );

        let hklm2 = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey2 =
        hklm2.open_subkey_with_flags(r#"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce"#,
                                    KEY_READ).expect("Failed to open Registry");
        

        for (name, value) in subkey2.enum_values().map(|x| x.unwrap()) {
            self.info_windows_startup_applications.push_str(format!("{} = {}\n", name, value).as_str());
        }
    }
}


impl windowsprobe {

    pub fn probe_windows_get_clipboard(&mut self) {

        self.info_windows_get_clipboard = format!("windows clipboard ... \n").cyan().bold().to_string();

        if cli_clipboard::get_contents().is_ok() {
            let result: String = cli_clipboard::get_contents().unwrap();
            self.info_windows_get_clipboard.push_str(format!("{}\n", result).as_str());
        } else {
            self.info_windows_get_clipboard.push_str(format!("nothing or something wrong with clipboard!\n").as_str());
        }
    }
}

impl windowsprobe {

    pub fn probe_windows_get_process_list(&mut self) {

        let mut rowcount = 1;
        let sys = System::new_all();

        self.info_windows_get_process_list = format!("running processes ... \n").cyan().bold().to_string();
        self.info_windows_get_process_list.push_str(format!("pid     - ppid   - process name    - exe path\n").cyan().green().to_string().as_str());

        for (pid, process) in sys.processes() {
            self.info_windows_get_process_list.push_str(format!("[{}]  - ", pid).as_str());
            //self.info_windows_get_process_list.push_str(format!("{:17}  - ", pid).as_str());

            match process.parent() {
                Some(p) => self.info_windows_get_process_list.push_str(format!("[{}]  - ", p).as_str()),
                //Some(p) => self.info_windows_get_process_list.push_str(format!("{:7}  - ", p).as_str()),
                None => self.info_windows_get_process_list.push_str(format!("error  - ").as_str()),
            }
            self.info_windows_get_process_list.push_str(format!("{} - {:?}\n", process.name(), process.exe()).as_str());
            if rowcount > 32 {
                rowcount = 1;
                self.info_windows_get_process_list.push_str(format!("pid     - ppid   - process name    - exe path\n").cyan().green().to_string().as_str());
            } else {
                rowcount = rowcount + 1;
            }
        }
    
    }
}



impl windowsprobe {
    pub fn probe_windows_datetime(&mut self) {

        //println!("{}" , format!("time and date ... ").cyan().bold(), );
        self.info_windows_datetime= format!("time and date ... \n").cyan().bold().to_string();

        let utc = Utc::now();
        let local = Local::now();
         
        //println!("UTC time now: {:?}", utc.to_rfc2822());
        self.info_windows_datetime.push_str(
            format!("UTC time now: {:?}\n", utc.to_rfc2822().as_str()).as_str()
        );

        //println!("local time now: {:?}", local.to_rfc2822());            
        self.info_windows_datetime.push_str(
            format!("local time now: {:?}\n", local.to_rfc2822().as_str()).as_str()
        );
    }
}
    

pub fn windows_test() {

    println!("{}" , format!("windows test ... ").cyan().bold(), );


}

