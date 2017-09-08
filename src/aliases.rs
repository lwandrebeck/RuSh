//
// aliases.rs
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

/// RuSh aliases management begins here.
///
/// aliases.rs contains aliases structure and affiliated methods.
/// aliases (un)setting, update methods.

extern crate seahash;

use std::collections::HashMap;
//use std::hash::BuildHasher;
//use variables::{Variable, Variables, Value, SeaRandomState};
use variables::SeaRandomState;

/// Opt structure is defined here to store options status (setopt)
pub struct Aliases {
    aliases: HashMap<String, String, SeaRandomState>
}

/// Needed methods for Aliases.
impl Aliases {
    /// Get an alias value from its name. Returns value as Option<String>.
    /// Returns None if key does not exist.
    pub fn get(&self, key: &str) -> Option<String> {
        match self.aliases.get(key) {
            Some(val) => Some(val.to_string()),
            None => None
        }
    }

    /// Set an alias for a given name. Entry is created if needed, otherwise value is updated.
    pub fn set(&mut self, key: String, value: String) {
        self.aliases.insert(key, value);
    }

    /// Define a couple default aliases.
    pub fn init_aliases () -> Aliases {
        let mut aliases = Aliases { aliases: HashMap::with_capacity_and_hasher(30, SeaRandomState) };
        aliases.set("egrep".to_string(), "egrep --color=auto".to_string());
        aliases.set("fgrep".to_string(), "fgrep --color=auto".to_string());
        aliases.set("grep".to_string(), "grep --color=auto".to_string());
        aliases.set("l.".to_string(), "ls -d .* --color=auto".to_string());
        aliases.set("ll".to_string(), "ls -l --color=auto".to_string());
        aliases.set("ls".to_string(), "ls --color=auto".to_string());
        aliases.set("which".to_string(), "alias | /usr/bin/which --tty-only --read-alias --show-dot --show-tilde".to_string());
        aliases.set("xzegrep".to_string(), "xzegrep --color=auto".to_string());
        aliases.set("xzfgrep".to_string(), "xzfgrep --color=auto".to_string());
        aliases.set("xzgrep".to_string(), "xzgrep --color=auto".to_string());
        aliases.set("zegrep".to_string(), "zegrep --color=auto".to_string());
        aliases.set("zfgrep".to_string(), "zfgrep --color=auto".to_string());
        aliases.set("zgrep".to_string(), "zgrep --color=auto".to_string());
        aliases
    }
}
