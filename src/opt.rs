//
// opt.rs
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

/// RuSh opt (shopt, set) management begins here.
///
/// opt.rs contains OptionRW and Opt structures and affiliated methods.
/// opt (un)setting, update methods for both shopt and options.
extern crate seahash;

use crate::variables::Access;
use crate::variables::SeaRandomState;
use std::collections::HashMap;

/// OptionRW structure, allows to store values for a given option.
pub struct OptionRW {
    /// Is the option set or not ?
    set: bool,
    /// Is it allowed to change option value ?
    access: Access,
}
/// Methods linked to OptionRW
impl OptionRW {
    /// Classic getter returning if the OptionRW is set or not.
    pub fn get(&self) -> bool {
        self.set
    }
}

/// Opt structure is defined here to store options status (setopt and set)
//#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Opt {
    /// Opt has a single field
    opt: HashMap<String, OptionRW, SeaRandomState>,
}

/// Methods for Opt structure.
impl Opt {
    /// Get an opt value from `Opt`. Returns value as Option<OptionRW>.
    ///
    /// # Examples
    /// ```rust
    /// use Opt;
    /// use opt::OptionRW;
    /// let mut o = Opt::init_set_options();
    /// match o.get("notify") {
    ///     Some(v) => { assert_eq!(v.set, false); assert_eq!(v.access, Access::ReadWrite); },
    ///     None => panic!("notify set option should be defined.")
    /// }
    /// ```
    pub fn get(&self, key: &str) -> Option<OptionRW> {
        match self.opt.get(key) {
            Some(val) => {
                let var = OptionRW {
                    set: val.set,
                    access: val.access.clone(),
                };
                Some(var)
            }
            None => None,
        }
    }

    /// Set a opt for a given name. Opt is created if needed, otherwise value is updated.
    ///
    /// # Examples
    /// ```rust
    /// use Opt;
    /// use opt::OptionRW;
    /// let mut o = Opt::init_shopt_options();
    /// o.set(String::from("opttest"), OptionRW { set: false, access: access:ReadOnly });
    /// match o.get("opttest") {
    ///     Some(v) => { assert_eq!(v.set, false); assert_eq!(v.access, access:ReadOnly); },
    ///     None => panic!("opttest shopt option should be defined.")
    /// }
    /// ```
    /// # Tests

    pub fn set(&mut self, key: String, value: OptionRW) {
        self.opt.insert(key, value);
    }

    /// Initialize default shopt_options(&mut self). Returns `Opt`.
    pub fn init_shopt_options() -> Opt {
        //! 43 shopt entries. Allocate a big enough HashMap.
        let mut options = Opt {
            opt: HashMap::with_capacity_and_hasher(43, SeaRandomState),
        };
        // initialize default options.
        // If set, a command name that is the name of a directory is executed as if it were the argument to the cd command.  This option is only used by interactive shells.
        options.set(
            "autocd".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, an argument to the cd builtin command that is not a directory is assumed to be the name of a variable whose value is the directory to change to.
        options.set(
            "cdable_vars".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If  set, minor errors in the spelling of a directory component in a cd command will be corrected.  The errors checked for are transposed characters, a missing character, and one character too many.  If a correction is found, the corrected file name is printed, and the command proceeds.  This option is only used by interactive shells.
        options.set(
            "cdspell".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, rush checks that a command found in the hash table exists before trying to execute it.  If a hashed command no longer exists, a normal path search is performed.
        options.set(
            "checkhash".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, rush lists the status of any stopped and running jobs before exiting an interactive shell.  If any jobs are running, this causes the exit to be deferred until a second exit is attempted without an intervening command (see JOB CONTROL above).  The shell always postpones exiting if any jobs are stopped.
        options.set(
            "checkjobs".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, rush checks the window size after each command and, if necessary, updates the values of LINES and COLUMNS.
        options.set(
            "checkwinsize".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        // If set, rush attempts to save all lines of a multiple-line command in the same history entry.  This allows easy re-editing of multi-line commands.
        options.set(
            "cmdhist".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        // If set, rush changes its behavior to that of bash version 3.1 with respect to quoted arguments to the [[ conditional command's =~ operator. HIGHLY LIKELY NEVER TO BE TAKEN CARE OF.
        options.set(
            "compat31".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, rush  changes its behavior to that of bash version 3.2 with respect to locale-specific string comparison when using the [[ conditional command's < and > operators. HIGHLY LIKELY NEVER TO BE TAKEN CARE OF.
        options.set(
            "compat32".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, rush changes its behavior to that of bash version 4.0 with respect to locale-specific string comparison when using the [[ conditional command's <  and  > operators (see previous item) and the effect of interrupting a command list. HIGHLY LIKELY NEVER TO BE TAKEN CARE OF.
        options.set(
            "compat40".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, rush, when in posix mode, treats a single quote in a double-quoted parameter expansion as a special character. The single quotes must match (an even number) and the characters between the single quotes are considered quoted.  This is the behavior of posix mode through version 4.1. The default bash behavior remains as in previous versions. HIGHLY LIKELY NEVER TO BE TAKEN CARE OF.
        options.set(
            "compat41".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, rush does not process the replacement string in the pattern substitution word expansion using quote removal. HIGHLY LIKELY NEVER TO BE TAKEN CARE OF.
        options.set(
            "compat42".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, rush quotes all shell metacharacters in filenames and directory names when performing completion. If not set, bash removes metacharacters such as the dollar sign from the set of characters that will be quoted in completed filenames when these metacharacters appear in shell variable references in words to be completed. This means that dollar signs in variable names that expand to directories will not be quoted; however, any dollar signs appearing in filenames will not be quoted, either. This is active only when bash is using backslashes to quote completed filenames. This variable is set by default, which is the default bash behavior in versions through 4.2.
        options.set(
            "complete_fullquote".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        // If  set, rush replaces directory names with the results of word expansion when performing filename completion. This changes the contents of the readline editing buffer. If not set, rush attempts to preserve what the user typed.
        options.set(
            "direxpand".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, rush attempts spelling correction on directory names during word completion if the directory name initially supplied does not exist.
        options.set(
            "dirspell".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, rush includes filenames beginning with a `.' in the results of pathname expansion.
        options.set(
            "dotglob".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, a non-interactive shell will not exit if it cannot execute the file specified as an argument to the exec builtin command. An interactive shell does not exit if exec fails.
        options.set(
            "execfail".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, aliases are expanded as described above under ALIASES.  This option is enabled by default for interactive shells.
        options.set(
            "expand_aliases".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        // If set, behavior intended for use by debuggers is enabled:
        // 1.     The -F option to the declare builtin displays the source file name and line number corresponding to each function name supplied as an argument.
        // 2.     If the command run by the DEBUG trap returns a non-zero value, the next command is skipped and not executed.
        // 3.     If the command run by the DEBUG trap returns a value of 2, and the shell is executing in a subroutine (a shell function or a shell script executed by the . or source builtins), a call to return is simulated.
        // 4.     BASH_ARGC and BASH_ARGV are updated as described in their descriptions above.
        // 5.     Function tracing is enabled:  command substitution, shell functions, and subshells invoked with ( command ) inherit the DEBUG and RETURN traps.
        // 6.     Error tracing is enabled:  command substitution, shell functions, and subshells invoked with ( command ) inherit the ERR trap.
        options.set(
            "extdebug".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, the extended pattern matching features described above under Pathname Expansion are enabled.
        options.set(
            "extglob".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        // If set, $'string' and $"string" quoting is performed within ${parameter} expansions enclosed in double quotes.  This option is enabled by default.
        options.set(
            "extquote".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        // If set, patterns which fail to match filenames during pathname expansion result in an expansion error.
        options.set(
            "failglob".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If  set, the  suffixes  specified by the FIGNORE shell variable cause words to be ignored when performing word completion even if the ignored words are the only possible completions. See SHELL VARIABLES above for a description of FIGNORE.  This option is enabled by default.
        options.set(
            "force_fignore".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        // If set, range expressions used in pattern matching bracket expressions (see Pattern Matching above) behave as if in the traditional C locale when performing comparisons.  That is, the current locale's collating sequence is not taken into account, so b will not collate between A and B, and upper-case and lower-case ASCII characters will collate together. FIXME: Rust does use UTF-8.
        options.set(
            "globasciiranges".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If  set, the pattern ** used in a pathname expansion context will match all files and zero or more directories and subdirectories.  If the pattern is followed by a /, only directories and subdirectories match.
        options.set(
            "globstar".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, shell error messages are written in the standard GNU error message format.
        options.set(
            "gnu_errfmt".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, the history list is appended to the file named by the value of the HISTFILE variable when the shell exits, rather than overwriting the file.
        options.set(
            "histappend".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        // If set, and readline is being used, a user is given the opportunity to re-edit a failed history substitution.
        options.set(
            "histreedit".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, and readline is being used, the results of history substitution are not immediately passed to the shell parser. Instead, the resulting line is loaded into the readline editing buffer, allowing further modification.
        options.set(
            "histverify".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, and readline is being used, rush will attempt to perform hostname completion when a word containing a @ is being completed. This is enabled by default.
        options.set(
            "hostcomplete".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        // If set, rush will send SIGHUP to all jobs when an interactive login shell exits.
        options.set(
            "huponexit".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, allow a word beginning with # to cause that word and all remaining characters on that line to be ignored in an interactive shell. This option is enabled by default.
        options.set(
            "interactive_comments".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        // If set, and job control is not active, the shell runs the last command of a pipeline not executed in the background in the current shell environment.
        options.set(
            "lastpipe".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, and the cmdhist option is enabled, multi-line commands are saved to the history with embedded newlines rather than using semicolon separators where possible.
        options.set(
            "lithist".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // The shell sets this option if it is started as a login shell (see INVOCATION above). The value may not be changed.
        options.set(
            "login_shell".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, and a file that rush is checking for mail has been accessed since the last time it was checked, the message ``The mail in mailfile has been read'' is displayed.
        options.set(
            "mailwarn".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, and readline is being used, rush will not attempt to search the PATH for possible completions when completion is attempted on an empty line.
        options.set(
            "no_empty_cmd_completion".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, rush matches filenames in a case-insensitive fashion when performing pathname expansion.
        options.set(
            "nocaseglob".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, rush matches patterns in a case-insensitive fashion when performing matching while executing case or [[ conditional commands.
        options.set(
            "nocasematch".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, rush allows patterns which match no files to expand to a null string, rather than themselves.
        options.set(
            "nullglob".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, the programmable completion facilities are enabled.  This option is enabled by default.
        options.set(
            "progcomp".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        // If set, prompt strings undergo parameter expansion, command substitution, arithmetic expansion, and quote removal after being expanded. This option is enabled by default.
        options.set(
            "promptvars".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        // The shell sets this option if it is started in restricted mode. The value may not be changed. This is not reset when the startup files are executed, allowing the startup files to discover whether or not a shell is restricted.
        options.set(
            "restricted_shell".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, the shift builtin prints an error message when the shift count exceeds the number of positional parameters.
        options.set(
            "shift_verbose".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        // If set, the source (.) builtin uses the value of PATH to find the directory containing the file supplied as an argument. This option is enabled by default.
        options.set(
            "sourcepath".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        // If set, the echo builtin expands backslash-escape sequences by default.
        options.set(
            "xpg_echo".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options
    }

    /// Initialize default set_options(&mut self)
    /// According to help set and echo $-
    /// himBHs (or himBH depending on bash version it seems) is not to be found anywhere !?
    pub fn init_set_options() -> Opt {
        // 27 set options. Allocate a big enough HashMap.
        let mut options = Opt {
            opt: HashMap::with_capacity_and_hasher(27, SeaRandomState),
        };
        options.set(
            "allexport".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "braceexpand".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "emacs".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "errexit".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "errtrace".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "functrace".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "hashall".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "histexpand".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "history".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "ignoreeof".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "interactive-comments".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "keyword".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "monitor".to_string(),
            OptionRW {
                set: true,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "noclobber".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "noexec".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "noglob".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "nolog".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "notify".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "nounset".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "onecmd".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "physical".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "pipefail".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "posix".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "privileged".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "verbose".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "vi".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options.set(
            "xtrace".to_string(),
            OptionRW {
                set: false,
                access: Access::ReadWrite,
            },
        );
        options
    }
}

#[cfg(test)]
mod tests {
    use crate::opt::Opt;
    use crate::opt::OptionRW;
    use crate::variables::Access;

    #[test]
    fn test_opt_get() {
        let o = Opt::init_set_options();
        match o.get("notify") {
            Some(v) => {
                assert_eq!(v.set, false);
                assert_eq!(v.access, Access::ReadWrite);
            }
            None => panic!("notify set option should be defined."),
        }
    }

    #[test]
    fn test_opt_init_set_options() {
        let o = Opt::init_set_options();
        match o.get("xtrace") {
            Some(v) => {
                assert_eq!(v.set, false);
                assert_eq!(v.access, Access::ReadWrite);
            }
            None => panic!("xtrace set option should be defined."),
        }
    }

    #[test]
    fn test_opt_init_shopt_options() {
        let o = Opt::init_shopt_options();
        match o.get("histappend") {
            Some(v) => {
                assert_eq!(v.set, true);
                assert_eq!(v.access, Access::ReadWrite);
            }
            None => panic!("histappend shopt option should be defined."),
        }
    }

    #[test]
    fn test_opt_set() {
        let mut o = Opt::init_shopt_options();
        o.set(
            String::from("opttest"),
            OptionRW {
                set: false,
                access: Access::ReadOnly,
            },
        );
        match o.get("opttest") {
            Some(v) => {
                assert_eq!(v.set, false);
                assert_eq!(v.access, Access::ReadOnly);
            }
            None => panic!("opttest shopt option should be defined."),
        }
    }
}
