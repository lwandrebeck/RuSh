/*
 * builtins.rs
 *
 * Copyright 2015-2017 Laurent Wandrebeck <l.wandrebeck@quelquesmots.fr>
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

//! Builtins of RuSh.
//!
//! This is where every builtin is implemented.
//! History, tests, conditionnals, I/O, filesystem, variables, commands, job control, completion.

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

// nothing builtin

/// nothing builtin. Yes, it does exist
pub fn colon(command: &[&str]) -> bool {
    // yes, it really does nothing, but always return true.
    true
}

// history builtins

/// fc history builtin - works only in interactive mode
pub fn fc(command: &[&str]) {
    unimplemented!();
}

/// history history builtin - works only in interactive mode
pub fn history(command: &[&str]) {
    unimplemented!();
}

// tests, conditionnal builtins

/// & builtin
pub fn and(command: &[&str]) {
    unimplemented!();
}

/// if builtin
pub fn bi_if(command: &[&str]) {
    unimplemented!();
}

/// && builtin
pub fn dand(command: &[&str]) {
    unimplemented!();
}

/// [[ builtin (equivalent to etest
pub fn dbracket(command: &[&str]) {
    // equivalent to etest
    unimplemented!();
}

/// || builtin
pub fn dpipe(command: &[&str]) {
    unimplemented!();
}

/// etest builtin (equivalent to [[)
pub fn etest(command: &[&str]) {
    unimplemented!();
}

/// { builtin
pub fn obrace(command: &[&str]) {
    unimplemented!();
}

/// [ builtin (equivalent to test)
pub fn obracket(command: &[&str]) {
    // equivalent to test
    unimplemented!();
}

/// | builtin
pub fn pipe(command: &[&str]) {
    unimplemented!();
}

/// test builtin (equivalent to [)
pub fn test(command: &[&str]) {
    //equivalent to [ ]
    unimplemented!();
}

// I/O builtins

/// clear builtin
pub fn clear(command: &[&str]) {
    linenoise::clear_screen();
}

/// >> builtin
pub fn dgtsign(command: &[&str]) {
    unimplemented!();
}

/// << builtin
pub fn dltsign(command: &[&str]) {
    unimplemented!();
}

/// echo builtin
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

/// > builtin
pub fn gtsign(command: &[&str]) {
    unimplemented!();
}

/// < builtin
pub fn ltsign(command: &[&str]) {
    unimplemented!();
}

/// mapfile builtin (equivalent to readarray)
pub fn mapfile(command: &[&str]) {
    // equivalent to readarray
    unimplemented!();
}

/// printf builtin
pub fn printf(command: &[&str]) {
    unimplemented!();
}

/// read builtin
pub fn read(command: &[&str]) {
    unimplemented!();
}

/// readarray builtin (equivalent to mapfile)
pub fn readarray(command: &[&str]) {
    // equivalent to mapfile
    unimplemented!();
}

/// umask builtin
pub fn umask(command: &[&str]) {
    unimplemented!();
}

// Filesystem builtins

/// cd builtin
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

/// dirs builtin
pub fn dirs(command: &[&str]) {
    unimplemented!();
}

/// popd builtin
pub fn popd(command: &[&str]) {
    unimplemented!();
}

/// pushd builtin
pub fn pushd(command: &[&str]) {
    unimplemented!();
}

// Variables builtins

/// let builtin
pub fn bi_let(command: &[&str]) {
    // equivalent to (( ))
    unimplemented!();
}

/// caller builtin
pub fn caller(command: &[&str]) {
    unimplemented!();
}

/// declare builtin
pub fn declare(command: &[&str]) {
    unimplemented!();
}

/// (( builtin (equivalent to let)
pub fn dparenthesis(command: &[&str]) {
    // equivalent to let
    unimplemented!();
}

/// " builtin
pub fn dquote(command: &[&str]) {
    unimplemented!();
}

/// eval builtin
pub fn eval(command: &[&str]) {
    unimplemented!();
}

/// exec builtin
pub fn exec(command: &[&str]) {
    unimplemented!();
}

/// export builtin
pub fn export(command: &[&str]) {
    unimplemented!();
}

/// getopts builtin
pub fn getopts(command: &[&str]) {
    unimplemented!();
}

/// local builtin
pub fn local(command: &[&str]) {
    // for use only in functions
    unimplemented!();
}

/// readonly builtin
pub fn readonly(command: &[&str]) {
    unimplemented!();
}

/// set builtin
pub fn set(command: &[&str]) {
    unimplemented!();
}

/// shift builtin
pub fn shift(command: &[&str]) {
    unimplemented!();
}

/// shopt builtin
pub fn shopt(command: &[&str]) {
    unimplemented!();
}

/// source builtin (equivalent to .)
pub fn source(command: &[&str]) {
    // equivalent to .
    unimplemented!();
}

/// ' builtin
pub fn squote(command: &[&str]) {
    unimplemented!();
}

/// typeset builtin
pub fn typeset(command: &[&str]) {
    unimplemented!();
}

/// unset builtin
pub fn unset(command: &[&str]) {
    // unset var first if it exists, unset function if no such var exists.
    unimplemented!();
}

// Commands builtins

/// alias builtin
pub fn alias(command: &[&str]) {
    // use aliases HashMap. insert() is enough, even if the key already exists.
    unimplemented!();
}

/// ` builtin
pub fn backtick(command: &[&str]) {
    unimplemented!();
}

/// break builtin
pub fn bi_break(command: &[&str]) {
    unimplemented!();
}

/// continue builtin
pub fn bi_continue(command: &[&str]) {
    unimplemented!();
}

/// false builtin
pub fn bi_false(command: &[&str]) {
    unimplemented!();
}

/// for builtin
pub fn bi_for(command: &[&str]) {
    unimplemented!();
}

/// return builtin
pub fn bi_return(command: &[&str]) {
    unimplemented!();
}

/// true builtin
pub fn bi_true(command: &[&str]) {
    unimplemented!();
}

/// type builtin
pub fn bi_type(command: &[&str]) {
    unimplemented!();
}

/// while builtin
pub fn bi_while(command: &[&str]) {
    unimplemented!();
}

/// bind builtin
pub fn bind(command: &[&str]) {
    unimplemented!();
}

/// case builtin
pub fn case(command: &[&str]) {
    unimplemented!();
}

/// coproc builtin
pub fn coproc(command: &[&str]) {
    unimplemented!();
}

/// expr builtin
pub fn expr(command: &[&str]) {
    unimplemented!();
}

/// function builtin
pub fn function(command: &[&str]) {
    unimplemented!();
}

/// hash builtin
pub fn hash(command: &[&str]) {
    unimplemented!();
}

/// help builtin
pub fn help(command: &[&str]) {
    unimplemented!();
}

/// select builtin
pub fn select(command: &[&str]) {
    unimplemented!();
}

/// unalias builtin
pub fn unalias(command: &[&str]) {
    // use aliases HashMap. remove() is enough even if the key does not exist.
    unimplemented!();
}

/// until builtin
pub fn until(command: &[&str]) {
    unimplemented!();
}

// job control commands

/// autoload builtin
pub fn autoload(command: &[&str]) {
    unimplemented!();
}

/// bg builtin
pub fn bg(command: &[&str]) {
    unimplemented!();
}

/// builtin builtin
pub fn builtin(command: &[&str]) {
    unimplemented!();
}

/// command builtin
pub fn command(command: &[&str]) {
    unimplemented!();
}

/// disown builtin
pub fn disown(command: &[&str]) {
    unimplemented!();
}

/// enable builtin
pub fn enable(command: &[&str]) {
    unimplemented!();
}

/// exit builtin
pub fn exit(command: &[&str]) {
    unimplemented!();
}

/// fg builtin
pub fn fg(command: &[&str]) {
    unimplemented!();
}

/// jobs builtin
pub fn jobs(command: &[&str]) {
    unimplemented!();
}

/// job_spec builtin
pub fn job_spec(command: &[&str]) {
    unimplemented!();
}

/// kill builtin
pub fn kill(command: &[&str]) {
    unimplemented!();
}

/// logout builtin
pub fn logout(command: &[&str]) {
    unimplemented!();
}

/// suspend builtin
pub fn suspend(command: &[&str]) {
    unimplemented!();
}

/// time builtin
pub fn time(command: &[&str]) {
    unimplemented!();
}

/// times builtin
pub fn times(command: &[&str]) {
    unimplemented!();
}

/// trap builtin
pub fn trap(command: &[&str]) {
    unimplemented!();
}

/// ulimit builtin
pub fn ulimit(command: &[&str]) {
    unimplemented!();
}

/// wait builtin
pub fn wait(command: &[&str]) {
    unimplemented!();
}

// Completion commands

/// compgen builtin
pub fn compgen(command: &[&str]) {
    // intended for use from a shell function only
    unimplemented!();
}

/// complete builtin
pub fn complete(command: &[&str]) {
    unimplemented!();
}

/// compopt builtin
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

/// pwd builtin
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

