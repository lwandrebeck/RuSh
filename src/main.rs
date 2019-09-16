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
extern crate rush;
extern crate rustyline;
extern crate term;

// pub for use is there so doc is generated.
pub use rush::arrays::{Array, Index};
pub use rush::opt::Opt;
pub use rush::parse::parse;
pub use rush::prompt::Prompt;
pub use rush::rush::RuSh;
pub use rush::variables::{Access, Value, Variable, Variables};

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
                parse(&mut rush, &input);
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
