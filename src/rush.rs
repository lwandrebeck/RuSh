//
// rush.rs
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

pub use crate::aliases::Aliases;
pub use crate::opt::Opt;
pub use crate::prompt::Prompt;
pub use crate::variables::Variables;
#[allow(unused_imports)]
use pest::Parser;
/// pest grammar inclusion. dummy const so that .pest file changes are taken care of.
#[derive(Parser)]
#[grammar = "rush.pest"]
#[allow(dead_code)]
struct Script;

/// Core structure containing everything needed for RuSh
pub struct RuSh {
    /// aliases: Stored as Aliases { aliases: HashMap<String, String> }
    pub aliases: Aliases,
    /// shopt_options: autocd, etc. See man bash, shopt options. Stored as Opt { opt: HashMap<String, <set: bool, rw: bool>> }
    pub shopt_options: Opt,
    /// set_options: allexport, braceexpand, etc. See man bash, set command. Stored as HashMap<String, <bool, bool>>
    pub set_options: Opt,
    /// shell_vars: RUSH, RUSHPID, etc. See man bash, shell variables. Stored as HashMap<String, <i64 or f64 or String, bool>>
    pub shell_vars: Variables,
    /// Command history. Stored as History from rustyline
    pub history: rustyline::history::History,
    /// line case, needed for prompt management
    pub line_case: u8,
    /// command number, may be needed by prompt
    pub cmd_nb: u64,
    /// prompt contents. Stored as Prompt { prompt: String }
    pub prompt: Prompt,
    /// information about RuSh version: major minor patch build release MACHTYPE
    pub versinfo: (u8, u8, u8, u8, String, String),
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
            prompt: Prompt {
                prompt: String::from(""),
            },
            /// actual RuSh version informations
            versinfo: (0, 0, 0, 0, "alpha0".to_string(), "x86_64-redhat-linux-gnu".to_string()), //FIXME MACHTYPE
        };
        shell
    }
}
