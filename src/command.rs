/*
 * command.rs
 *
 * Copyright 2015 Laurent Wandrebeck <l.wandrebeck@quelquesmots.fr>
 * Copyright 2015 Guillaume Gomez <guillaume1.gomez@gmail.com>
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

use std::path::{Path, PathBuf};
use std::fs::PathExt;
use std::env;
use libc::funcs::posix88::unistd::{fork, wait, execve};
use libc::funcs::c95::stdio::perror;
use std::ffi::CString;
use std::ptr::null;

pub fn execute_line(command: &str, arguments: &[&str]) {
    if Path::new(command).exists() {
        execute_command(Path::new(command).to_path_buf(), arguments)
    } else {
        match search_command(command) {
            Some(p) => {
                execute_command(p, arguments)
            }
            None => {
                println!("Command not found: \"{}\"", command);
            }
        }
    }
}

pub fn search_command(command: &str) -> Option<PathBuf> {
    let path : String = env::var("PATH").unwrap_or("".to_owned());
    let paths : Vec<&str> = path.split(":").collect();

    for path in paths {
        let p = Path::new(path).join(command);

        if p.exists() {
            return Some(p);
        }
    }
    None
}

pub fn execute_command(command_path: PathBuf, arguments: &[&str]) {
    let mut v : Vec<CString> = Vec::new();
    let mut a : Vec<CString> = Vec::new();

    for (variable, value) in env::vars() {
        v.push(CString::new(format!("{}={}", variable, value)).unwrap());
    }
    let mut c_v = Vec::new();
    for tmp in v.iter() {
        c_v.push(tmp.as_ptr());
    }
    c_v.push(null());

    for arg in arguments.iter() {
        a.push(CString::new(*arg).unwrap());
    }
    let mut c_a = Vec::new();
    for tmp in a.iter() {
        c_a.push(tmp.as_ptr());
    }
    c_a.push(null());
    unsafe {
        let mut pid = fork();

        if pid == -1 {
            println!("An error occured...");
        } else if pid != 0 {
            // parent process
            wait(&mut pid);
        } else {
            // child process
            let mut c = CString::new(command_path.as_path().to_str().unwrap()).unwrap();

            execve(c.as_ptr() as *const i8, c_a.as_mut_ptr() as *mut *const i8, c_v.as_mut_ptr() as *mut *const i8);
        }
    }
}