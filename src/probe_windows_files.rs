// 0.16.2 - naming ...

use sysinfo::{Pid, ProcessExt, System, SystemExt, UserExt};
use colored::Colorize;
use winreg::enums::*;
use cli_clipboard::{ClipboardContext};
use std::{env, fs};
use directories::UserDirs;
use walkdir::WalkDir;
use chrono::prelude::*;

pub struct probewindowsfiles {
    pub info_windows_files_list_user_directories: String,
    pub info_windows_files_list_appdata_roaming_directory_files: String,
    pub info_windows_files_list_appdata_local_directory_files: String,
    pub info_windows_files_list_appdata_local_temp_directory_files: String,
    pub info_windows_files_list_temp_directory_files: String,
    pub info_windows_files_list_desktop_directory_files: String,
    pub info_windows_files_list_pfiles_directory_files: String,
    pub info_windows_files_list_pfiles86_directory_files: String,
    pub info_windows_files_list_windows_directory_files: String,
    pub info_windows_files_list_system32_directory_files: String,
}




impl probewindowsfiles {

    pub fn probe_windows_files_list_user_directories(&mut self) {

        let pubadir = UserDirs::new();
        self.info_windows_files_list_user_directories = format!("info_windows_files_list_user_directories ... \n").cyan().bold().to_string();

        if let Some(user_dirs) = UserDirs::new() {
            let homedir = user_dirs.home_dir();
            self.info_windows_files_list_user_directories.push_str(format!("home dir: {}\n", homedir.to_str().unwrap()).as_str());

            let desktopdir = user_dirs.desktop_dir();
            match desktopdir {
                Some(dir) => self.info_windows_files_list_user_directories.push_str(format!("desktop dir: {}\n", dir.to_str().unwrap()).as_str()),
                None => println!("error"),
            }
            let documentdir = user_dirs.document_dir();
            match documentdir {
                Some(dir) => self.info_windows_files_list_user_directories.push_str(format!("documents dir: {}\n", dir.to_str().unwrap()).as_str()),
                None => println!("error"),
            }
    
            let downloaddir = user_dirs.download_dir();
            match downloaddir {
                Some(dir) => self.info_windows_files_list_user_directories.push_str(format!("downloads dir: {}\n", dir.to_str().unwrap()).as_str()),
                None => println!("error"),
            }

            let picturedir = user_dirs.picture_dir();
            match picturedir {
                Some(dir) => self.info_windows_files_list_user_directories.push_str(format!("pictures dir: {}\n", dir.to_str().unwrap()).as_str()),
                None => println!("error"),
            }

            let audiodir = user_dirs.audio_dir();
            match audiodir {
                Some(dir) => self.info_windows_files_list_user_directories.push_str(format!("music dir: {}\n", dir.to_str().unwrap()).as_str()),
                None => println!("error"),
            }

            let videodir = user_dirs.video_dir();
            match videodir {
                Some(dir) => self.info_windows_files_list_user_directories.push_str(format!("videos dir: {}\n", dir.to_str().unwrap()).as_str()),
                None => println!("error"),
            }
        }
    }
}


impl probewindowsfiles {

    pub fn probe_windows_files_list_appdata_roaming_directory_files(&mut self) {

        //return;
    
        if let Some(user_dirs) = UserDirs::new() {

            let homedir = user_dirs.home_dir();
            let homedirstring = homedir.to_str().unwrap();

            let appdataroaming_dir = format!("{}{}", homedirstring, "\\AppData\\");
            self.info_windows_files_list_appdata_roaming_directory_files = format!("files under directory {}\\Roaming\\ ... \n", appdataroaming_dir).cyan().bold().to_string();

            env::set_current_dir(&appdataroaming_dir);
            for entry in WalkDir::new("Roaming").into_iter().filter_map(|e| e.ok()) {
                if entry.metadata().unwrap().is_dir() {
                    self.info_windows_files_list_appdata_roaming_directory_files.push_str(
                        format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                    );
                    } else if entry.metadata().unwrap().is_file() {
                    self.info_windows_files_list_appdata_roaming_directory_files.push_str(
                        format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                    );          
                }
            }
        }
    }
}

impl probewindowsfiles {

    pub fn probe_windows_files_list_appdata_local_directory_files(&mut self) {

        //return;

        if let Some(user_dirs) = UserDirs::new() {

            let homedir = user_dirs.home_dir();
            let homedirstring = homedir.to_str().unwrap();

            let appdatalocal_dir = format!("{}{}", homedirstring, "\\AppData\\");
            env::set_current_dir(&appdatalocal_dir);
            self.info_windows_files_list_appdata_local_directory_files = format!("files under directory {}\\Local\\ ... \n", appdatalocal_dir).cyan().bold().to_string();

            for entry in WalkDir::new("Local").into_iter().filter_map(|e| e.ok()) {
                if entry.metadata().unwrap().is_dir() {
                    self.info_windows_files_list_appdata_local_directory_files.push_str(
                        format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                    );
                } else if entry.metadata().unwrap().is_file() {
                    self.info_windows_files_list_appdata_local_directory_files.push_str(
                        format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                    );
                }
            }
        }
    }
}

impl probewindowsfiles {

    pub fn probe_windows_files_list_appdata_local_temp_directory_files(&mut self) {
        
        //return;

        if let Some(user_dirs) = UserDirs::new() {

            let homedir = user_dirs.home_dir();
            let homedirstring = homedir.to_str().unwrap();

            let appdatalocal_dir = format!("{}{}", homedirstring, "\\AppData\\Local\\");
            env::set_current_dir(&appdatalocal_dir);

            self.info_windows_files_list_appdata_local_temp_directory_files = format!("files under directory {}\\Temp\\ ... ",appdatalocal_dir).cyan().bold().to_string();

            for entry in WalkDir::new("Temp").into_iter().filter_map(|e| e.ok()) {

                if entry.metadata().unwrap().is_dir() {
                    self.info_windows_files_list_appdata_local_temp_directory_files.push_str(
                        format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                    );
                } else if entry.metadata().unwrap().is_file() {
                    self.info_windows_files_list_appdata_local_temp_directory_files.push_str(
                        format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                    );
                }
            }
        }
    }
}

impl probewindowsfiles {

    pub fn probe_windows_files_list_temp_directory_files(&mut self) {
        
        //return;

        env::set_current_dir("C:\\Windows\\");
        self.info_windows_files_list_temp_directory_files = format!("files under directory C:\\Windows\\Temp ... \n").cyan().bold().to_string();

        for entry in WalkDir::new("Temp").into_iter().filter_map(|e| e.ok()) {

            if entry.metadata().unwrap().is_dir() {
                self.info_windows_files_list_temp_directory_files.push_str(
                    format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            } else if entry.metadata().unwrap().is_file() {
                self.info_windows_files_list_temp_directory_files.push_str(
                    format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            }
        }
    }
}

impl probewindowsfiles {

    pub fn probe_windows_files_list_desktop_directory_files(&mut self) {
        
        //return;

        if let Some(user_dirs) = UserDirs::new() {

            let desktopdir = user_dirs.desktop_dir();
            let desktopdirstring = user_dirs.desktop_dir().unwrap().to_string_lossy();

            let desktopdirstring_dir = format!("{}", desktopdirstring);
            env::set_current_dir(&desktopdirstring_dir);

            match desktopdir {
                Some(dir) => {
                    self.info_windows_files_list_desktop_directory_files = format!("files under directory {} ... \n", desktopdir.unwrap().to_string_lossy()).cyan().bold().to_string();

                    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
                
                        if entry.metadata().unwrap().is_dir() {
                            self.info_windows_files_list_desktop_directory_files.push_str(
                            format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                            );

                        } else if entry.metadata().unwrap().is_file() {
                            self.info_windows_files_list_desktop_directory_files.push_str(
                            format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                            );
                        }
                    }
                },
                None => println!("error"),
            }
        }
    }
}

impl probewindowsfiles {

    pub fn probe_windows_files_list_pfiles_directory_files(&mut self) {
        
        //return;

        env::set_current_dir("C:\\Program Files\\");
        self.info_windows_files_list_pfiles_directory_files = format!("files under directory C:\\Program Files\\ ... \n").cyan().bold().to_string();

        for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {

            if entry.metadata().unwrap().is_dir() {
                self.info_windows_files_list_pfiles_directory_files.push_str(
                    format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            } else if entry.metadata().unwrap().is_file() {
                self.info_windows_files_list_pfiles_directory_files.push_str(
                    format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            }
        }
    }
}

impl probewindowsfiles {

    pub fn probe_windows_files_list_pfiles86_directory_files(&mut self) {
        
        //return;

        env::set_current_dir("C:\\Program Files (x86)\\");
        self.info_windows_files_list_pfiles86_directory_files = format!("files under directory C:\\Program Files(x86)\\ ... \n").cyan().bold().to_string();

        for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {

            if entry.metadata().unwrap().is_dir() {
                self.info_windows_files_list_pfiles86_directory_files.push_str(
                    format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            } else if entry.metadata().unwrap().is_file() {
                self.info_windows_files_list_pfiles86_directory_files.push_str(
                    format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            }
        }
    }
}

impl probewindowsfiles {

    pub fn probe_windows_files_list_windows_directory_files(&mut self) {
        
        env::set_current_dir("C:\\Windows\\");
        self.info_windows_files_list_windows_directory_files = format!("files under directory C:\\Windows\\ ... \n").cyan().bold().to_string();

        for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {

            if entry.metadata().unwrap().is_dir() {
                self.info_windows_files_list_windows_directory_files.push_str(
                    format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            } else if entry.metadata().unwrap().is_file() {
                self.info_windows_files_list_windows_directory_files.push_str(
                    format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            }
        }
    }
}

impl probewindowsfiles {

    pub fn probe_windows_files_list_system32_directory_files(&mut self) {
        
        env::set_current_dir("C:\\Windows\\");
        self.info_windows_files_list_system32_directory_files = format!("files under directory C:\\Windows\\System32\\ ... \n").cyan().bold().to_string();

        for entry in WalkDir::new("System32").into_iter().filter_map(|e| e.ok()) {

            if entry.metadata().unwrap().is_dir() {
                self.info_windows_files_list_system32_directory_files.push_str(
                    format!("dir: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            } else if entry.metadata().unwrap().is_file() {
                self.info_windows_files_list_system32_directory_files.push_str(
                    format!("file: {}\n", entry.path().to_str().unwrap()).as_str()
                );
            }
        }
    }
}

pub fn windows_test() {

    println!("{}" , format!("windows test ... ").cyan().bold(), );
}

