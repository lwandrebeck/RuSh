/*
 * config.rs
 *
 * Copyright 2015 Laurent Wandrebeck <l.wandrebeck@quelquesmots.fr>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 * MA 02110-1301, USA.
 *
 *
 */

extern crate chrono;
extern crate libc;

use self::chrono::*;
use self::libc::{c_char, c_int, size_t, funcs};
use std::ffi::CStr;
use std::{env,  io, str};
use std::path::PathBuf;

pub fn init_env() {
    let mut bufc = vec![0u8; 40];
    unsafe {
        let id = funcs::posix88::unistd::getuid();
        env::set_var("UID", id.to_string());
    }
    unsafe {
        let id = funcs::posix88::unistd::getgid();
        env::set_var("GID", id.to_string());
    }
    env::set_var("PWD", env::current_dir().unwrap_or(PathBuf::from("/")));
    unsafe {
        let log = funcs::posix88::unistd::getlogin();
        env::set_var("USERNAME",
            String::from_utf8(CStr::from_ptr(log).to_bytes().to_owned()).unwrap_or("no login".to_owned()));
    }
    extern "C" {
        fn gethostname(bufc: *mut c_char, len: size_t) -> c_int;
    }
    unsafe {
        gethostname(bufc.as_mut_ptr() as *mut c_char, 40);
    };
    env::set_var("HOSTNAME", str::from_utf8(bufc.split(|x| *x == 0).next().unwrap()).unwrap_or("wtf"));
}

pub fn load_config() {
    match env::home_dir() {
        Some(ref p) => println!("{}", p.display()),
        None => println!("Impossible to get your home dir!")
    }
}

pub fn prompt(case: u8) -> String {
    let mut p = String::new();
    let mut aslash = false;
    match case {
        1 => {
            let ps1 = match env::var("PS1") {
                Ok(r) => { r }
                Err(e) => { env::set_var("PS1", "[\\u@\\h \\W]\\$ "); String::from("[\\u@\\h \\W]\\$ ") }
            };
            let mut pr: Vec<(usize, char)> = ps1.char_indices().collect();
            for i in pr {
                if i.1 == '\\' {
                    aslash = true;
                    continue;
                }
                if aslash {
                    aslash = false;
                    match i {
						// See http://ss64.com/bash/syntax-prompt.html
                        (index, 'd') => { let dt = Local::now(); p.push_str(&dt.format("%a %b %e").to_string()); },
                        (index, 'h') => p.push_str(&env::var("HOSTNAME").unwrap_or("localhost".to_owned()).split('.').next().unwrap()),
                        (index, 'H') => p.push_str(&env::var("HOSTNAME").unwrap_or("localhost.localdomain".to_owned())),
                        (index, 'j') => unimplemented!(),
                        (index, 'l') => p.push_str(&env::var("TERM").unwrap_or("unknown term".to_owned())),
                        (index, 's') => p.push_str(&env::var("0").unwrap_or("rush".to_owned()).split('/').last().unwrap()),
                        (index, 't') => { let dt = Local::now(); p.push_str(&dt.format("%H:%M:%S").to_string()); },
                        (index, 'T') => { let dt = Local::now(); p.push_str(&dt.format("%I:%M:%S").to_string()); },
                        (index, '@') => { let dt = Local::now(); p.push_str(&dt.format("%I:%M:%S%P").to_string()); },
                        (index, 'u') => p.push_str(&env::var("USERNAME").unwrap_or("unknown user".to_owned())),
                        (index, 'v') => p.push_str("0.0.1"), // FIXME
                        (index, 'V') => p.push_str("0.0.1"), // FIXME
                        (index, 'w') => p.push_str(&env::var("PWD").unwrap()),
                        (index, 'W') => p.push_str(&env::var("PWD").unwrap().split('/').last().unwrap()),
                        (index, '!') => unimplemented!(),
                        (index, '#') => unimplemented!(),
                        (index, '$') => match &env::var("UID").unwrap()[..] {
                            "0" => p.push_str("#"),
                            _ => p.push_str("$"), },
                        (index, '0'...'8') => unimplemented!(),
                        (index, 'n') => p.push_str("\n"),
                        (index, 'r') => p.push_str("\r"),
                        (index, 'e') => unimplemented!(),
                        (index, 'a') => unimplemented!(),
                        (index, '\\') => p.push_str("\\"),
                        (index, '[') => unimplemented!(),
                        (index, ']') => unimplemented!(),
                        (_, _) => continue,
                    }
                } else {
                    p.push(i.1);
                }
            }
            return p;
        }
        2 => {
            match env::var("PS2") {
                Ok(ps2) => { return ps2; }
                Err(e) => { let ps2 = String::from(">"); env::set_var("PS2", ps2); return String::from(">"); }
            };
        }
        3 => {
            match env::var("PS3") {
                Ok(ps3) => { return ps3; }
                Err(e) => { let ps3 = String::from(">"); env::set_var("PS3", ps3); return String::from(">"); }
            };
        }
        4 => {
            match env::var("PS4") {
                Ok(ps4) => { return ps4; }
                Err(e) => { let ps4 = String::from(">"); env::set_var("PS4", ps4); return String::from(">"); }
            };
        }
        _ => panic!("line_case value that should not have happened"),
    }
}
