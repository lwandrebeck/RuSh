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

use crate::arrays::{Array, Index};
use crate::rush::RuSh;
use crate::variables::{Access, Value, Variable};
use std::convert::TryInto;
use std::str::FromStr;

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
//~ use pest_consume::{Error, Parser, match_nodes};
//~ type Result<T> = std::result::Result<T, Error<Rule>>;
//~ type Node<'i> = pest_consume::Node<'i, Rule, ()>;
#[derive(Parser)]
#[grammar = "rush.pest"]
struct Script;

//~ #[pest_consume::parser]
//~ impl Script {
//~ fn float(input: Node) -> Result<Value> {
//~ Ok(Value::F(f64::from_str(&input.as_str()).unwrap()))
//~ }

//~ fn binnum(input: Node) -> Result<Value> {
//~ Ok(Value::I(i64::from_str_radix(&input.as_str()[2..], 2).unwrap()))
//~ }

//~ fn hexnum(input: Node) -> Result<Value> {
//~ Ok(Value::I(i64::from_str_radix(&input.as_str()[2..], 16).unwrap()))
//~ }

//~ fn octnum(input: Node) -> Result<Value> {
//~ Ok(Value::I(i64::from_str_radix(&input.as_str()[1..], 8).unwrap()))
//~ }

//~ fn int(input: Node) -> Result<Value> {
//~ Ok(Value::I(i64::from_str(&input.as_str()).unwrap()))
//~ }

//~ fn bla(input: Node) -> Result<Value> {
//~ Ok(Value::I(-1))
//~ }

//~ fn squoted(input: Node) -> Result<Value> {
//~ Ok(Value::S(input.as_str().to_string()))
//~ }

//~ fn varname(input: Node) -> Result<String> {
//~ Ok(input.as_str().to_string())
//~ }

//~ fn varlowe(input: Node) -> Result<bool> {
//~ Ok(true)
//~ }

//~ fn varlow(input: Node) -> Result<bool> {
//~ Ok(true)
//~ }

//~ fn varupp(input: Node) -> Result<bool> {
//~ Ok(true)
//~ }

//~ fn varup(input: Node) -> Result<bool> {
//~ Ok(true)
//~ }

//~ fn varvalue(input: Node) -> Result<Value> {
//~ Ok(match_nodes!(input.into_children();
//~ [varname(v), index(i), varlowe(lo)] => {
//~ let val = RuSh::shell_array_vars::get(v, i);
//~ match val {
//~ Value::F(f) => f.to_string(),
//~ Value::I(i) => i.to_string(),
//~ Value::S(s) => s.to_lowercase()
//~ rush.shell_array_vars::get(v, i)
//~ },
//~ // FIXME, no std method to lowercase only first char
//~ [varname(v), index(i), varlow(lo)] => {
//~ let val = rush::RuSh::shell_array_vars::get(v, i);
//~ match val {
//~ Value::F(f) => Ok(Value::F(f)),
//~ Value::I(i) => Ok(Value::I(i)),
//~ Value::S(s) => Ok(Value::S(s.to_lowercase()))
//~ }},
//~ [varname(v), index(i), varupp(up)] => {
//~ let val = RuSh::shell_array_vars::get(v, i);
//~ match val {
//~ Value::F(f) => Ok(Value::F(f)),
//~ Value::I(i) => Ok(Value::I(i)),
//~ Value::S(s) => Ok(Value::S(s.to_uppercase()))
//~ }},
//~ // FIXME, no std method to uppercase only first char
//~ [varname(v), index(i), varup(up)] => {
//~ let val = RuSh::shell_array_vars::get(v, i);
//~ match val {
//~ Value::F(f) => Ok(Value::F(f)),
//~ Value::I(i) => Ok(Value::I(i)),
//~ Value::S(s) => Ok(Value::S(s.to_uppercase()))
//~ }},
//~ [varname(v), varlowe(lo)] => {
//~ let val = RuSh::shell_vars::get(v);
//~ match val {
//~ Value::F(f) => Ok(Value::F(f)),
//~ Value::I(i) => Ok(Value::I(i)),
//~ Value::S(s) => Ok(Value::S(s.to_lowercase()))
//~ }},
//~ // FIXME, no std method to lowercase only first char
//~ [varname(v), varlow(lo)] => {
//~ let val = RuSh::shell_vars::get(v);
//~ match val {
//~ Value::F(f) => Ok(Value::F(f)),
//~ Value::I(i) => Ok(Value::I(i)),
//~ Value::S(s) => Ok(Value::S(s.to_lowercase()))
//~ }},
//~ [varname(v), varupp(up)] => {
//~ let val = RuSh::shell_vars::get(v);
//~ match val {
//~ Value::F(f) => Ok(Value::F(f)),
//~ Value::I(i) => Ok(Value::I(i)),
//~ Value::S(s) => Ok(Value::S(s.to_uppercase()))
//~ }},
//~ // FIXME, no std method to uppercase only first char
//~ [varname(v), varup(up)] => {
//~ let val = RuSh::shell_vars::get(v);
//~ match val {
//~ Value::F(f) => Ok(Value::F(f)),
//~ Value::I(i) => Ok(Value::I(i)),
//~ Value::S(s) => Ok(Value::S(s.to_uppercase()))
//~ }},
//~ [varname(v)] => {
//~ let val = RuSh::shell_vars::get(v);
//~ match val {
//~ Value::F(f) => Ok(Value::F(f)),
//~ Value::I(i) => Ok(Value::I(i)),
//~ Value::S(s) => Ok(Value::S(s.to_uppercase()))
//~ }}
//~ ))
//~ }
//~ // FIXME
//~ fn index(input: Node) -> Result<Value> {
//~ Ok(Value::I(0))
//~ }

//~ ///
//~ fn varassign(input: Node) -> Result<Value> {
//~ Ok(match_nodes!(input.into_children();
//~ [varname(v), index(i), float(f)] => { RuSh::shell_array_vars.set(v, i, f); f  },
//~ [varname(v), float(f)] => { RuSh::shell_vars.set(v.as_str(), f); f },
//~ ))
//~ }
//~ }

//~ pub fn parse(rush: &mut RuSh, input_str: &str) -> Value {
//~ // Parser::parse
//~ let inputs = Script::parse(Rule::bla, input_str).unwrap();
//~ let input = inputs.single().unwrap();
//~ let output = Script::bla(input);
//~ match output {
//~ Ok(r) => r,
//~ Err(e) => Value::I(1)
//~ }
//~ }

fn varname(p: &Pair<Rule>, name: String) -> (String, Option<Index>) {
    //for i in p.as_rule() {
    match p.as_rule() {
        Rule::index => (p.as_span().as_str().to_string(), Some(index(p))),
        Rule::arg => (p.as_span().as_str().to_string(), None), //FIXME
        Rule::numarg => (p.as_span().as_str().to_string(), None), //FIXME
        Rule::allarg => (p.as_span().as_str().to_string(), None), //FIXME
        Rule::exitstatus => (p.as_span().as_str().to_string(), None), //FIXME
        Rule::pid => (p.as_span().as_str().to_string(), None), //FIXME
        // FIXME ensure we cover everything correctly
        _ => panic!(),
    }
    //}
    //~ let mut v:Option<Value>;
    //~ if arr {
    //~ v = rush::shell_array_vars.get(p.into_span().as_str().unwrap(), i);
    //~ } else {
    //~ v = rush::shell_vars.get(p.into_span().as_str().unwrap());
    //~ }
    //~ match v {
    //~ Some(v) => v,
    //~ None => Value::S("")
    //~ }
}

fn varops(p: &Pair<Rule>) -> Value {
    match p.as_rule() {
        Rule::varmatch => unimplemented!(),
        Rule::varlen => unimplemented!(),
        Rule::varreplall => unimplemented!(),
        Rule::varrepl => unimplemented!(),
        Rule::varsubstback => unimplemented!(),
        Rule::varsubstfront => unimplemented!(),
        Rule::varlongbackmatch => unimplemented!(),
        Rule::varshortbackmatch => unimplemented!(),
        Rule::varlongfrontmatch => unimplemented!(),
        Rule::varshortfrontmatch => unimplemented!(),
        Rule::varsubstrlen => unimplemented!(),
        Rule::varsubstr => unimplemented!(),
        Rule::varerrmsg => unimplemented!(),
        Rule::varaltvalue => unimplemented!(),
        Rule::varsetdefault => unimplemented!(),
        Rule::varusedefault => unimplemented!(),
        // FIXME ensure we cover everything correctly
        _ => panic!(),
    }
}

fn index(p: &Pair<Rule>) -> Index {
    let mut index: Index;
    match p.as_rule() {
        // FIXME index::I is usize and need to be correctly parsed (ie -1 is last element)
        Rule::hexnum => Index::I(hexnum(p).try_into().unwrap()),
        Rule::binnum => Index::I(binnum(p).try_into().unwrap()),
        Rule::octnum => Index::I(octnum(p).try_into().unwrap()),
        Rule::int => Index::I(int(p).try_into().unwrap()),
        Rule::varvalue => unimplemented!(),
        Rule::dquoted => unimplemented!(),
        Rule::squoted => unimplemented!(),
        Rule::btquoted => unimplemented!(),
        Rule::nonquoted => unimplemented!(),
        // FIXME ensure we cover everything correctly
        _ => panic!(),
    }
}

fn hexnum(p: &Pair<Rule>) -> i64 {
    i64::from_str_radix(&p.as_span().as_str()[2..], 16).unwrap()
}

fn octnum(p: &Pair<Rule>) -> i64 {
    i64::from_str_radix(&p.as_span().as_str()[1..], 8).unwrap()
}

fn binnum(p: &Pair<Rule>) -> i64 {
    i64::from_str_radix(&p.as_span().as_str()[2..], 2).unwrap()
}

fn int(p: &Pair<Rule>) -> i64 {
    i64::from_str(&p.as_span().as_str()).unwrap()
}

fn float(p: &Pair<Rule>) -> f64 {
    f64::from_str(&p.as_span().as_str()).unwrap()
}

fn varassign(rush: &mut RuSh, p: &Pair<Rule>) {
    let mut var: (String, Option<Index>);
    var = (String::new(), None);
    let mut val = Value::I(i64::from(0));
    let mut local: bool = false;
    print!("{:?}", p);
    //~ for inner in p.into_inner() {
    //~ match inner.as_rule() {
    //~ Rule::localvar => local = true,
    //~ Rule::varname => var = varname(p, inner.as_span().as_str().to_string()),
    //~ Rule::float => val = Value::F(float(&inner)),
    //~ Rule::hexnum => val = Value::I(hexnum(&inner)),
    //~ Rule::octnum => val = Value::I(octnum(&inner)),
    //~ Rule::binnum => val = Value::I(binnum(&inner)),
    //~ Rule::int => val = Value::I(int(&inner)),
    //~ // FIXME ensure we cover everything correctly
    //~ _ => panic!()
    //~ }
    //~ }
    //FIXME take care of local variables
    match var.1 {
        // array case
        Some(i) => rush.shell_array_vars.set(&var.0, i, val),
        // simple variable
        None => rush.shell_vars.set(
            var.0,
            Variable {
                value: val,
                access: Access::ReadWrite,
            },
        ),
    }
}

/// Parsing loop - To be rewritten
pub fn parse(rush: &mut RuSh, input: &str) {
    // fn parse(rule: R, input: &str) -> Result<Pairs<R>, Error<R>>
    let pest = Script::parse(Rule::bla, input).unwrap_or_else(|e| panic!("{}", e));
    for line in pest.flatten() {
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
            Rule::varassign => println!("varassign: {:?}", line.as_span().as_str()), //varassign(rush, &line),

            //println!("varassing: {}", line.as_span().as_str()),
            //~ Rule::floatvarassign => {
            //~ let mut var = String::new();
            //~ let mut val = 0.0;
            //~ let mut index: Option<Index> = None;
            //~ for inner in line.into_inner() {
            //~ match inner.as_rule() {
            //~ Rule::varname => var = inner.as_span().as_str().to_string(),
            //~ Rule::float => val = f64::from_str(inner.as_span().as_str()).unwrap(),
            //~ // FIXME - index management needs to be implemented.
            //~ Rule::index => {
            //~ index = Some(Index::I(0));
            //~ unimplemented!();
            //~ }
            //~ _ => unreachable!(),
            //~ }
            //~ }
            //~ rush.shell_vars.set(
            //~ var,
            //~ Variable {
            //~ value: Value::F(val),
            //~ access: Access::ReadWrite,
            //~ },
            //~ );
            //~ }
            //~ Rule::binvarassign => println!("binvarassign: {}", line.as_span().as_str()),
            //~ Rule::intvarassign => {
            //~ let mut var = String::new();
            //~ let mut val = 0;
            //~ for inner in line.into_inner() {
            //~ match inner.as_rule() {
            //~ Rule::varname => var = inner.as_span().as_str().to_string(),
            //~ Rule::int => val = i64::from_str(inner.as_span().as_str()).unwrap(),
            //~ _ => unreachable!(),
            //~ }
            //~ }
            //~ rush.shell_vars.set(
            //~ var,
            //~ Variable {
            //~ value: Value::I(val),
            //~ access: Access::ReadWrite,
            //~ },
            //~ );
            //~ }
            //~ Rule::hexvarassign => println!("hexvarassign: {}", line.as_span().as_str()),
            //~ Rule::octvarassign => println!("octvarassign: {}", line.as_span().as_str()),
            //~ Rule::stringvarassign => {
            //~ let mut var = String::new();
            //~ let mut val = String::new();
            //~ for inner in line.into_inner() {
            //~ match inner.as_rule() {
            //~ Rule::varname => var = inner.as_span().as_str().to_string(),
            //~ // FIXME: add variables management
            //~ Rule::dquoted => val = inner.as_span().as_str().to_string(),
            //~ Rule::squoted => val = inner.as_span().as_str().to_string(),
            //~ // FIXME: add execution management
            //~ Rule::btquoted => val = inner.as_span().as_str().to_string(),
            //~ Rule::nonquoted => val = inner.as_span().as_str().to_string(),
            //~ _ => unreachable!(),
            //~ }
            //~ }
            //~ rush.shell_vars.set(
            //~ var,
            //~ Variable {
            //~ value: Value::S(val),
            //~ access: Access::ReadWrite,
            //~ },
            //~ );
            //~ }
            Rule::nop => println!("nop: {}", line.as_span().as_str()),

            //~ Rule::localfloatvarassign => {
            //~ println!("localfloatvarassign: {}", line.as_span().as_str())
            //~ }
            //~ Rule::localbinvarassign => println!("localbinvarassign: {}", line.as_span().as_str()),
            //~ Rule::localintvarassign => println!("localintvarassign: {}", line.as_span().as_str()),
            //~ Rule::localhexvarassign => println!("localhexvarassign: {}", line.as_span().as_str()),
            //~ Rule::localoctvarassign => println!("localoctvarassign: {}", line.as_span().as_str()),
            //~ Rule::localstringvarassign => {
            //~ println!("localstringvarassign: {}", line.as_span().as_str())
            //~ }
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
