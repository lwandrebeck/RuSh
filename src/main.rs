//
// main.rs
//
// Copyright 2015-2019 Laurent Wandrebeck <l.wandrebeck@quelquesmots.fr>
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

//! # RuSh
//!
//! `RuSh` is a shell written in Rust
//! RuSh aims to be (maybe one day) a POSIX Shell, Bash compatible (or close to) with candies.
//! Source code is GPL3. Please note that this is a personal project (read not funded), in order to learn Rust language.
//! That does not mean feedback or patches are not welcome.
//! Right now, RuSh is definitely not useable. A couple little things have been done, but 99% (at least) have to be written.

extern crate chrono;
extern crate libc;
extern crate pest;
extern crate rand;
extern crate rustyline;
extern crate term;
#[macro_use]
extern crate pest_derive;
extern crate rush;
extern crate structopt;
extern crate structopt_derive;

use pest::Parser;
// pub for use is there so doc is generated.
pub use rush::arrays::Array;
pub use rush::opt::Opt;
pub use rush::prompt::Prompt;
pub use rush::rush::RuSh;
pub use rush::variables::Variables;

/// pest grammar inclusion. dummy const so that .pest file changes are taken care of.
#[derive(Parser)]
#[grammar = "rush.pest"]
struct Script;

/// This is the main function. Initializes RuSh structure and starts the shell.
fn main() {
    let mut rush = RuSh::default();
    //rush.prompt = Prompt::get(&mut rush.shell_vars, "PS1");
    rush.prompt = Prompt::get(&mut rush, "PS1");
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
        let line = rl.readline(&rush.prompt.prompt);
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
                        Rule::floatvarassign => {
                            println!("floatvarassign: {}", line.as_span().as_str())
                        }
                        Rule::binvarassign => println!("binvarassign: {}", line.as_span().as_str()),
                        Rule::intvarassign => println!("intvarassign: {}", line.as_span().as_str()),
                        Rule::hexvarassign => println!("hexvarassign: {}", line.as_span().as_str()),
                        Rule::octvarassign => println!("octvarassign: {}", line.as_span().as_str()),
                        Rule::stringvarassign => {
                            println!("stringvarassign: {}", line.as_span().as_str())
                        }
                        Rule::nop => println!("nop: {}", line.as_span().as_str()),
                        Rule::localfloatvarassign => {
                            println!("localfloatvarassign: {}", line.as_span().as_str())
                        }
                        Rule::localbinvarassign => {
                            println!("localbinvarassign: {}", line.as_span().as_str())
                        }
                        Rule::localintvarassign => {
                            println!("localintvarassign: {}", line.as_span().as_str())
                        }
                        Rule::localhexvarassign => {
                            println!("localhexvarassign: {}", line.as_span().as_str())
                        }
                        Rule::localoctvarassign => {
                            println!("localoctvarassign: {}", line.as_span().as_str())
                        }
                        Rule::localstringvarassign => {
                            println!("localstringvarassign: {}", line.as_span().as_str())
                        }
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
                                    Rule::echooptne => {
                                        println!("echooptne: {}", inner.as_span().as_str())
                                    }
                                    Rule::echooptnE => {
                                        println!("echooptnE: {}", inner.as_span().as_str())
                                    }
                                    Rule::echoopte => {
                                        println!("echoopte: {}", inner.as_span().as_str())
                                    }
                                    Rule::echooptE => {
                                        println!("echooptE: {}", inner.as_span().as_str())
                                    }
                                    Rule::echooptn => {
                                        println!("echooptn: {}", inner.as_span().as_str())
                                    }
                                    Rule::dquoted => {
                                        println!("echo dquoted:  {}", inner.as_span().as_str())
                                    }
                                    Rule::squoted => {
                                        println!("echo squoted:  {}", inner.as_span().as_str())
                                    }
                                    Rule::btquoted => {
                                        println!("echo btquoted:  {}", inner.as_span().as_str())
                                    }
                                    Rule::nonquoted => {
                                        println!("echo nonquoted:  {}", inner.as_span().as_str())
                                    }
                                    _ => unreachable!(),
                                };
                            }
                        }
                        _ => unreachable!(), // ident rule is silent and cannot be reached
                    };
                }
                rush.cmd_nb += 1;
            }
            Err(_) => break,
        }
        // Use correct variable to define next prompt display.
        match rush.line_case {
            1 => rush.prompt = Prompt::get(&mut rush, "PS1"),
            2 => rush.prompt = Prompt::get(&mut rush, "PS2"),
            3 => rush.prompt = Prompt::get(&mut rush, "PS3"),
            4 => rush.prompt = Prompt::get(&mut rush, "PS4"),
            _ => panic!("wrong line_case value."),
        }
    }
}
