/*
 * builtins.rs
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
use std::env;
use std::convert::From;
use std::fs;
use std::path::PathBuf;


// println_stderr is like println, but to stderr.
//macro_rules! println_stderr(
//    ($($arg:tt)*) => (
//        match writeln!(&mut io::stdio::stderr(), $($arg)* ) {
//            Ok(_) => {},
//            Err(x) => panic!("Unable to write to stderr: {}", x),
//        }
//    )
//);

// tests, conditionnal builtins
pub fn etest(command: &Vec<&str>) {
	unimplemented!();
}

pub fn bi_if(command: &Vec<&str>) {
	unimplemented!();
}

pub fn test(command: &Vec<&str>) {
	unimplemented!();
}

// I/O builtins

pub fn echo(command: &Vec<&str>) {
    unimplemented!();
}

pub fn printf(command: &Vec<&str>) {
    unimplemented!();
}

pub fn read(command: &Vec<&str>) {
    unimplemented!();
}

// Filesystem builtins

pub fn cd(command: &Vec<&str>) {
    // cd is the "change directory" command. It can take either 0 or 1
    // arguments. If given no arguments, then the $HOME directory is
    // chosen.
    let homedir = match env::home_dir() {
        Some(home) => home,
        None => PathBuf::from("/")
    };
    let current = match env::current_dir() {
        Ok(path) => path,
        Err(e) => homedir.clone()
    };
    let previous = match env::var("OLDPWD") {
        Ok(oldpwd) => PathBuf::from(oldpwd),
        Err(e) => homedir.clone()
    };
    let attr = match fs::metadata(command[1]) {
        Ok(s) => s,
        Err(e) => return
    };
    let mut next;
    if attr.is_dir() {
        next = PathBuf::from(command[1]);
    } else {
        next = current.clone();
    }

    let dir = match command.len() {
        0 => panic!("invalid cd invocation"),
        1 => { env::set_var("OLDPWD", &current);
               env::set_var("PWD", &homedir);
               homedir.clone()
             },
        _ => { if command[1] == "-" {
                    env::set_var("PWD", &previous);
                    env::set_var("OLDPWD", &current);
                    previous
               } else {
                   env::set_var("OLDPWD", &current);
                   env::set_var("PWD", &next);
                   next
               }
             },
    };
    if dir.eq(&PathBuf::from("")) {
        println!("cd: no directory to change to");
        return;
    }
    let result = env::set_current_dir(&dir);
    match result {
        Err(err) => {
            println!("cd: {:?}: {}", dir, err);
        },
        _ => {},
    }
}

pub fn pushd(command: &Vec<&str>) {
    unimplemented!();
}

pub fn popd(command: &Vec<&str>) {
    unimplemented!();
}

pub fn dirs(command: &Vec<&str>) {
    unimplemented!();
}

// Variables builtins

pub fn bi_let(command: &Vec<&str>) {
    unimplemented!();
}

pub fn eval(command: &Vec<&str>) {
    unimplemented!();
}

pub fn set(command: &Vec<&str>) {
    unimplemented!();
}

pub fn unset(command: &Vec<&str>) {
    unimplemented!();
}

pub fn export(command: &Vec<&str>) {
    unimplemented!();
}

pub fn declare(command: &Vec<&str>) {
    unimplemented!();
}

pub fn typeset(command: &Vec<&str>) {
    unimplemented!();
}

pub fn readonly(command: &Vec<&str>) {
    unimplemented!();
}

pub fn getopts(command: &Vec<&str>) {
    unimplemented!();
}

pub fn source(command: &Vec<&str>) {
    unimplemented!();
}

pub fn exit(command: &Vec<&str>) {
    unimplemented!();
}

pub fn exec(command: &Vec<&str>) {
    unimplemented!();
}

pub fn shopt(command: &Vec<&str>) {
    unimplemented!();
}

pub fn caller(command: &Vec<&str>) {
    unimplemented!();
}

// Commands builtins

pub fn bi_true(command: &Vec<&str>) {
    unimplemented!();
}

pub fn bi_false(command: &Vec<&str>) {
    unimplemented!();
}

pub fn bi_type(command: &Vec<&str>) {
    unimplemented!();
}

pub fn hash(command: &Vec<&str>) {
    unimplemented!();
}

pub fn bind(command: &Vec<&str>) {
    unimplemented!();
}

pub fn help(command: &Vec<&str>) {
    unimplemented!();
}

// job control commands

pub fn jobs(command: &Vec<&str>) {
    unimplemented!();
}

pub fn disown(command: &Vec<&str>) {
    unimplemented!();
}

pub fn fg(command: &Vec<&str>) {
    unimplemented!();
}

pub fn bg(command: &Vec<&str>) {
    unimplemented!();
}

pub fn wait(command: &Vec<&str>) {
    unimplemented!();
}

pub fn suspend(command: &Vec<&str>) {
    unimplemented!();
}

pub fn logout(command: &Vec<&str>) {
    unimplemented!();
}

pub fn times(command: &Vec<&str>) {
    unimplemented!();
}

pub fn kill(command: &Vec<&str>) {
    unimplemented!();
}

pub fn killall(command: &Vec<&str>) {
    unimplemented!();
}

pub fn command(command: &Vec<&str>) {
    unimplemented!();
}

pub fn builtin(command: &Vec<&str>) {
    unimplemented!();
}

pub fn enable(command: &Vec<&str>) {
    unimplemented!();
}

pub fn autoload(command: &Vec<&str>) {
    unimplemented!();
}

// BONUS. Additionnal other commands running as builtins.

pub fn pwd(command: &Vec<&str>) {
    let current = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("no current dir !? {}",e)
    };
    let p = match command.len() {
        0 => panic!("invalid pwd invocation"),
        1 => &current,
        _ => if command[1] == "-P" {
                &current
             } else {
                panic!("non supported argument")
             }
    };
    println!("{:?}", current);
}

pub fn chown(command: &Vec<&str>) {
    unimplemented!();
}

pub fn chmod(command: &Vec<&str>) {
    unimplemented!();
}

pub fn mkdir(command: &Vec<&str>) {
    unimplemented!();
}

pub fn rmdir(command: &Vec<&str>) {
    unimplemented!();
}

pub fn touch(command: &Vec<&str>) {
    unimplemented!();
}

pub fn ln(command: &Vec<&str>) {
    unimplemented!();
}

