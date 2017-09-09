//
// prompt.rs
//
// Copyright 2015-2017 Laurent Wandrebeck <l.wandrebeck@quelquesmots.fr>
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
use variables::{Variable, Variables, Value};

/// Public structure Prompt
pub struct Prompt {
    /// prompt is stored in a String.
    pub prompt: String
}

/// Methods for Prompt.
impl Prompt {
    /// Get `Prompt` from `Variables`. Returns interpreted `Prompt`.
	///
	/// # Examples
	/// ```rust
	/// let mut var = Variables { vars: HashMap::with_capacity_and_hasher(200, SeaRandomState) };
	/// var.set("PS1", Variable { value: Value::S(""), rw: true);
	/// p = Prompt.get("PS1");
	/// println!("Prompt var value interpreted from PS1 variable is: {}", p.prompt);
	/// ```
    pub fn get(vars: &mut Variables, p: &str) -> Prompt {
        let mut aslash = false;
        let mut pt = String::new();
        let ps: String = match p {
            "PS1" => { match vars.get(p) {
                Some(ps1) => { match ps1 {
                                    Variable { value: Value::S(s), rw: true } => s,
                                    _ => String::from("")
                               }
                             }
                None => { let ps1 = "\\u@\\h \\W\\$ ".to_string(); vars.set(String::from("PS1"), Variable { value: Value::S(ps1), rw:true }); "\\u@\\h \\W\\$ ".to_string() }
                } },
            "PS2" => { match vars.get(p) {
                Some(ps2) => { match ps2 {
                                    Variable { value: Value::S(s), rw: true } => s,
                                    _ => String::from("")
                               }
                             }
                None => { let ps2 = ">".to_string(); vars.set(String::from("PS2"), Variable { value: Value::S(ps2), rw:true }); ">".to_string() }
                } },
            "PS3" => { match vars.get(p) {
                Some(ps3) => { match ps3 {
                                    Variable { value: Value::S(s), rw: true } => s,
                                    _ => String::from("")
                               }
                             }
                None => { let ps3 = ">".to_string(); vars.set(String::from("PS3"), Variable { value: Value::S(ps3), rw:true }); ">".to_string() }
                } },
            "PS4" => { match vars.get(p) {
                Some(ps4) => { match ps4 {
                                    Variable { value: Value::S(s), rw: true } => s,
                                    _ => String::from("")
                               }
                             }
                None => { let ps4 = ">".to_string(); vars.set(String::from("PS4"), Variable { value: Value::S(ps4), rw:true }); ">".to_string() }
                } },
            _     => { panic!("prompt env var should not have that value !"); },
        };
        let pr: Vec<(usize, char)> = ps.char_indices().collect();
        for i in pr {
            if i.1 == '\\' {
                aslash = true;
                continue;
            }
            if aslash {
                aslash = false;
                match i {
                    // See http://ss64.com/bash/syntax-prompt.html
                    (index, 'd') => { let dt = Local::now(); pt.push_str(&dt.format("%a %b %e").to_string()); },
                    // TODO fix 'h' (short hostname must be returned)
                    (index, 'h') => pt.push_str(&vars.get("HOSTNAME").unwrap().gets()),
                    (index, 'H') => pt.push_str(&vars.get("HOSTNAME").unwrap().gets()),
                    (index, 'j') => unimplemented!(),
                    (index, 'l') => pt.push_str(&vars.get("TERM").unwrap().gets()),
                    (index, 's') => pt.push_str(&vars.get("0").unwrap().gets()),
                    (index, 't') => { let dt = Local::now(); pt.push_str(&dt.format("%H:%M:%S").to_string()); },
                    (index, 'T') => { let dt = Local::now(); pt.push_str(&dt.format("%I:%M:%S").to_string()); },
                    (index, '@') => { let dt = Local::now(); pt.push_str(&dt.format("%I:%M:%S%P").to_string()); },
                    (index, 'u') => pt.push_str(&vars.get("USERNAME").unwrap().gets()),
                    (index, 'v') => pt.push_str("0.0.1"), // FIXME
                    (index, 'V') => pt.push_str("0.0.1"), // FIXME
                    (index, 'w') => pt.push_str(&vars.get("PWD").unwrap().gets()),
                    (index, 'W') => pt.push_str(&vars.get("PWD").unwrap().gets()),
                    (index, '!') => unimplemented!(),
                    (index, '#') => unimplemented!(),
                    (index, '$') => { match vars.get("UID").unwrap().geti() {
                                        0 => pt.push_str("#"),
                                        _ => pt.push_str("$")
                                      }
                                    }
                    (index, '0'...'8') => unimplemented!(),
                    (index, 'n') => pt.push_str("\n"),
                    (index, 'r') => pt.push_str("\r"),
                    (index, 'e') => unimplemented!(),
                    (index, 'a') => unimplemented!(),
                    (index, '\\') => pt.push_str("\\"),
                    (index, '[') => unimplemented!(),
                    (index, ']') => unimplemented!(),
                    (_, _) => continue,
                }
            } else {
              pt.push(i.1);
            }
        }
        let prompt = Prompt { prompt: pt };
        prompt
    }
}

#[cfg(test)]
mod tests {
	use prompt::Prompt;
	use Variables;
	use variables::{Variable, Value};
	
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
