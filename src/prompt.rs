//
// prompt.rs
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

/// RuSh prompt management begins here.
///
/// prompt.rs contains prompt affiliated methods.
/// prompt is parsed here too.
extern crate chrono;
extern crate rand;

use self::chrono::*;
use crate::variables::{Value, Variable, Variables};
use pest::Parser;

/// pest grammar inclusion. dummy const so that .pest file changes are taken care of.
#[derive(Parser)]
#[grammar = "prompt.pest"]
struct Script;

/// Public structure Prompt
pub struct Prompt {
    /// prompt is stored in a String.
    pub prompt: String,
}

/// Methods for Prompt.
impl Prompt {
    /// Get `Prompt` from `Variables`. Returns interpreted `Prompt`.
    ///
    /// # Examples
    /// ```rust
    /// use prompt::Prompt;
    /// use Variables;
    /// use variables::{Variable, Value};
    /// let mut vars = Variables::init_shell_vars();
    /// let mut p = Prompt::get(&mut vars, "PS2");
    /// assert_eq!(p.prompt, ">");
    /// p = Prompt::get(&mut vars, "PS3");
    /// assert_eq!(p.prompt, ">");
    /// p = Prompt::get(&mut vars, "PS4");
    /// assert_eq!(p.prompt, ">");
    /// ```
    pub fn get(vars: &mut Variables, p: &str) -> Prompt {
        let mut aslash = false;
        let mut pt = String::new();
        let ps: String = match p {
            "PS1" => match vars.get(p) {
                Some(ps1) => match ps1 {
                    Variable {
                        value: Value::S(s),
                        rw: true,
                    } => s,
                    _ => String::from(""),
                },
                None => {
                    let ps1 = "\\u@\\h \\W\\$ ".to_string();
                    vars.set(
                        String::from("PS1"),
                        Variable {
                            value: Value::S(ps1),
                            rw: true,
                        },
                    );
                    "\\u@\\h \\W\\$ ".to_string()
                }
            },
            "PS2" => match vars.get(p) {
                Some(ps2) => match ps2 {
                    Variable {
                        value: Value::S(s),
                        rw: true,
                    } => s,
                    _ => String::from(""),
                },
                None => {
                    let ps2 = ">".to_string();
                    vars.set(
                        String::from("PS2"),
                        Variable {
                            value: Value::S(ps2),
                            rw: true,
                        },
                    );
                    ">".to_string()
                }
            },
            "PS3" => match vars.get(p) {
                Some(ps3) => match ps3 {
                    Variable {
                        value: Value::S(s),
                        rw: true,
                    } => s,
                    _ => String::from(""),
                },
                None => {
                    let ps3 = ">".to_string();
                    vars.set(
                        String::from("PS3"),
                        Variable {
                            value: Value::S(ps3),
                            rw: true,
                        },
                    );
                    ">".to_string()
                }
            },
            "PS4" => match vars.get(p) {
                Some(ps4) => match ps4 {
                    Variable {
                        value: Value::S(s),
                        rw: true,
                    } => s,
                    _ => String::from(""),
                },
                None => {
                    let ps4 = ">".to_string();
                    vars.set(
                        String::from("PS4"),
                        Variable {
                            value: Value::S(ps4),
                            rw: true,
                        },
                    );
                    ">".to_string()
                }
            },
            _ => {
                panic!("prompt env var should not have that value !");
            }
        };
        let pest = Script::parse(Rule::prompt, &ps).unwrap_or_else(|e| panic!("{}", e));
        for element in pest {
			match element.as_rule() {
				Rule::normal_prompt => pt.push_str(element.as_span().as_str()),
				Rule::prompt_date => {
                        let dt = Local::now();
                        pt.push_str(&dt.format("%a %b %e").to_string());
                    },
				Rule::prompt_host => pt.push_str(&vars.get("HOSTNAME").unwrap().gets()), //FIXME
				Rule::prompt_hostname => pt.push_str(&vars.get("HOSTNAME").unwrap().gets()),
				Rule::prompt_jobs => unimplemented!(),
				Rule::prompt_term_dev_basename => pt.push_str(&vars.get("TERM").unwrap().gets()),
				Rule::prompt_time_s24 => {
                        let dt = Local::now();
                        pt.push_str(&dt.format("%H:%M:%S").to_string());
                    },
				Rule::prompt_time_s12 => {
                        let dt = Local::now();
                        pt.push_str(&dt.format("%I:%M:%S").to_string());
                    },
				Rule::prompt_time_12 => {
                        let dt = Local::now();
                        pt.push_str(&dt.format("%I:%M:%S%P").to_string());
                    },
				Rule::prompt_username => pt.push_str(&vars.get("USERNAME").unwrap().gets()),
				Rule::prompt_version => pt.push_str("0.0.1"), // FIXME
				Rule::prompt_version_patch => pt.push_str("0.0.1"), // FIXME
				Rule::prompt_pwd => pt.push_str(&vars.get("PWD").unwrap().gets()),
				Rule::prompt_pwd_basename => pt.push_str(&vars.get("PWD").unwrap().gets()), //FIXME
				Rule::prompt_history_command_number => unimplemented!(),
				Rule::prompt_command_number => unimplemented!(),
				Rule::prompt_is_root => match vars.get("UID").unwrap().geti() {
                        0 => pt.push_str("#"),
                        _ => pt.push_str("$"),
                    },
				Rule::prompt_octal => unimplemented!(),
				Rule::prompt_newline => pt.push_str("\n"),
				Rule::prompt_car_ret => pt.push_str("\r"),
				Rule::prompt_esc => unimplemented!(),
				Rule::prompt_bell => unimplemented!(),
				Rule::prompt_backslash => pt.push_str("\\"),
				Rule::prompt_non_print => unimplemented!(),
				Rule::prompt_end_non_print => unimplemented!(),
				_ => panic!(),
			};
		};
        Prompt { prompt: pt }
    }
}

#[cfg(test)]
mod tests {
    use crate::prompt::Prompt;
    use crate::variables::Variables;
    //use crate::variables::{Variable, Value};

    #[test]
    fn test_get() {
        let mut vars = Variables::init_shell_vars();
        let mut p = Prompt::get(&mut vars, "PS2");
        assert_eq!(p.prompt, ">");
        p = Prompt::get(&mut vars, "PS3");
        assert_eq!(p.prompt, ">");
        p = Prompt::get(&mut vars, "PS4");
        assert_eq!(p.prompt, ">");
    }
}
