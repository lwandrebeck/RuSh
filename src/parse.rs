/*
 * parse.rs
 *
 * Copyright 2015-2019 Laurent Wandrebeck <l.wandrebeck@quelquesmots.fr>
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

//! RuSh parser
//!
//! Every functions related to parsing of shell input and files are located in that file

use crate::arrays::Index;
use crate::rush::RuSh;
use crate::variables::{Access, Value, Variable};
use std::str::FromStr;

use pest::Parser;
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "rush.pest"]
struct Script;

//~ pub fn parse(rush: &mut RuSh, input: &str) {
//~ let pest = Script::parse(Rule::bla, input).unwrap_or_else(|e| panic!("{}", e));
//~ for line in pest {
//~ //TODO
//~ }
//~ }

pub fn parse(rush: &mut RuSh, input: &str) {
    let pest = Script::parse(Rule::bla, input).unwrap_or_else(|e| panic!("{}", e));
    for line in pest {
        println!("{:#?}", line.as_rule());
        match line.as_rule() {
            Rule::float => println!("float: {}", line.as_span().as_str()),
            Rule::binnum => println!("binnum: {}", line.as_span().as_str()),
            Rule::hexnum => println!("hexnum: {}", line.as_span().as_str()),
            Rule::octnum => println!("octnum: {}", line.as_span().as_str()),
            Rule::int => println!("int: {}", line.as_span().as_str()),
            Rule::squoted => println!("squoted: {}", line.as_span().as_str()),
            Rule::dquoted => println!("dquoted: {}", line.as_span().as_str()),
            Rule::btquoted => println!("btquoted: {}", line.as_span().as_str()),
            Rule::nonquoted => println!("nonquoted: {:#?}", line),
            Rule::shebang => println!("shebang: {}", line.as_span().as_str()),
            Rule::comment => println!("comment: {}", line.as_span().as_str()),
            Rule::floatvarassign => {
                let mut var = String::new();
                let mut val = 0.0;
                let mut index: Option<Index> = None;
                for inner in line.into_inner() {
                    match inner.as_rule() {
                        Rule::varname => var = inner.as_span().as_str().to_string(),
                        Rule::float => val = f64::from_str(inner.as_span().as_str()).unwrap(),
                        // FIXME - index management needs to be implemented.
                        Rule::index => {
                            index = Some(Index::I(0));
                            unimplemented!();
                        }
                        _ => unreachable!(),
                    }
                }
                rush.shell_vars.set(
                    var,
                    Variable {
                        value: Value::F(val),
                        access: Access::ReadWrite,
                    },
                );
            }
            Rule::binvarassign => println!("binvarassign: {}", line.as_span().as_str()),
            Rule::intvarassign => {
                let mut var = String::new();
                let mut val = 0;
                for inner in line.into_inner() {
                    match inner.as_rule() {
                        Rule::varname => var = inner.as_span().as_str().to_string(),
                        Rule::int => val = i64::from_str(inner.as_span().as_str()).unwrap(),
                        _ => unreachable!(),
                    }
                }
                rush.shell_vars.set(
                    var,
                    Variable {
                        value: Value::I(val),
                        access: Access::ReadWrite,
                    },
                );
            }
            Rule::hexvarassign => println!("hexvarassign: {}", line.as_span().as_str()),
            Rule::octvarassign => println!("octvarassign: {}", line.as_span().as_str()),
            Rule::stringvarassign => {
                let mut var = String::new();
                let mut val = String::new();
                for inner in line.into_inner() {
                    match inner.as_rule() {
                        Rule::varname => var = inner.as_span().as_str().to_string(),
                        // FIXME: add variables management
                        Rule::dquoted => val = inner.as_span().as_str().to_string(),
                        Rule::squoted => val = inner.as_span().as_str().to_string(),
                        // FIXME: add execution management
                        Rule::btquoted => val = inner.as_span().as_str().to_string(),
                        Rule::nonquoted => val = inner.as_span().as_str().to_string(),
                        _ => unreachable!(),
                    }
                }
                rush.shell_vars.set(
                    var,
                    Variable {
                        value: Value::S(val),
                        access: Access::ReadWrite,
                    },
                );
            }
            Rule::nop => println!("nop: {}", line.as_span().as_str()),
            Rule::localfloatvarassign => {
                println!("localfloatvarassign: {}", line.as_span().as_str())
            }
            Rule::localbinvarassign => println!("localbinvarassign: {}", line.as_span().as_str()),
            Rule::localintvarassign => println!("localintvarassign: {}", line.as_span().as_str()),
            Rule::localhexvarassign => println!("localhexvarassign: {}", line.as_span().as_str()),
            Rule::localoctvarassign => println!("localoctvarassign: {}", line.as_span().as_str()),
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
                        Rule::echooptne => println!("echooptne: {}", inner.as_span().as_str()),
                        Rule::echooptnE => println!("echooptnE: {}", inner.as_span().as_str()),
                        Rule::echoopte => println!("echoopte: {}", inner.as_span().as_str()),
                        Rule::echooptE => println!("echooptE: {}", inner.as_span().as_str()),
                        Rule::echooptn => println!("echooptn: {}", inner.as_span().as_str()),
                        Rule::dquoted => println!("echo dquoted:  {}", inner.as_span().as_str()),
                        Rule::squoted => println!("echo squoted:  {}", inner.as_span().as_str()),
                        Rule::btquoted => println!("echo btquoted:  {}", inner.as_span().as_str()),
                        Rule::nonquoted => {
                            println!("echo nonquoted:  {}", inner.as_span().as_str());
                            println!("{:#?}", inner.into_inner());
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(), // ident rule is silent and cannot be reached
        };
    }
}
