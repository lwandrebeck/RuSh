/*
 * builtins.rs
 *
 * Copyright 2015-2016 Laurent Wandrebeck <l.wandrebeck@quelquesmots.fr>
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

extern crate linenoise;

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

// =========================================
// == nothing builtin. Yes, it does exist ==
// =========================================

pub fn colon(command: &[&str]) -> bool {
    // yes, it really does nothing, but always return true.
    true
}

// =======================================================
// == history builtins - works only in interactive mode ==
// =======================================================

pub fn fc(command: &[&str]) {
    unimplemented!();
}

pub fn history(command: &[&str]) {
    unimplemented!();
}

// ==================================
// == tests, conditionnal builtins ==
// ==================================

pub fn and(command: &[&str]) {
    unimplemented!();
}

pub fn bi_if(command: &[&str]) {
    unimplemented!();
}

pub fn dand(command: &[&str]) {
    unimplemented!();
}

pub fn dbracket(command: &[&str]) {
    // equivalent to etest
    unimplemented!();
}

pub fn dpipe(command: &[&str]) {
    unimplemented!();
}

pub fn etest(command: &[&str]) {
    unimplemented!();
}

pub fn obrace(command: &[&str]) {
    unimplemented!();
}

pub fn obracket(command: &[&str]) {
    // equivalent to test
    unimplemented!();
}

pub fn pipe(command: &[&str]) {
    unimplemented!();
}

pub fn test(command: &[&str]) {
    //equivalent to [ ]
    unimplemented!();
}

// ==================
// == I/O builtins ==
// ==================

pub fn clear(command: &[&str]) {
    linenoise::clear_screen();
}

pub fn dgtsign(command: &[&str]) {
    unimplemented!();
}

pub fn dltsign(command: &[&str]) {
    unimplemented!();
}

pub fn echo(command: &[&str]) {
    // FIXME echo must be able to parse several args. use iter ?
    match command.len() {
        0 => println!(""),
        1 => if command[0].starts_with("$") {
                let var = env::var(&command[0][1..]).unwrap_or("".to_owned());
                println!("{}", var);
            } else {
                println!("{}", command[0]);
            },
        2 => match command[0] {
                "-n" => if command[1].starts_with("$") {
                            let var = env::var(&command[1][1..]).unwrap_or("".to_owned());
                            print!("{}", var);
                        } else {
                            print!("{}", command[1]);
                        },
                "-e" => panic!(),
                "-E" => panic!(),
                _    => panic!(),
             },
        _ => panic!(),
    }
}

pub fn gtsign(command: &[&str]) {
    unimplemented!();
}

pub fn ltsign(command: &[&str]) {
    unimplemented!();
}

pub fn mapfile(command: &[&str]) {
    // equivalent to readarray
    unimplemented!();
}

pub fn printf(command: &[&str]) {
    unimplemented!();
}

pub fn read(command: &[&str]) {
    unimplemented!();
}

pub fn readarray(command: &[&str]) {
    // equivalent to mapfile
    unimplemented!();
}

pub fn umask(command: &[&str]) {
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
            PathBuf::from(env::var("OLDPWD").unwrap_or(".".to_owned()))
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
    env::set_var("OLDPWD", env::current_dir().unwrap());
    env::set_current_dir(&dir);
    env::set_var("PWD", dir.into_os_string());
}

pub fn dirs(command: &[&str]) {
    unimplemented!();
}

pub fn popd(command: &[&str]) {
    unimplemented!();
}

pub fn pushd(command: &[&str]) {
    unimplemented!();
}

// ========================
// == Variables builtins ==
// ========================

pub fn bi_let(command: &[&str]) {
    // equivalent to (( ))
    unimplemented!();
}

pub fn caller(command: &[&str]) {
    unimplemented!();
}

pub fn declare(command: &[&str]) {
    unimplemented!();
}

pub fn dparenthesis(command: &[&str]) {
    // equivalent to let
    unimplemented!();
}

pub fn dquote(command: &[&str]) {
    unimplemented!();
}

pub fn eval(command: &[&str]) {
    unimplemented!();
}

pub fn exec(command: &[&str]) {
    unimplemented!();
}

pub fn export(command: &[&str]) {
    unimplemented!();
}

pub fn getopts(command: &[&str]) {
    unimplemented!();
}

pub fn local(command: &[&str]) {
    // for use only in functions
    unimplemented!();
}

pub fn readonly(command: &[&str]) {
    unimplemented!();
}

pub fn set(command: &[&str]) {
    unimplemented!();
}

pub fn shift(command: &[&str]) {
    unimplemented!();
}

pub fn shopt(command: &[&str]) {
    unimplemented!();
}

pub fn source(command: &[&str]) {
    // equivalent to .
    unimplemented!();
}

pub fn squote(command: &[&str]) {
    unimplemented!();
}

pub fn typeset(command: &[&str]) {
    unimplemented!();
}

pub fn unset(command: &[&str]) {
    // unset var first if it exists, unset function if no such var exists.
    unimplemented!();
}

// =======================
// == Commands builtins ==
// =======================

pub fn alias(command: &[&str]) {
    unimplemented!();
}

pub fn backtick(command: &[&str]) {
    unimplemented!();
}

pub fn bi_break(command: &[&str]) {
    unimplemented!();
}

pub fn bi_continue(command: &[&str]) {
    unimplemented!();
}

pub fn bi_false(command: &[&str]) {
    unimplemented!();
}

pub fn bi_for(command: &[&str]) {
    unimplemented!();
}

pub fn bi_return(command: &[&str]) {
    unimplemented!();
}

pub fn bi_true(command: &[&str]) {
    unimplemented!();
}

pub fn bi_type(command: &[&str]) {
    unimplemented!();
}

pub fn bi_while(command: &[&str]) {
    unimplemented!();
}

pub fn bind(command: &[&str]) {
    unimplemented!();
}

pub fn case(command: &[&str]) {
    unimplemented!();
}

pub fn coproc(command: &[&str]) {
    unimplemented!();
}

pub fn expr(command: &[&str]) {
    unimplemented!();
}

pub fn function(command: &[&str]) {
    unimplemented!();
}

pub fn hash(command: &[&str]) {
    unimplemented!();
}

pub fn help(command: &[&str]) {
    unimplemented!();
}

pub fn select(command: &[&str]) {
    unimplemented!();
}

pub fn unalias(command: &[&str]) {
    unimplemented!();
}

pub fn until(command: &[&str]) {
    unimplemented!();
}

// ==========================
// == job control commands ==
// ==========================

pub fn autoload(command: &[&str]) {
    unimplemented!();
}

pub fn bg(command: &[&str]) {
    unimplemented!();
}

pub fn builtin(command: &[&str]) {
    unimplemented!();
}

pub fn command(command: &[&str]) {
    unimplemented!();
}

pub fn disown(command: &[&str]) {
    unimplemented!();
}

pub fn enable(command: &[&str]) {
    unimplemented!();
}

pub fn exit(command: &[&str]) {
    unimplemented!();
}

pub fn fg(command: &[&str]) {
    unimplemented!();
}

pub fn jobs(command: &[&str]) {
    unimplemented!();
}

pub fn job_spec(command: &[&str]) {
    unimplemented!();
}

pub fn kill(command: &[&str]) {
    unimplemented!();
}

pub fn logout(command: &[&str]) {
    unimplemented!();
}

pub fn suspend(command: &[&str]) {
    unimplemented!();
}

pub fn time(command: &[&str]) {
    unimplemented!();
}

pub fn times(command: &[&str]) {
    unimplemented!();
}

pub fn trap(command: &[&str]) {
    unimplemented!();
}

pub fn ulimit(command: &[&str]) {
    unimplemented!();
}

pub fn wait(command: &[&str]) {
    unimplemented!();
}

// =========================
// == Completion commands ==
// =========================

pub fn compgen(command: &[&str]) {
    // intended for use from a shell function only
    unimplemented!();
}

pub fn complete(command: &[&str]) {
    unimplemented!();
}

pub fn compopt(command: &[&str]) {
    unimplemented!();
}

// =============================================================
//  == BONUS. Additionnal other commands running as builtins. ==
// =============================================================

/* Not BUILTINS !!!
pub fn chmod(command: &[&str]) {
    unimplemented!();
}

pub fn chown(command: &[&str]) {
    unimplemented!();
}

pub fn killall(command: &[&str]) {
    unimplemented!();
}

pub fn ln(command: &[&str]) {
    unimplemented!();
}

pub fn mkdir(command: &[&str]) {
    unimplemented!();
}*/

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
                panic!("not supported argument")
            }
        }
    };
    println!("{}", current.to_str().unwrap_or(""));
}

/* Not BUILTINS !!!
pub fn rmdir(command: &[&str]) {
    unimplemented!();
}

pub fn touch(command: &[&str]) {
    unimplemented!();
}*/
