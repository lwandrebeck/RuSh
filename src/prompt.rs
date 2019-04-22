//
// prompt.rs
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

/// RuSh prompt management begins here.
///
/// prompt.rs contains prompt affiliated methods.
/// prompt is parsed here too.
extern crate chrono;
extern crate pest;
extern crate rand;

use self::chrono::*;
use crate::prompt::pest::Parser;
use crate::rush::RuSh;
use crate::variables::{Access, Value, Variable};
use pest_derive::Parser;

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
    /// Get `Prompt` from `RuSh`. Returns interpreted `Prompt`.
    ///
    /// # Examples
    /// ```rust
    /// use crate::RuSh;
    /// use RuSh::prompt::Prompt;
    /// use RuSh::variables::{Variables, Variable, Value};
    /// let mut r = RuSh::rush::RuSh::default();
    /// let mut p = Prompt::get(&mut r, "PS2");
    /// assert_eq!(p.prompt, ">");
    /// p = Prompt::get(&mut r, "PS3");
    /// assert_eq!(p.prompt, ">");
    /// p = Prompt::get(&mut r, "PS4");
    /// assert_eq!(p.prompt, ">");
    /// ```
    pub fn get(rush: &mut RuSh, p: &str) -> Prompt {
        let mut pt = String::new();
        let ps: String = match p {
            "PS1" => match rush.shell_vars.get(p) {
                Some(ps1) => match ps1 {
                    Variable {
                        value: Value::S(s),
                        access: Access::ReadWrite,
                    } => s,
                    _ => String::from(""),
                },
                None => {
                    let ps1 = "[\\u@\\h \\W]\\$ ".to_string();
                    rush.shell_vars.set(
                        String::from("PS1"),
                        Variable {
                            value: Value::S(ps1),
                            access: Access::ReadWrite,
                        },
                    );
                    "[\\u@\\h \\W]$ ".to_string()
                }
            },
            "PS2" => match rush.shell_vars.get(p) {
                Some(ps2) => match ps2 {
                    Variable {
                        value: Value::S(s),
                        access: Access::ReadWrite,
                    } => s,
                    _ => String::from(""),
                },
                None => {
                    let ps2 = ">".to_string();
                    rush.shell_vars.set(
                        String::from("PS2"),
                        Variable {
                            value: Value::S(ps2),
                            access: Access::ReadWrite,
                        },
                    );
                    ">".to_string()
                }
            },
            "PS3" => match rush.shell_vars.get(p) {
                Some(ps3) => match ps3 {
                    Variable {
                        value: Value::S(s),
                        access: Access::ReadWrite,
                    } => s,
                    _ => String::from(""),
                },
                None => {
                    let ps3 = ">".to_string();
                    rush.shell_vars.set(
                        String::from("PS3"),
                        Variable {
                            value: Value::S(ps3),
                            access: Access::ReadWrite,
                        },
                    );
                    ">".to_string()
                }
            },
            "PS4" => match rush.shell_vars.get(p) {
                Some(ps4) => match ps4 {
                    Variable {
                        value: Value::S(s),
                        access: Access::ReadWrite,
                    } => s,
                    _ => String::from(""),
                },
                None => {
                    let ps4 = ">".to_string();
                    rush.shell_vars.set(
                        String::from("PS4"),
                        Variable {
                            value: Value::S(ps4),
                            access: Access::ReadWrite,
                        },
                    );
                    ">".to_string()
                }
            },
            _ => {
                panic!("prompt env var should not have that value !");
            }
        };
        // Parse prompt variable value element by element, and evaluate each when needed.
        let pest = Script::parse(Rule::prompt, &ps).unwrap_or_else(|e| panic!("{}", e));
        for element in pest {
            match element.as_rule() {
                Rule::normal_prompt => pt.push_str(element.as_span().as_str()),
                Rule::prompt_date => {
                    let dt = Local::now();
                    pt.push_str(&dt.format("%a %b %e").to_string());
                }
                Rule::prompt_host => {
                    let host = &rush.shell_vars.get("HOSTNAME").unwrap().gets();
                    let pos = host.find('.').unwrap_or_else(|| host.len());
                    pt.push_str(&host[..pos]);
                }
                Rule::prompt_hostname => {
                    pt.push_str(&rush.shell_vars.get("HOSTNAME").unwrap().gets())
                }
                Rule::prompt_jobs => unimplemented!(),
                Rule::prompt_term_dev_basename => {
                    pt.push_str(&rush.shell_vars.get("TERM").unwrap().gets())
                }
                Rule::prompt_time_s24 => {
                    let dt = Local::now();
                    pt.push_str(&dt.format("%H:%M:%S").to_string());
                }
                Rule::prompt_time_s12 => {
                    let dt = Local::now();
                    pt.push_str(&dt.format("%I:%M:%S").to_string());
                }
                Rule::prompt_time_12 => {
                    let dt = Local::now();
                    pt.push_str(&dt.format("%I:%M:%S%P").to_string());
                }
                Rule::prompt_username => {
                    pt.push_str(&rush.shell_vars.get("USERNAME").unwrap().gets())
                }
                Rule::prompt_version => pt.push_str("0.0.0.0-alpha0-x86_64-redhat-linux-gnu"), // FIXME
                Rule::prompt_version_patch => pt.push_str("0.0.0.0-alpha0-x86_64-redhat-linux-gnu"), // FIXME
                Rule::prompt_pwd => pt.push_str(&rush.shell_vars.get("PWD").unwrap().gets()),
                Rule::prompt_pwd_basename => {
                    let path = &rush.shell_vars.get("PWD").unwrap().gets();
                    let pos = path.rfind('/').unwrap_or(0);
                    if pos == 0 {
                        pt.push_str(&path[pos..]);
                    } else {
                        pt.push_str(&path[pos + 1..]);
                    }
                }
                Rule::prompt_history_command_number => match rush.shell_vars.get("HISTCMD") {
                    Some(phcn) => pt.push_str(&phcn.geti().to_string()),
                    None => pt.push_str("0"),
                },
                Rule::prompt_command_number => pt.push_str(&rush.cmd_nb.to_string()),
                Rule::prompt_is_root => match rush.shell_vars.get("UID").unwrap().geti() {
                    0 => pt.push_str("#"),
                    _ => pt.push_str("$"),
                },
                Rule::prompt_octal => pt.push_str(
                    &u8::from_str_radix(element.as_span().as_str(), 8)
                        .unwrap()
                        .to_string(),
                ),
                Rule::prompt_newline => pt.push_str("\n"),
                Rule::prompt_car_ret => pt.push_str("\r"),
                // 33 is octal value of ascii espace character
                Rule::prompt_esc => pt.push_str(&u8::from_str_radix("33", 8).unwrap().to_string()),
                // 7 is octal value of ascii bell character
                Rule::prompt_bell => pt.push_str(&u8::from_str_radix("7", 8).unwrap().to_string()),
                Rule::prompt_backslash => pt.push_str("\\"),
                Rule::prompt_non_print => unimplemented!(),
                Rule::prompt_end_non_print => unimplemented!(),
                _ => panic!(),
            };
        }
        Prompt { prompt: pt }
    }
}

#[cfg(test)]
mod tests {
    use crate::prompt::Prompt;
    use crate::rush::RuSh;

    #[test]
    fn test_get() {
        let mut rush = RuSh::default();
        let mut p = Prompt::get(&mut rush, "PS2");
        assert_eq!(p.prompt, ">");
        p = Prompt::get(&mut rush, "PS3");
        assert_eq!(p.prompt, ">");
        p = Prompt::get(&mut rush, "PS4");
        assert_eq!(p.prompt, ">");
    }
}
