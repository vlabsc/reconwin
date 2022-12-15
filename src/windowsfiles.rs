// 0.16.1 - windows_desktop_directory_files

use sysinfo::{Pid, ProcessExt, System, SystemExt, UserExt};
use colored::Colorize;
use winreg::enums::*;
use cli_clipboard::{ClipboardContext};
use std::{env, fs};
use directories::UserDirs;
use walkdir::WalkDir;
use chrono::prelude::*;

pub struct windowsfilesprobe {
    pub user_directories_info: String,
    pub appdata_roaming_directory_files: String,
    pub appdata_local_directory_files: String,
    pub appdata_local_temp_directory_files: String,
    pub windows_temp_directory_files: String,
    pub windows_desktop_directory_files: String,
    pub windows_pfiles_directory_files: String,
    pub windows_pfiles86_directory_files: String,
    pub windows_windows_directory_files_info: String,
    pub windows_windows_system32_directory_files_info: String,
}




impl windowsfilesprobe {

    pub fn windows_show_user_directories(&mut self) {

        let pubadir = UserDirs::new();
        self.user_directories_info = format!("user_directories_info ... \n").cyan().bold().to_string();

        
        if let Some(user_dirs) = UserDirs::new() {
            let homedir = user_dirs.home_dir();
            self.user_directories_info.push_str(format!("home dir: {}\n", homedir.to_str().unwrap()).as_str());

            let desktopdir = user_dirs.desktop_dir();
            match desktopdir {
                Some(dir) => self.user_directories_info.push_str(format!("desktop dir: {}\n", dir.to_str().unwrap()).as_str()),
                None => println!("error"),
            }
            let documentdir = user_dirs.document_dir();
            match documentdir {
                Some(dir) => self.user_directories_info.push_str(format!("documents dir: {}\n", dir.to_str().unwrap()).as_str()),
                None => println!("error"),
            }
    
            let downloaddir = user_dirs.download_dir();
            match downloaddir {
                Some(dir) => self.user_directories_info.push_str(format!("downloads dir: {}\n", dir.to_str().unwrap()).as_str()),
                None => println!("error"),
            }

            let picturedir = user_dirs.picture_dir();
            match picturedir {
                Some(dir) => self.user_directories_info.push_str(format!("pictures dir: {}\n", dir.to_str().unwrap()).as_str()),
                None => println!("error"),
            }

            let audiodir = user_dirs.audio_dir();
            match audiodir {
                Some(dir) => self.user_directories_info.push_str(format!("music dir: {}\n", dir.to_str().unwrap()).as_str()),
                None => println!("error"),
            }

            let videodir = user_dirs.video_dir();
            match videodir {
                Some(dir) => self.user_directories_info.push_str(format!("videos dir: {}\n", dir.to_str().unwrap()).as_str()),
                None => println!("error"),
            }
        }
    }
}


impl windowsfilesprobe {

    pub fn windows_appdata_roaming_directory_files(&mut self) {

        //return;
    
        if let Some(user_dirs) = UserDirs::new() {

            let homedir = user_dirs.home_dir();
            let homedirstring = homedir.to_str().unwrap();

            let appdataroaming_dir = format!("{}{}", homedirstring, "\\AppData\\");
            self.appdata_roaming_directory_files = format!("files under directory {}\\Roaming\\ ... \n", appdataroaming_dir).cyan().bold().to_string();

            env::set_current_dir(&appdataroaming_dir);
            for entry in WalkDir::new("Roaming").into_iter().filter_map(|e| e.ok()) {
                if entry.metadata().unwrap().is_dir() {
                    self.appdata_roaming_directory_files.push_str(
                        format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                    );
                    } else if entry.metadata().unwrap().is_file() {
                    self.appdata_roaming_directory_files.push_str(
                        format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                    );          
                }
            }
        }
    }
}

impl windowsfilesprobe {

    pub fn windows_appdata_local_directory_files(&mut self) {

        //return;

        if let Some(user_dirs) = UserDirs::new() {

            let homedir = user_dirs.home_dir();
            let homedirstring = homedir.to_str().unwrap();

            let appdatalocal_dir = format!("{}{}", homedirstring, "\\AppData\\");
            env::set_current_dir(&appdatalocal_dir);
            self.appdata_local_directory_files = format!("files under directory {}\\Local\\ ... \n", appdatalocal_dir).cyan().bold().to_string();

            for entry in WalkDir::new("Local").into_iter().filter_map(|e| e.ok()) {
                if entry.metadata().unwrap().is_dir() {
                    self.appdata_local_directory_files.push_str(
                        format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                    );
                } else if entry.metadata().unwrap().is_file() {
                    self.appdata_local_directory_files.push_str(
                        format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                    );
                }
            }
        }

    }
}

impl windowsfilesprobe {

    pub fn windows_appdata_local_temp_directory_files(&mut self) {
        
        //return;

        if let Some(user_dirs) = UserDirs::new() {

            let homedir = user_dirs.home_dir();
            let homedirstring = homedir.to_str().unwrap();

            let appdatalocal_dir = format!("{}{}", homedirstring, "\\AppData\\Local\\");
            env::set_current_dir(&appdatalocal_dir);

            self.appdata_local_temp_directory_files = format!("files under directory {}\\Temp\\ ... ",appdatalocal_dir).cyan().bold().to_string();

            for entry in WalkDir::new("Temp").into_iter().filter_map(|e| e.ok()) {

                if entry.metadata().unwrap().is_dir() {
                    self.appdata_local_temp_directory_files.push_str(
                        format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                    );
                } else if entry.metadata().unwrap().is_file() {
                    self.appdata_local_temp_directory_files.push_str(
                        format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                    );
                }
            }
        }
    }
}

impl windowsfilesprobe {

    pub fn windows_temp_directory_files(&mut self) {
        
        //return;

        env::set_current_dir("C:\\Windows\\");
        self.windows_temp_directory_files = format!("files under directory C:\\Windows\\Temp ... \n").cyan().bold().to_string();

        for entry in WalkDir::new("Temp").into_iter().filter_map(|e| e.ok()) {

            if entry.metadata().unwrap().is_dir() {
                self.windows_temp_directory_files.push_str(
                    format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            } else if entry.metadata().unwrap().is_file() {
                self.windows_temp_directory_files.push_str(
                    format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            }
        }
    }

}

impl windowsfilesprobe {

    pub fn windows_desktop_directory_files(&mut self) {
        
        //return;

        if let Some(user_dirs) = UserDirs::new() {

            let desktopdir = user_dirs.desktop_dir();
            let desktopdirstring = user_dirs.desktop_dir().unwrap().to_string_lossy();

            let desktopdirstring_dir = format!("{}", desktopdirstring);
            env::set_current_dir(&desktopdirstring_dir);

            //env::set_current_dir(desktopdir);

            match desktopdir {
                Some(dir) => {
                    self.windows_desktop_directory_files = format!("files under directory {} ... \n", desktopdir.unwrap().to_string_lossy()).cyan().bold().to_string();;
                    //println!("files under directory {:?} ... \n", desktopdirstring_dir);

                    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
                
                        if entry.metadata().unwrap().is_dir() {
                            self.windows_desktop_directory_files.push_str(
                            format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                            );

                            //println!("dir: {}\n", entry.path().display());

                        } else if entry.metadata().unwrap().is_file() {
                            self.windows_desktop_directory_files.push_str(
                            format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                            );

                            //println!("file: {}\n", entry.path().display());
                        }
                    }
                },
                None => println!("error"),
            }
        }
    }

}

impl windowsfilesprobe {

    pub fn windows_pfiles_directory_files(&mut self) {
        
        //return;

        env::set_current_dir("C:\\Program Files\\");
        self.windows_pfiles_directory_files = format!("files under directory C:\\Program Files\\ ... \n").cyan().bold().to_string();

        for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {

            if entry.metadata().unwrap().is_dir() {
                self.windows_pfiles_directory_files.push_str(
                    format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            } else if entry.metadata().unwrap().is_file() {
                self.windows_pfiles_directory_files.push_str(
                    format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            }
        }
    }
}

impl windowsfilesprobe {

    pub fn windows_pfiles86_directory_files(&mut self) {
        
        //return;

        env::set_current_dir("C:\\Program Files (x86)\\");
        self.windows_pfiles86_directory_files = format!("files under directory C:\\Program Files(x86)\\ ... \n").cyan().bold().to_string();

        for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {

            if entry.metadata().unwrap().is_dir() {
                self.windows_pfiles86_directory_files.push_str(
                    format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            } else if entry.metadata().unwrap().is_file() {
                self.windows_pfiles86_directory_files.push_str(
                    format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            }
        }
    }
}

impl windowsfilesprobe {

    pub fn windows_windows_directory_files_probe(&mut self) {
        
        env::set_current_dir("C:\\Windows\\");
        self.windows_windows_directory_files_info = format!("files under directory C:\\Windows\\ ... \n").cyan().bold().to_string();

        for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {

            if entry.metadata().unwrap().is_dir() {
                self.windows_windows_directory_files_info.push_str(
                    format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            } else if entry.metadata().unwrap().is_file() {
                self.windows_windows_directory_files_info.push_str(
                    format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            }
        }
    }
}

impl windowsfilesprobe {

    pub fn windows_windows_system32_directory_files_probe(&mut self) {
        
        env::set_current_dir("C:\\Windows\\");
        self.windows_windows_system32_directory_files_info = format!("files under directory C:\\Windows\\System32\\ ... \n").cyan().bold().to_string();

        for entry in WalkDir::new("System32").into_iter().filter_map(|e| e.ok()) {

            if entry.metadata().unwrap().is_dir() {
                self.windows_windows_system32_directory_files_info.push_str(
                    format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            } else if entry.metadata().unwrap().is_file() {
                self.windows_windows_system32_directory_files_info.push_str(
                    format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            }
        }
    }
}

pub fn windows_test() {

    println!("{}" , format!("windows test ... ").cyan().bold(), );


}

