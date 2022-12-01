// 0.16

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



pub fn windows_test() {

    println!("{}" , format!("windows test ... ").cyan().bold(), );


}

