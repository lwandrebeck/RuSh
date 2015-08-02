/*
 * main.rs
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

extern crate readline;
extern crate term;

use std::io;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

mod builtins;
mod config;

trait ShellCommand {
    fn run(&self);
}

fn handle_command(user_expr: &str) -> bool {
    // Clean up the string by removing the newline at the end
    let expr = user_expr.trim_matches('\n');
    let components: Vec<&str> = expr.split(' ').collect();
    builtins(&components)
}

fn builtins(command: &Vec<&str>) -> bool {
    match command[0] {
		"[[" => { builtins::etest(&command[1..]); },
        "autoload" => { builtins::autoload(&command[1..]); },
        "bg" => { builtins::bg(&command[1..]); },
        "bind" => { builtins::bind(&command[1..]); },
        "builtin" => { builtins::builtin(&command[1..]); },
        "caller" => { builtins::caller(&command[1..]); },
        "cd" => { builtins::cd(&command[1..]); },
        "chmod" => { builtins::chmod(&command[1..]); },
        "chown" => { builtins::chown(&command[1..]); },
        "command" => { builtins::command(&command[1..]); },
        "declare" => { builtins::declare(&command[1..]); },
        "dirs" => { builtins::dirs(&command[1..]); },
        "disown" => { builtins::disown(&command[1..]); },
        "echo" => { builtins::echo(&command[1..]); },
        "enable" => { builtins::enable(&command[1..]); },
        "eval" => { builtins::eval(&command[1..]); },
        "exec" => { builtins::exec(&command[1..]); },
        "exit" => { return true; },
        "export" => {builtins::export(&command[1..]); },
        "false" => { builtins::bi_false(&command[1..]); },
        "fg" => { builtins::fg(&command[1..]); },
        "getopts" => { builtins::getopts(&command[1..]); },
        "hash" => { builtins::hash(&command[1..]); },
        "help" => { builtins::help(&command[1..]); },
        "if" => { builtins::bi_if(&command[1..]); },
        "jobs" => { builtins::jobs(&command[1..]); },
        //"kill" => { builtins::kill(&command[1..]); },
        "killall" => { builtins::killall(&command[1..]); },
        "let" => { builtins::bi_let(&command[1..]); },
        "ln" => { builtins::ln(&command[1..]); },
        "logout" => { builtins::logout(&command[1..]); },
        "mkdir" => { builtins::mkdir(&command[1..]); },
        "printf" => { builtins::printf(&command[1..]); },
        "popd" => { builtins::popd(&command[1..]); },
        "pushd" => { builtins::pushd(&command[1..]); },
        "pwd" => { builtins::pwd(&command[1..]); },
        "read" => { builtins::read(&command[1..]); },
        "readonly" => { builtins::readonly(&command[1..]); },
        "rmdir" => { builtins::rmdir(&command[1..]); },
        "set" => { builtins::set(&command[1..]); },
        "shopt" => { builtins::shopt(&command[1..]); },
        "source" | "." => { builtins::source(&command[1..]); },
        "suspend" => { builtins::suspend(&command[1..]); },
        "test" | "[" => { builtins::test(&command[1..]); },
        "touch" => { builtins::touch(&command[1..]); },
        "true" => { builtins::bi_true(&command[1..]); },
        "times" => { builtins::times(&command[1..]); },
        "type" => { builtins::bi_type(&command[1..]); },
        "typeset" => { builtins::typeset(&command[1..]); },
        "unset" => { builtins::unset(&command[1..]); },
        "wait" => { builtins::wait(&command[1..]); },
        _ => {
            // execute non-builtin command here
        },
    }
    false
}

fn main() {
    let mut stdin = io::stdin();

    loop {
        let mut line = String::new();
        // Add "correct" prompt management
        print!("$ ");
        stdout().flush();
        //let line = stdin.read_line();
        let err = stdin.read_line(&mut line);
        line.pop();
        match err {
            Ok(_) => {
                if handle_command(&line) {
                    return;
                }
            },
            Err(_) => { break; },
        }
    }
}
