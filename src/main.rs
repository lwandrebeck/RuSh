/*
 * main.rs
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

//! RuSh begins here.
//!
//! main.rs contains the very beginning of RuSh.
//! aliases, options structures, environment are defined/set.
//! prompt is updated there.

// Include other files.
mod variables;
mod prompt;
mod opt;
mod aliases;

extern crate libc;
extern crate rustyline;
extern crate term;
extern crate seahash;
extern crate rand;
extern crate chrono;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::str;
use std::collections::HashMap;
use pest::Parser;
use variables::Variables;
use opt::Opt;
use aliases::Aliases;

/// pest grammar inclusion. dummy const so that .pest file changes are taken care of.
const _GRAMMAR: &'static str = include_str!("rush.pest"); // relative to src path

#[derive(Parser)]
#[grammar = "rush.pest"]
struct Script;

/// Structure to store variable value and rw state.
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct ValueRW {
    value: String,
    rw: bool
}

/// Core structure containing everything needed for RuSh
//#[derive(Hash, Eq, PartialEq, Debug)]
pub struct RuSh {
    /// aliases: Stored as HashMap<&str, &str>
    aliases: Aliases,
    /// shopt_options: autocd, etc. See man bash, shopt options. Stored as HashMap<&str, &bool>
    shopt_options: Opt,
    /// set_options: allexport, braceexpand, etc. See man bash, set command. Stored as HashMap<&str, &bool>
    set_options: Opt,
    /// shell_vars: RUSH, RUSHPID, etc. See man bash, shell variables. Stored as HashMap<&str, &str>
    shell_vars: Variables,
    /// Command history. Stored as History from rustyline
    history: rustyline::history::History,
    /// line case, needed for prompt management
    line_case: u8,
    /// command number, may be needed by prompt
    cmd_nb: u64,
    /// prompt contents. Stored as a simple String.
    prompt: prompt::Prompt,
}

impl Default for RuSh {
    fn default() -> RuSh {
        let mut shell = RuSh {
            // 15 aliases by default in Fedora 26.
            aliases: Aliases::init_aliases(),
            // 46 shopt options by default, so let’s have a big enough HashMap to store these.
            shopt_options: Opt::init_shopt_options(),
            // 27 set options by default, so let’s have a big enough HashMap to store these.
            set_options: Opt::init_set_options(),
            // 100 or so shell vars are defined upon startup. Let’s say most scripts do use up to 200 vars, so let’s alloc enough.
            shell_vars: Variables::init_shell_vars(),
            // TODO set history size
            // rl.set_history_max_len(1000);
            history: rustyline::history::History::new(),
            // prompt management. TODO
            line_case: 1,
            cmd_nb: 0,
            prompt: prompt::Prompt { prompt: String::from("") }
        };
        shell.prompt = prompt::Prompt::get(&mut shell.shell_vars, "PS1");
        //let mut stdin = io::stdin();
        let mut rl = rustyline::Editor::<()>::new();
        // take care of SECOND env var
        //thread::spawn(move ||  {
        //    loop {
        //        thread::sleep(time::Duration::new(1, 0));
        //        match shell.shell_vars.entry("SECONDS".into()) {
        //          Entry::Occupied(val) =>  { let mut s:u64 = val.get().value.parse().unwrap_or(0); s += 1; shell.shell_vars.insert("SECONDS".to_string(), ValueRW { value: s.to_string(), rw: true }); },
        //          Entry::Vacant(val) => { shell.shell_vars.insert("SECONDS".to_string(), ValueRW { value: "1".to_string(), rw: true }); }
        //      }
        //  }
        //});
        loop {
            let line = rl.readline(&shell.prompt.prompt);
            match line {
                Ok(input) => {
                    // TODO fix history management
                    rl.add_history_entry(&input);
                    let pest = Script::parse_str(Rule::bla, &input).unwrap_or_else(|e| panic!("{}", e));
                    for line in pest {
                        match line.as_rule() {
                            Rule::float => println!("float: {}", line.into_span().as_str()),
                            Rule::binnum => println!("binnum: {}", line.into_span().as_str()),
                            Rule::hexnum => println!("hexnum: {}", line.into_span().as_str()),
                            Rule::octnum => println!("octnum: {}", line.into_span().as_str()),
                            Rule::int => println!("int: {}", line.into_span().as_str()),
                            Rule::squoted => println!("squoted: {}", line.into_span().as_str()),
                            Rule::dquoted => println!("dquoted: {}", line.into_span().as_str()),
                            Rule::btquoted => println!("btquoted: {}", line.into_span().as_str()),
                            Rule::nonquoted => println!("nonquoted: {}", line.into_span().as_str()),
                            Rule::shebang => println!("shebang: {}", line.into_span().as_str()),
                            Rule::comment => println!("comment: {}", line.into_span().as_str()),
                            Rule::floatvarassign => println!("floatvarassign: {}", line.into_span().as_str()),
                            Rule::binvarassign => println!("binvarassign: {}", line.into_span().as_str()),
                            Rule::intvarassign => println!("intvarassign: {}", line.into_span().as_str()),
                            Rule::hexvarassign => println!("hexvarassign: {}", line.into_span().as_str()),
                            Rule::octvarassign => println!("octvarassign: {}", line.into_span().as_str()),
                            Rule::stringvarassign => println!("stringvarassign: {}", line.into_span().as_str()),
                            _ => unreachable!() // ident rule is silent and cannot be reached
                        };
                    }
                    shell.cmd_nb +=1;
                    },
                Err(_) => { break }
            }
            match shell.line_case {
                1 => shell.prompt = prompt::Prompt::get(&mut shell.shell_vars, "PS1"),
                2 => shell.prompt = prompt::Prompt::get(&mut shell.shell_vars, "PS2"),
                3 => shell.prompt = prompt::Prompt::get(&mut shell.shell_vars, "PS3"),
                4 => shell.prompt = prompt::Prompt::get(&mut shell.shell_vars, "PS4"),
                _ => panic!("wrong line_case value.")
            }
        }
    shell
    }
}

/// main loop. the fun begins here !
fn main() {
    RuSh::default();
}

