//
// main.rs
//
// Copyright 2015-2018 Laurent Wandrebeck <l.wandrebeck@quelquesmots.fr>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
// MA 02110-1301, USA.
//

/// RuSh begins here.
///
/// main.rs contains the very beginning of RuSh.
/// Shell structure is defined and initialized here.
/// Main loop of the program.

/// Include variables management.
mod variables;
/// Include prompt management.
mod prompt;
/// Include options management (shopt, set)
mod opt;
/// Include aliases management (alias, unalias)
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
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;
//use std::{str, thread, time};
use std::str;
use std::collections::HashMap;
//use std::collections::hash_map::Entry::{Occupied, Vacant};
use pest::Parser;
//use variables::{Variables, Variable, Value};
// pub for use is there so doc is generated.
pub use crate::variables::Variables;
pub use crate::opt::Opt;
pub use crate::aliases::Aliases;

/// pest grammar inclusion. dummy const so that .pest file changes are taken care of.
#[derive(Parser)]
#[grammar = "rush.pest"]
struct Script;

/// Core structure containing everything needed for RuSh
pub struct RuSh {
    /// aliases: Stored as Aliases { aliases: HashMap<String, String> }
    aliases: Aliases,
    /// shopt_options: autocd, etc. See man bash, shopt options. Stored as Opt { opt: HashMap<String, <set: bool, rw: bool>> }
    shopt_options: Opt,
    /// set_options: allexport, braceexpand, etc. See man bash, set command. Stored as HashMap<String, <bool, bool>>
    set_options: Opt,
    /// shell_vars: RUSH, RUSHPID, etc. See man bash, shell variables. Stored as HashMap<String, <i64 or f64 or String, bool>>
    shell_vars: Variables,
    /// Command history. Stored as History from rustyline
    history: rustyline::history::History,
    /// line case, needed for prompt management
    line_case: u8,
    /// command number, may be needed by prompt
    cmd_nb: u64,
    /// prompt contents. Stored as Prompt { prompt: String }
    prompt: prompt::Prompt,
}

/// Default method for RuSh
impl Default for RuSh {
    fn default() -> RuSh {
        let mut shell = RuSh {
            /// 15 aliases are defined by default in Fedora 26, so let’s allocate twice that.
            aliases: Aliases::init_aliases(),
            /// 46 shopt options by default, so let’s have a big enough HashMap to store these.
            shopt_options: Opt::init_shopt_options(),
            /// 27 set options by default, so let’s have a big enough HashMap to store these.
            set_options: Opt::init_set_options(),
            /// 100 or so shell vars are defined upon startup. Allocate twice that.
            shell_vars: Variables::init_shell_vars(),
            // TODO set history size
            // rl.set_history_max_len(1000);
            /// Manage commands history with rustyline crate.
            history: rustyline::history::History::new(),
            /// Variable line_case allows to know which PS[1234] variable to use to display prompt.
            line_case: 1,
            /// Command number in this session. Can be used in prompt.
            cmd_nb: 0,
            /// Variable prompt contains interpreted definition of PS[1234].
            prompt: prompt::Prompt { prompt: String::from("") }
        };
        shell.prompt = prompt::Prompt::get(&mut shell.shell_vars, "PS1");
        //let mut stdin = io::stdin();
        let mut rl = rustyline::Editor::<()>::new();
        // take care of SECOND env var
        //~ let child = thread::spawn(move ||  {
            //~ loop {
                //~ thread::sleep(time::Duration::new(1, 0));
                //~ match shell.shell_vars.get("SECONDS") {
                  //~ Some(val) =>  { let mut s = val.geti(); s += 1; shell.shell_vars.set("SECONDS".to_string(), Variable { value: Value::I(s), rw: true }); },
                  //~ None => { shell.shell_vars.set("SECONDS".to_string(), Variable { value: Value::I(1), rw: true }); }
              //~ }
          //~ }
        //~ });
        // main loop. display prompt, wait for input, parse, etc.
        loop {
            let line = rl.readline(&shell.prompt.prompt);
            // (very) Basic parsing for now. To be moved in parser.rs later on.
            match line {
                Ok(input) => {
                    // TODO fix history management
                    // rl.add_history_entry(&input);
                    let pest = Script::parse(Rule::bla, &input).unwrap_or_else(|e| panic!("{}", e));
                    for line in pest {
                        match line.as_rule() {
                            Rule::float => println!("float: {}", line.as_span().as_str()),
                            Rule::binnum => println!("binnum: {}", line.as_span().as_str()),
                            Rule::hexnum => println!("hexnum: {}", line.as_span().as_str()),
                            Rule::octnum => println!("octnum: {}", line.as_span().as_str()),
                            Rule::int => println!("int: {}", line.as_span().as_str()),
                            Rule::squoted => println!("squoted: {}", line.as_span().as_str()),
                            Rule::dquoted => println!("dquoted: {}", line.as_span().as_str()),
                            Rule::btquoted => println!("btquoted: {}", line.as_span().as_str()),
                            Rule::nonquoted => println!("nonquoted: {}", line.as_span().as_str()),
                            Rule::shebang => println!("shebang: {}", line.as_span().as_str()),
                            Rule::comment => println!("comment: {}", line.as_span().as_str()),
                            Rule::floatvarassign => println!("floatvarassign: {}", line.as_span().as_str()),
                            Rule::binvarassign => println!("binvarassign: {}", line.as_span().as_str()),
                            Rule::intvarassign => println!("intvarassign: {}", line.as_span().as_str()),
                            Rule::hexvarassign => println!("hexvarassign: {}", line.as_span().as_str()),
                            Rule::octvarassign => println!("octvarassign: {}", line.as_span().as_str()),
                            Rule::stringvarassign => println!("stringvarassign: {}", line.as_span().as_str()),
                            Rule::nop => println!("nop: {}", line.as_span().as_str()),
                            Rule::localfloatvarassign => println!("localfloatvarassign: {}", line.as_span().as_str()),
                            Rule::localbinvarassign => println!("localbinvarassign: {}", line.as_span().as_str()),
                            Rule::localintvarassign => println!("localintvarassign: {}", line.as_span().as_str()),
                            Rule::localhexvarassign => println!("localhexvarassign: {}", line.as_span().as_str()),
                            Rule::localoctvarassign => println!("localoctvarassign: {}", line.as_span().as_str()),
                            Rule::localstringvarassign => println!("localstringvarassign: {}", line.as_span().as_str()),
                            Rule::alnum => println!("alnum: {}", line.as_span().as_str()),
                            Rule::alph => println!("alph: {}", line.as_span().as_str()),
                            Rule::blank => println!("blank: {}", line.as_span().as_str()),
                            Rule::cntrl => println!("cntrl: {}", line.as_span().as_str()),
                            Rule::digi => println!("digi: {}", line.as_span().as_str()),
                            Rule::graph => println!("graph: {}", line.as_span().as_str()),
                            Rule::lower => println!("lower: {}", line.as_span().as_str()),
                            Rule::prin => println!("prin: {}", line.as_span().as_str()),
                            Rule::space => println!("space: {}", line.as_span().as_str()),
                            Rule::upper => println!("upper: {}", line.as_span().as_str()),
                            Rule::xdigit => println!("xdigit: {}", line.as_span().as_str()),
                            Rule::brea => println!("break: {}", line.as_span().as_str()),
                            Rule::continu => println!("continue: {}", line.as_span().as_str()),
                            //Rule::pwd => println!("pwd: {}", line.as_span().as_str()),
                            Rule::exit => println!("exit: {}", line.as_span().as_str()),
                            Rule::tru => println!("true: {}", line.as_span().as_str()),
                            Rule::fals => println!("false: {}", line.as_span().as_str()),
                            //Rule::help => println!("help: {}", line.as_span().as_str()),
                            //Rule::bg => println!("bg: {}", line.as_span().as_str()),
                            //Rule::fg => println!("fg: {}", line.as_span().as_str()),
                            Rule::logout => println!("logout: {}", line.as_span().as_str()),
                            Rule::echo => {
                                for inner in line.into_inner() {
                                    match inner.as_rule() {
                                        Rule::echooptne => println!("echooptne: {}", inner.as_span().as_str()),
                                        Rule::echooptnE => println!("echooptnE: {}", inner.as_span().as_str()),
                                        Rule::echoopte => println!("echoopte: {}", inner.as_span().as_str()),
                                        Rule::echooptE => println!("echooptE: {}", inner.as_span().as_str()),
                                        Rule::echooptn => println!("echooptn: {}", inner.as_span().as_str()),
                                        Rule::dquoted => println!("echo dquoted:  {}", inner.as_span().as_str()),
                                        Rule::squoted => println!("echo squoted:  {}", inner.as_span().as_str()),
                                        Rule::btquoted => println!("echo btquoted:  {}", inner.as_span().as_str()),
                                        Rule::nonquoted => println!("echo nonquoted:  {}", inner.as_span().as_str()),
                                        _ => unreachable!()
                                    };
                                }
                            },
                            _ => unreachable!() // ident rule is silent and cannot be reached
                        };
                    }
                    shell.cmd_nb +=1;
                    },
                Err(_) => { break }
            }
            // Use correct variable to define next prompt display.
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

/// This is the main function. Initializes RuSh structure and starts the shell.
fn main() {
    RuSh::default();
}
