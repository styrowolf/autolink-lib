extern crate serde;
pub extern crate chrono;
extern crate serde_json;
extern crate clap;
use std::process::Command;
use std::convert::AsRef;
use std::fmt;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::env;
use serde::{Serialize, Deserialize};
use chrono::{Datelike, Timelike};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone)]
pub struct Plan {
    pub name: String,
    pub link: String,
    pub times: Vec<TimeDay>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TimeDay {
    pub time: chrono::NaiveTime,
    pub day: chrono::Weekday,
}

impl TimeDay {
    pub fn new(t: chrono::NaiveTime, d: chrono::Weekday) -> Self {
        TimeDay {
            time: t,
            day: d,
        }
    }
}

impl fmt::Display for TimeDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "time: {}, day: {},\n", self.time, self.day)
    }
}

impl Plan {
    pub fn new(n: String, l: String, t: Vec<TimeDay>) -> Self {
        Plan {
            name: n,
            link: l,
            times: t,
        }
    }

    pub fn new_user_friendly(n: &str, l: &str, t: &str, d: &str) -> Self {
        let time = chrono::NaiveTime::parse_from_str(&t, "%H:%M").expect("date cannot be parsed");
        let day = match d.to_lowercase().as_str() {
            "monday" => chrono::Weekday::Mon,
            "tuesday" => chrono::Weekday::Tue,
            "wednesday" => chrono::Weekday::Wed,
            "thursday" => chrono::Weekday::Thu,
            "friday" => chrono::Weekday::Fri,
            "saturday" => chrono::Weekday::Sat,
            "sunday" => chrono::Weekday::Sun,
            _ => panic!("undefined day")
        };

        let mut tdv = Vec::new();
        tdv.push(TimeDay::new(time, day));
        
        Self::new(n.to_string(), l.to_string(), tdv)
    }

    pub fn remove_matching_time(&mut self, td: &TimeDay) {
        let length = self.times.len();
        for i in 0..length {
            if self.times.get(i).unwrap() == td {
                self.times.remove(i);
                break
            }
        }
    }
}

impl fmt::Display for Plan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut times = String::new();
        for t in self.times.clone() {
            times += format!("{}", &t).as_str();
        }
        write!(f, "name: {},\n\ntimes:\n{}\nlink: {}\n", self.name, times, self.link)
    }
}

pub fn export<T: AsRef<Path>>(v: Vec<Plan>, p: T) {
    let mut file = OpenOptions::new().write(true).open(p).expect("error in opening config file"); //File::open(p).expect("error in opening config file");
    let _ = file.set_len(0);
    let _ = file.write(serde_json::to_string(&v).expect("error in serializing").as_bytes());
}

pub fn import<T: AsRef<Path>>(p: T) -> Vec<Plan> {
    let file = File::open(&p);
    let mut file = match file.is_err() {
        true => {
            let _ = File::create(&p);
            File::open(&p).unwrap()
        },
        false => file.unwrap(),
    };

    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);
    if buf.len() == 0 {
        Vec::new()
    } else {
        serde_json::from_slice(&buf).expect("error while parsing config file")
    }
}

pub fn check(p: &Plan, td: &TimeDay) -> bool {
    let mut result = false;

    for t in &p.times {
        if td == t {
            open_link(&p.link);
            result |= true
        } else {
            result |= false
        }
    }

    result

}

pub fn check_all(v: &Vec<Plan>) {
    let mut cv = v.clone();
    loop { 
        let day = chrono::Local::now().naive_local().date().weekday();
        let time = chrono::Local::now().naive_local().time();
        let time = chrono::NaiveTime::from_hms(time.hour(), time.minute(), 0);
        let timeday = TimeDay::new(time, day);
        let length = cv.len();
        for i in 0..length {
            if check(cv.get(i).unwrap(), &timeday) {
                let mut p = cv.get(i).unwrap().clone();
                p.remove_matching_time(&timeday);
                cv.remove(i);
                cv.push(p);
                break
            }
        }

        let mut times = 0;
        for p in &cv {
            times += p.times.len();
        }
        if times == 0 {
            cv = v.clone()
        }
        std::thread::sleep(std::time::Duration::new(5, 0))
    }
    
}

#[cfg(target_os = "macos")]
pub fn open_link(z: &String) {
    Command::new("open").arg(z).spawn().expect("error in opening link using open");
}

#[cfg(all(not(target_os = "macos"), target_family = "unix"))]
pub fn open_link(z: &String) {
    Command::new("xdg-open").arg(z).spawn().expect("error in opening link using xdg-open");
}

#[cfg(target_os = "windows")]
pub fn open_link(z: &String) {
    Command::new("start").arg(z).spawn().expect("error in opening link using start");
}
