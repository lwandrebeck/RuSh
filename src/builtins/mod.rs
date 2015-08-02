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
use std::path::{Path, PathBuf};

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
pub fn etest(command: &[&str]) {
	unimplemented!();
}

pub fn bi_if(command: &[&str]) {
	unimplemented!();
}

pub fn test(command: &[&str]) {
	unimplemented!();
}

// ==================
// == I/O builtins ==
// ==================

pub fn echo(command: &[&str]) {
    unimplemented!();
}

pub fn printf(command: &[&str]) {
    unimplemented!();
}

pub fn read(command: &[&str]) {
    unimplemented!();
}

// =========================
// == Filesystem builtins ==
// =========================

pub fn cd(command: &[&str]) {
    let current = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            return;
        }
    };

    // cd is the "change directory" command. It can take either 0 or 1
    // arguments. If given no arguments, then the $HOME directory is
    // chosen.
    let dir : PathBuf = match command.len() {
        0 => {
            match env::home_dir() {
                Some(home) => home,
                None => PathBuf::from("/")
            }
        },
        1 if command[0] == "-" => {
            PathBuf::from(env::var("OLD_PWD").unwrap_or(".".to_owned()))
        },
        1 => {
            let attr = match fs::metadata(command[0]) {
                Ok(s) => s,
                Err(e) => return
            };
            if attr.is_dir() {
                PathBuf::from(command[0])
            } else {
                current
            }
        }
        _ => {
            println!("Invalid parameter number");
            return;
        },
    };
    env::set_var("OLD_PWD", env::current_dir().unwrap());
    env::set_current_dir(&dir);
    env::set_var("PWD", env::current_dir().unwrap());
}

pub fn pushd(command: &[&str]) {
    unimplemented!();
}

pub fn popd(command: &[&str]) {
    unimplemented!();
}

pub fn dirs(command: &[&str]) {
    unimplemented!();
}

// ========================
// == Variables builtins ==
// ========================

pub fn bi_let(command: &[&str]) {
    unimplemented!();
}

pub fn eval(command: &[&str]) {
    unimplemented!();
}

pub fn set(command: &[&str]) {
    unimplemented!();
}

pub fn unset(command: &[&str]) {
    unimplemented!();
}

pub fn export(command: &[&str]) {
    unimplemented!();
}

pub fn declare(command: &[&str]) {
    unimplemented!();
}

pub fn typeset(command: &[&str]) {
    unimplemented!();
}

pub fn readonly(command: &[&str]) {
    unimplemented!();
}

pub fn getopts(command: &[&str]) {
    unimplemented!();
}

pub fn source(command: &[&str]) {
    unimplemented!();
}

pub fn exec(command: &[&str]) {
    unimplemented!();
}

pub fn shopt(command: &[&str]) {
    unimplemented!();
}

pub fn caller(command: &[&str]) {
    unimplemented!();
}

// =======================
// == Commands builtins ==
// =======================

pub fn bi_true(command: &[&str]) {
    unimplemented!();
}

pub fn bi_false(command: &[&str]) {
    unimplemented!();
}

pub fn bi_type(command: &[&str]) {
    unimplemented!();
}

pub fn hash(command: &[&str]) {
    unimplemented!();
}

pub fn bind(command: &[&str]) {
    unimplemented!();
}

pub fn help(command: &[&str]) {
    unimplemented!();
}

// ==========================
// == job control commands ==
// ==========================

pub fn jobs(command: &[&str]) {
    unimplemented!();
}

pub fn disown(command: &[&str]) {
    unimplemented!();
}

pub fn fg(command: &[&str]) {
    unimplemented!();
}

pub fn bg(command: &[&str]) {
    unimplemented!();
}

pub fn wait(command: &[&str]) {
    unimplemented!();
}

pub fn suspend(command: &[&str]) {
    unimplemented!();
}

pub fn logout(command: &[&str]) {
    unimplemented!();
}

pub fn times(command: &[&str]) {
    unimplemented!();
}

// not a builtin !
/*pub fn kill(command: &[&str]) {
    unimplemented!();
}*/

pub fn killall(command: &[&str]) {
    unimplemented!();
}

pub fn command(command: &[&str]) {
    unimplemented!();
}

pub fn builtin(command: &[&str]) {
    unimplemented!();
}

pub fn enable(command: &[&str]) {
    unimplemented!();
}

pub fn autoload(command: &[&str]) {
    unimplemented!();
}

// =============================================================
//  == BONUS. Additionnal other commands running as builtins. ==
// =============================================================

pub fn pwd(command: &[&str]) {
    let current = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("no current dir !? {}", e)
    };
    let p = match command.len() {
        0 => &current,
        _ => {
            if command[1] == "-P" {
                &current
            } else {
                panic!("non supported argument")
            }
        }
    };
    println!("{}", current.to_str().unwrap_or(""));
}

pub fn chown(command: &[&str]) {
    unimplemented!();
}

pub fn chmod(command: &[&str]) {
    unimplemented!();
}

pub fn mkdir(command: &[&str]) {
    unimplemented!();
}

pub fn rmdir(command: &[&str]) {
    unimplemented!();
}

pub fn touch(command: &[&str]) {
    unimplemented!();
}

pub fn ln(command: &[&str]) {
    unimplemented!();
}
