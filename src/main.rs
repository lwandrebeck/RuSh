/*
 * main.rs
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

//! RuSh begins here.
//!
//! main.rs contains the very beginning of RuSh.
//! aliases, options structures, environment are defined/set.
//! prompt is updated there.

extern crate libc;
extern crate linenoise;
#[macro_use]
extern crate chomp;
extern crate clap;
extern crate term;

use std::io;
use std::io::{stdin, stdout, Write};
use std::{env, thread, time};
use std::collections::HashMap;
use std::collections::HashSet;
use clap::{App, Arg, SubCommand};

mod builtins;
mod command;
mod config;
mod parser;
mod error;

/// Opt structure is defined here to store options status (setopt)
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Opt<'option> {
    /// name of the option
    name: &'option str,
    /// option status, true or false
    status: bool,
}

/// TBD
trait ShellCommand {
    fn run(&self);
}

/// To be completely overhauled when nom parser is implemented
fn handle_command(user_expr: &str) -> bool {
    // Clean up the string by removing the newline at the end
    let expr = user_expr.trim_matches('\n');
    // Bash (kind of) compatibility. BASH_COMMAND contains command to be executed, RUSH_COMMAND in our case.
    env::set_var("RUSH_COMMAND", expr);
    let components: Vec<&str> = expr.split(' ').collect();
    builtins(&components)
}

/// To be completely overhauled when nom parser is implemented
fn builtins(command: &Vec<&str>) -> bool {
    // TODO to be replaced by some nom magic.
    match command[0] {
        "" => { return false; },
        "[[" => { builtins::etest(&command[1..]); },
        "<" => { builtins::ltsign(&command[1..]); },
        "<<" => { builtins::dltsign(&command[1..]); },
        ">" => { builtins::gtsign(&command[1..]); },
        ">>" => { builtins::dgtsign(&command[1..]); },
        "|" => { builtins::pipe(&command[1..]); },
        "||" => { builtins::dpipe(&command[1..]); },
        "&" => { builtins::and(&command[1..]); },
        "&&" => { builtins::dand(&command[1..]); },
        "`" => { builtins::backtick(&command[1..]); },
        "'" => { builtins::squote(&command[1..]); },
        "\"" => { builtins::dquote(&command[1..]); },
        "alias" => { builtins::alias(&command[1..]); },
        "autoload" => { builtins::autoload(&command[1..]); },
        "bg" => { builtins::bg(&command[1..]); },
        "bind" => { builtins::bind(&command[1..]); },
        "break" => { builtins::bi_break(&command[1..]); },
        "builtin" => { builtins::builtin(&command[1..]); },
        "caller" => { builtins::caller(&command[1..]); },
        "case" => { builtins::case(&command[1..]); },
        "cd" => { builtins::cd(&command[1..]); },
        //"chmod" => { builtins::chmod(&command[1..]); },
        //"chown" => { builtins::chown(&command[1..]); },
        "clear" => { builtins::clear(&command[1..]); },
        "command" => { builtins::command(&command[1..]); },
        "continue" => { builtins::bi_continue(&command[1..]); },
        "declare" => { builtins::declare(&command[1..]); },
        "dirs" => { builtins::dirs(&command[1..]); },
        "disown" => { builtins::disown(&command[1..]); },
        "echo" => { builtins::echo(&command[1..]); },
        "enable" => { builtins::enable(&command[1..]); },
        "eval" => { builtins::eval(&command[1..]); },
        "exec" => { builtins::exec(&command[1..]); },
        "exit" => { return true; },
        "export" => {builtins::export(&command[1..]); },
        "expr" => {builtins::expr(&command[1..]); },
        "false" => { builtins::bi_false(&command[1..]); },
        "fg" => { builtins::fg(&command[1..]); },
        "for" => { builtins::bi_for(&command[1..]); },
        "getopts" => { builtins::getopts(&command[1..]); },
        "hash" => { builtins::hash(&command[1..]); },
        "help" => { builtins::help(&command[1..]); },
        "if" => { builtins::bi_if(&command[1..]); },
        "jobs" => { builtins::jobs(&command[1..]); },
        "kill" => { builtins::kill(&command[1..]); },
        //"killall" => { builtins::killall(&command[1..]); },
        "let" => { builtins::bi_let(&command[1..]); },
        //"ln" => { builtins::ln(&command[1..]); },
        "logout" => { builtins::logout(&command[1..]); },
        //"mkdir" => { builtins::mkdir(&command[1..]); },
        "printf" => { builtins::printf(&command[1..]); },
        "popd" => { builtins::popd(&command[1..]); },
        "pushd" => { builtins::pushd(&command[1..]); },
        "pwd" => { builtins::pwd(&command[1..]); },
        "read" => { builtins::read(&command[1..]); },
        "readonly" => { builtins::readonly(&command[1..]); },
        //"rmdir" => { builtins::rmdir(&command[1..]); },
        "select" => { builtins::select(&command[1..]); },
        "set" => { builtins::set(&command[1..]); },
        "shopt" => { builtins::shopt(&command[1..]); },
        "source" | "." => { builtins::source(&command[1..]); },
        "suspend" => { builtins::suspend(&command[1..]); },
        "test" | "[" => { builtins::test(&command[1..]); },
        //"touch" => { builtins::touch(&command[1..]); },
        "true" => { builtins::bi_true(&command[1..]); },
        "times" => { builtins::times(&command[1..]); },
        "type" => { builtins::bi_type(&command[1..]); },
        "typeset" => { builtins::typeset(&command[1..]); },
        "unset" => { builtins::unset(&command[1..]); },
        "until" => { builtins::until(&command[1..]); },
        "wait" => { builtins::wait(&command[1..]); },
        "while" => { builtins::bi_while(&command[1..]); },
        _ => {
            // execute non-builtin command here
            command::execute_line(&command[0], &command);
        },
    }
    false
}

/// main loop. the fun begins here !
fn main() {
    let mut stdin = io::stdin();
    let mut line_case: u8 = 1; // use PS1 by default at launch
    let mut cmd_nb: u64 = 0; // command number, eventually used by prompt.
    let mut aliases = HashMap::<String, String>::with_capacity(30); // by default fedora 23 already has 14 aliases defined.
    let mut options = HashSet::<Opt>::with_capacity(46);
    config::init_options(&mut options);
    config::init_env();
    // take care of command line arguments.
    let clargs = App::new("RuSh")
                        .version("0.0.1")
                        .author("Laurent Wandrebeck <l.wandrebeck@quelquesmots.fr")
                        .about("A Bash compatible (and more) shell written in Rust")
                        .arg(Arg::with_name("command")
                                    .short("c")
                                    .value_name("COMMAND")
                                    .help("Execute a simple command.")
                                    .takes_value(true))
                        .get_matches();

    if let Some(cmd) = clargs.value_of("command") {
        println!("Value for command: {}", cmd)
    }
    // take care of SECOND env var
    thread::spawn(move ||  {
        loop {
            thread::sleep(time::Duration::new(1, 0));
            match env::var("SECONDS") {
                Ok(val) =>  { let mut s:u64 = val.parse().unwrap(); s += 1; env::set_var("SECONDS", s.to_string()); }
                Err(e) => return
        }; } } );
    loop {
        // FIXME Add "correct" prompt management
        let p = match line_case {
            1 => { let prompt_command = match env::var("PROMPT_COMMAND") {
                        Ok(pc) => pc,
                        Err(e) => String::from("")
                   };
                   if handle_command(&prompt_command) {
                       return;
                   }
                   "PS1" },
            2 => { "PS2" },
            3 => { "PS3" },
            4 => { "PS4" },
            _ => { panic!("impossible line_case value !"); }
        };
        let line = linenoise::input(&config::prompt(&p));
        match line {
            None => { break }
            Some(input) => {
                linenoise::history_add(&input);
                // TODO/FIXME : rewrite the basic parser
                 if handle_command(&input) {
                    return;
                }
                cmd_nb +=1;
            }
        }
     }
}

