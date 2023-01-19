use chrono::prelude::*;
use colored::*;
use std::fs::File;
use std::io::Write;
use std::io::ErrorKind::*;

fn handle_logfile(fpath: &str) -> File {
    match File::options().append(true).open(fpath) {
        Ok(file)=> { return file },
        Err(e)  => {
            if e.kind() == PermissionDenied || e.kind() == InvalidInput {
                print_error(format!("{:?}", e).as_str())
            }
            let date = Local::now().date_naive();
            print_warning(format!("Using default ~/.rprxy/log{:?}", date).as_str());
            match File::create(std::path::Path::new(fpath)) {
                Ok(file) => { return file },
                Err(e) => { print_error(format!("{:?}", e).as_str()) }
            }

            print_warning("Using fallback logfile: /tmp/rprxy.log");
            return File::options().append(true).open("/tmp/rprxy.log").unwrap()
        }
    }
}

#[allow(unused_must_use)]
pub fn log_message(s: &str, fpath: &str) {
    let mut logfile = handle_logfile(fpath);
    let dt: DateTime<Local> = Local::now();
    let timestamp = format!("{}:{}:{}", dt.hour(), dt.minute(), dt.second());
    let msg = format!("{} {}", timestamp, s);
    
    logfile.write(msg.as_bytes());
}

pub fn print_info(s: &str) {
    println!("{}", format!("[I]: {}", s).italic().dimmed().white());
}

pub fn print_error(s: &str) {
    println!("{}", format!("[-]: {}", s).red());
}

pub fn print_success(s: &str) {
    println!("{}", format!("[+]: {}", s).green());
}

pub fn print_warning(s: &str) {
    println!("{}", format!("[W]: {}", s).yellow());
}
