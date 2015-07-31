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

fn handle_command(user_expr: &str) {
    // Clean up the string by removing the newline at the end
    let expr = user_expr.trim_matches('\n');
    let components: Vec<&str> = expr.split(' ').collect();
    if builtins(&components) {
        return;
    }
}

fn builtins(command: &Vec<&str>) -> bool {
    match command[0] {
		"[[" => { builtins::etest(command); },
        "autoload" => { builtins::autoload(command); },
        "bg" => { builtins::bg(command); },
        "bind" => { builtins::bind(command); },
        "builtin" => { builtins::builtin(command); },
        "caller" => { builtins::caller(command); },
        "cd" => { builtins::cd(command); },
        "chmod" => { builtins::chmod(command); },
        "chown" => { builtins::chown(command); },
        "command" => { builtins::command(command); },
        "declare" => { builtins::declare(command); },
        "dirs" => { builtins::dirs(command); },
        "disown" => { builtins::disown(command); },
        "echo" => { builtins::echo(command); },
        "enable" => { builtins::enable(command); },
        "eval" => { builtins::eval(command); },
        "exec" => { builtins::exec(command); },
        "exit" => { builtins::exit(command); },
        "export" => {builtins::export(command); },
        "false" => { builtins::bi_false(command); },
        "fg" => { builtins::fg(command); },
        "getopts" => { builtins::getopts(command); },
        "hash" => { builtins::hash(command); },
        "help" => { builtins::help(command); },
        "if" => { builtins::bi_if(command); },
        "jobs" => { builtins::jobs(command); },
        "kill" => { builtins::kill(command); },
        "killall" => { builtins::killall(command); },
        "let" => { builtins::bi_let(command); },
        "ln" => { builtins::ln(command); },
        "logout" => { builtins::logout(command); },
        "mkdir" => { builtins::mkdir(command); },
        "printf" => { builtins::printf(command); },
        "popd" => { builtins::popd(command); },
        "pushd" => { builtins::pushd(command); },
        "pwd" => { builtins::pwd(command); },
        "read" => { builtins::read(command); },
        "readonly" => { builtins::readonly(command); },
        "rmdir" => { builtins::rmdir(command); },
        "set" => { builtins::set(command); },
        "shopt" => { builtins::shopt(command); },
        "source" | "." => { builtins::source(command); },
        "suspend" => { builtins::suspend(command); },
        "test" | "[" => { builtins::test(command); },
        "touch" => { builtins::touch(command); },
        "true" => { builtins::bi_true(command); },
        "times" => { builtins::times(command); },
        "type" => { builtins::bi_type(command); },
        "typeset" => { builtins::typeset(command); },
        "unset" => { builtins::unset(command); },
        "wait" => { builtins::wait(command); },
        _ => { return false; },
        //"" => { builtins::(command); },
    }
    true
}

fn main() {
    let mut stdin = io::stdin();
    let mut line = String::new();
    loop {
        // Add correct prompt management
        print!("$ ");
        stdout().flush();
        //let line = stdin.read_line();
        let err = stdin.read_line(&mut line);
        line.pop();
        match err {
            Ok(_) => { handle_command(&line); },
            Err(_) => { break; },
        }
        line.clear();
    }
}
