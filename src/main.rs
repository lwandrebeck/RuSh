/*
 * main.rs
 *
 * Copyright 2015-2017 Laurent Wandrebeck <l.wandrebeck@quelquesmots.fr>
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

//! RuSh begins here.
//!
//! main.rs contains the very beginning of RuSh.
//! aliases, options structures, environment are defined/set.
//! prompt is updated there.

// Include other files.
mod variables;
mod prompt;

extern crate libc;
extern crate rustyline;
extern crate term;
extern crate seahash;
extern crate rand;
extern crate chrono;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::str;
use std::collections::HashMap;
use pest::Parser;
use variables::Variables;

/// pest grammar inclusion. dummy const so that .pest file changes are taken care of.
const _GRAMMAR: &'static str = include_str!("rush.pest"); // relative to src path

#[derive(Parser)]
#[grammar = "rush.pest"]
struct Script;

/// Structure to store variable value and rw state.
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct ValueRW {
    value: String,
    rw: bool
}

/// Structure to store option state and rw state.
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct OptionRW {
    set: bool,
    rw: bool
}

/// Core structure containing everything needed for RuSh
//#[derive(Hash, Eq, PartialEq, Debug)]
pub struct RuSh {
    /// aliases: Stored as HashMap<&str, &str>
    aliases: HashMap<String, String, variables::SeaRandomState>,
    /// shopt_options: autocd, etc. See man bash, shopt options. Stored as HashMap<&str, &bool>
    shopt_options: HashMap<String, OptionRW, variables::SeaRandomState>,
    /// set_options: allexport, braceexpand, etc. See man bash, set command. Stored as HashMap<&str, &bool>
    set_options: HashMap<String, OptionRW, variables::SeaRandomState>,
    /// shell_vars: RUSH, RUSHPID, etc. See man bash, shell variables. Stored as HashMap<&str, &str>
    shell_vars: Variables,
    /// Command history. Stored as History from rustyline
    history: rustyline::history::History,
    /// line case, needed for prompt management
    line_case: u8,
    /// command number, may be needed by prompt
    cmd_nb: u64,
    /// prompt contents. Stored as a simple String.
    prompt: prompt::Prompt,
}

impl Default for RuSh {
    fn default() -> RuSh {
        let mut shell = RuSh {
            // 15 aliases by default in Fedora 26.
            aliases: RuSh::init_aliases(),
            // 46 shopt options by default, so let’s have a big enough HashMap to store these.
            shopt_options: RuSh::init_shopt_options(),
            // 27 set options by default, so let’s have a big enough HashMap to store these.
            set_options: RuSh::init_set_options(),
            // 100 or so shell vars are defined upon startup. Let’s say most scripts do use up to 200 vars, so let’s alloc enough.
            shell_vars: Variables::init_shell_vars(),
            // TODO set history size
            // rl.set_history_max_len(1000);
            history: rustyline::history::History::new(),
            // prompt management. TODO
            line_case: 1,
            cmd_nb: 0,
            prompt: prompt::Prompt { prompt: String::from("") }
        };
        shell.prompt = prompt::Prompt::get(&mut shell.shell_vars, "PS1");
        //let mut stdin = io::stdin();
        let mut rl = rustyline::Editor::<()>::new();
        // take care of SECOND env var
        //thread::spawn(move ||  {
        //    loop {
        //        thread::sleep(time::Duration::new(1, 0));
        //        match shell.shell_vars.entry("SECONDS".into()) {
        //          Entry::Occupied(val) =>  { let mut s:u64 = val.get().value.parse().unwrap_or(0); s += 1; shell.shell_vars.insert("SECONDS".to_string(), ValueRW { value: s.to_string(), rw: true }); },
        //          Entry::Vacant(val) => { shell.shell_vars.insert("SECONDS".to_string(), ValueRW { value: "1".to_string(), rw: true }); }
        //      }
        //  }
        //});
        loop {
            let line = rl.readline(&shell.prompt.prompt);
            match line {
                Ok(input) => {
                    // TODO fix history management
                    rl.add_history_entry(&input);
                    let pest = Script::parse_str(Rule::bla, &input).unwrap_or_else(|e| panic!("{}", e));
                    for line in pest {
                        match line.as_rule() {
                            Rule::float => println!("float: {}", line.into_span().as_str()),
                            Rule::binnum => println!("binnum: {}", line.into_span().as_str()),
                            Rule::hexnum => println!("hexnum: {}", line.into_span().as_str()),
                            Rule::octnum => println!("octnum: {}", line.into_span().as_str()),
                            Rule::int => println!("int: {}", line.into_span().as_str()),
                            Rule::squoted => println!("squoted: {}", line.into_span().as_str()),
                            Rule::dquoted => println!("dquoted: {}", line.into_span().as_str()),
                            Rule::btquoted => println!("btquoted: {}", line.into_span().as_str()),
                            Rule::nonquoted => println!("nonquoted: {}", line.into_span().as_str()),
                            Rule::shebang => println!("shebang: {}", line.into_span().as_str()),
                            Rule::comment => println!("comment: {}", line.into_span().as_str()),
                            Rule::floatvarassign => println!("floatvarassign: {}", line.into_span().as_str()),
                            Rule::binvarassign => println!("binvarassign: {}", line.into_span().as_str()),
                            Rule::intvarassign => println!("intvarassign: {}", line.into_span().as_str()),
                            Rule::hexvarassign => println!("hexvarassign: {}", line.into_span().as_str()),
                            Rule::octvarassign => println!("octvarassign: {}", line.into_span().as_str()),
                            Rule::stringvarassign => println!("stringvarassign: {}", line.into_span().as_str()),
                            _ => unreachable!() // ident rule is silent and cannot be reached
                        };
                    }
                    shell.cmd_nb +=1;
                    },
                Err(_) => { break }
            }
            match shell.line_case {
                1 => shell.prompt = prompt::Prompt::get(&mut shell.shell_vars, "PS1"),
                2 => shell.prompt = prompt::Prompt::get(&mut shell.shell_vars, "PS2"),
                3 => shell.prompt = prompt::Prompt::get(&mut shell.shell_vars, "PS3"),
                4 => shell.prompt = prompt::Prompt::get(&mut shell.shell_vars, "PS4"),
                _ => panic!("wrong line_case value.")
            }
        }
    shell
    }
}

impl RuSh {
    pub fn init_shopt_options() -> HashMap<String, OptionRW, variables::SeaRandomState> {
        // 46 shopt entries. Allocate a big enough HashMap.
        let mut options = HashMap::with_capacity_and_hasher(46, variables::SeaRandomState);
        // initialize default options.
        // If set, a command name that is the name of a directory is executed as if it were the argument to the cd command.  This option is only used by interactive shells.
        options.insert("autocd".to_string(), OptionRW { set: false, rw: false });
        // If set, an argument to the cd builtin command that is not a directory is assumed to be the name of a variable whose value is the directory to change to.
        options.insert("cdable_vars".to_string(), OptionRW { set: false, rw: false });
        // If  set, minor errors in the spelling of a directory component in a cd command will be corrected.  The errors checked for are transposed characters, a missing character, and one character too many.  If a correction is found, the corrected file name is printed, and the command proceeds.  This option is only used by interactive shells.
        options.insert("cdspell".to_string(), OptionRW { set: false, rw: false });
        // If set, rush checks that a command found in the hash table exists before trying to execute it.  If a hashed command no longer exists, a normal path search is performed.
        options.insert("checkhash".to_string(), OptionRW { set: false, rw: false });
        // If set, rush lists the status of any stopped and running jobs before exiting an interactive shell.  If any jobs are running, this causes the exit to be deferred until a second exit is attempted without an intervening command (see JOB CONTROL above).  The shell always postpones exiting if any jobs are stopped.
        options.insert("checkjobs".to_string(), OptionRW { set: false, rw: false });
        // If set, rush checks the window size after each command and, if necessary, updates the values of LINES and COLUMNS.
        options.insert("checkwinsize".to_string(), OptionRW { set: true, rw: false });
        // If set, rush attempts to save all lines of a multiple-line command in the same history entry.  This allows easy re-editing of multi-line commands.
        options.insert("cmdhist".to_string(), OptionRW { set: true, rw: false });
        // If set, rush changes its behavior to that of bash version 3.1 with respect to quoted arguments to the [[ conditional command's =~ operator. HIGHLY LIKELY NEVER TO BE TAKEN CARE OF.
        options.insert("compat31".to_string(), OptionRW { set: false, rw: false });
        // If set, rush  changes its behavior to that of bash version 3.2 with respect to locale-specific string comparison when using the [[ conditional command's < and > operators. HIGHLY LIKELY NEVER TO BE TAKEN CARE OF.
        options.insert("compat32".to_string(), OptionRW { set: false, rw: false });
        // If set, rush changes its behavior to that of bash version 4.0 with respect to locale-specific string comparison when using the [[ conditional command's <  and  > operators (see previous item) and the effect of interrupting a command list. HIGHLY LIKELY NEVER TO BE TAKEN CARE OF.
        options.insert("compat40".to_string(), OptionRW { set: false, rw: false });
        // If set, rush, when in posix mode, treats a single quote in a double-quoted parameter expansion as a special character. The single quotes must match (an even number) and the characters between the single quotes are considered quoted.  This is the behavior of posix mode through version 4.1. The default bash behavior remains as in previous versions. HIGHLY LIKELY NEVER TO BE TAKEN CARE OF.
        options.insert("compat41".to_string(), OptionRW { set: false, rw: false });
        // If set, rush does not process the replacement string in the pattern substitution word expansion using quote removal. HIGHLY LIKELY NEVER TO BE TAKEN CARE OF.
        options.insert("compat42".to_string(), OptionRW { set: false, rw: false });
        // If set, rush quotes all shell metacharacters in filenames and directory names when performing completion. If not set, bash removes metacharacters such as the dollar sign from the set of characters that will be quoted in completed filenames when these metacharacters appear in shell variable references in words to be completed. This means that dollar signs in variable names that expand to directories will not be quoted; however, any dollar signs appearing in filenames will not be quoted, either. This is active only when bash is using backslashes to quote completed filenames. This variable is set by default, which is the default bash behavior in versions through 4.2.
        options.insert("complete_fullquote".to_string(), OptionRW { set: true, rw: false });
        // If  set, rush replaces directory names with the results of word expansion when performing filename completion. This changes the contents of the readline editing buffer. If not set, rush attempts to preserve what the user typed.
        options.insert("direxpand".to_string(), OptionRW { set: false, rw: false });
        // If set, rush attempts spelling correction on directory names during word completion if the directory name initially supplied does not exist.
        options.insert("dirspell".to_string(), OptionRW { set: false, rw: false });
        // If set, rush includes filenames beginning with a `.' in the results of pathname expansion.
        options.insert("dotglob".to_string(), OptionRW { set: false, rw: false });
        // If set, a non-interactive shell will not exit if it cannot execute the file specified as an argument to the exec builtin command. An interactive shell does not exit if exec fails.
        options.insert("execfail".to_string(), OptionRW { set: false, rw: false });
        // If set, aliases are expanded as described above under ALIASES.  This option is enabled by default for interactive shells.
        options.insert("expand_aliases".to_string(), OptionRW { set: true, rw: false });
        // If set, behavior intended for use by debuggers is enabled:
        // 1.     The -F option to the declare builtin displays the source file name and line number corresponding to each function name supplied as an argument.
        // 2.     If the command run by the DEBUG trap returns a non-zero value, the next command is skipped and not executed.
        // 3.     If the command run by the DEBUG trap returns a value of 2, and the shell is executing in a subroutine (a shell function or a shell script executed by the . or source builtins), a call to return is simulated.
        // 4.     BASH_ARGC and BASH_ARGV are updated as described in their descriptions above.
        // 5.     Function tracing is enabled:  command substitution, shell functions, and subshells invoked with ( command ) inherit the DEBUG and RETURN traps.
        // 6.     Error tracing is enabled:  command substitution, shell functions, and subshells invoked with ( command ) inherit the ERR trap.
        options.insert("extdebug".to_string(), OptionRW { set: false, rw: false });
        // If set, the extended pattern matching features described above under Pathname Expansion are enabled.
        options.insert("extglob".to_string(), OptionRW { set: true, rw: false });
        // If set, $'string' and $"string" quoting is performed within ${parameter} expansions enclosed in double quotes.  This option is enabled by default.
        options.insert("extquote".to_string(), OptionRW { set: true, rw: false });
        // If set, patterns which fail to match filenames during pathname expansion result in an expansion error.
        options.insert("failglob".to_string(), OptionRW { set: false, rw: false });
        // If  set, the  suffixes  specified by the FIGNORE shell variable cause words to be ignored when performing word completion even if the ignored words are the only possible completions. See SHELL VARIABLES above for a description of FIGNORE.  This option is enabled by default.
        options.insert("force_fignore".to_string(), OptionRW { set: true, rw: false });
        // If set, range expressions used in pattern matching bracket expressions (see Pattern Matching above) behave as if in the traditional C locale when performing comparisons.  That is, the current locale's collating sequence is not taken into account, so b will not collate between A and B, and upper-case and lower-case ASCII characters will collate together. FIXME: Rust does use UTF-8.
        options.insert("globasciiranges".to_string(), OptionRW { set: false, rw: false });
        // If  set, the pattern ** used in a pathname expansion context will match all files and zero or more directories and subdirectories.  If the pattern is followed by a /, only directories and subdirectories match.
        options.insert("globstar".to_string(), OptionRW { set: false, rw: false });
        // If set, shell error messages are written in the standard GNU error message format.
        options.insert("gnu_errfmt".to_string(), OptionRW { set: false, rw: false });
        // If set, the history list is appended to the file named by the value of the HISTFILE variable when the shell exits, rather than overwriting the file.
        options.insert("histappend".to_string(), OptionRW { set: true, rw: false });
        // If set, and readline is being used, a user is given the opportunity to re-edit a failed history substitution.
        options.insert("histreedit".to_string(), OptionRW { set: false, rw: false });
        // If set, and readline is being used, the results of history substitution are not immediately passed to the shell parser. Instead, the resulting line is loaded into the readline editing buffer, allowing further modification.
        options.insert("histverify".to_string(), OptionRW { set: false, rw: false });
        // If set, and readline is being used, rush will attempt to perform hostname completion when a word containing a @ is being completed. This is enabled by default.
        options.insert("hostcomplete".to_string(), OptionRW { set: true, rw: false });
        // If set, rush will send SIGHUP to all jobs when an interactive login shell exits.
        options.insert("huponexit".to_string(), OptionRW { set: false, rw: false });
        // If set, allow a word beginning with # to cause that word and all remaining characters on that line to be ignored in an interactive shell. This option is enabled by default.
        options.insert("interactive_comments".to_string(), OptionRW { set: true, rw: false });
        // If set, and job control is not active, the shell runs the last command of a pipeline not executed in the background in the current shell environment.
        options.insert("lastpipe".to_string(), OptionRW { set: false, rw: false });
        // If set, and the cmdhist option is enabled, multi-line commands are saved to the history with embedded newlines rather than using semicolon separators where possible.
        options.insert("lithist".to_string(), OptionRW { set: false, rw: false });
        // The shell sets this option if it is started as a login shell (see INVOCATION above). The value may not be changed.
        options.insert("login_shell".to_string(), OptionRW { set: false, rw: false });
        // If set, and a file that rush is checking for mail has been accessed since the last time it was checked, the message ``The mail in mailfile has been read'' is displayed.
        options.insert("mailwarn".to_string(), OptionRW { set: false, rw: false });
        // If set, and readline is being used, rush will not attempt to search the PATH for possible completions when completion is attempted on an empty line.
        options.insert("no_empty_cmd_completion".to_string(), OptionRW { set: false, rw: false });
        // If set, rush matches filenames in a case-insensitive fashion when performing pathname expansion.
        options.insert("nocaseglob".to_string(), OptionRW { set: false, rw: false });
        // If set, rush matches patterns in a case-insensitive fashion when performing matching while executing case or [[ conditional commands.
        options.insert("nocasematch".to_string(), OptionRW { set: false, rw: false });
        // If set, rush allows patterns which match no files to expand to a null string, rather than themselves.
        options.insert("nullglob".to_string(), OptionRW { set: false, rw: false });
        // If set, the programmable completion facilities are enabled.  This option is enabled by default.
        options.insert("progcomp".to_string(), OptionRW { set: true, rw: false });
        // If set, prompt strings undergo parameter expansion, command substitution, arithmetic expansion, and quote removal after being expanded. This option is enabled by default.
        options.insert("promptvars".to_string(), OptionRW { set: true, rw: false });
        // The shell sets this option if it is started in restricted mode. The value may not be changed. This is not reset when the startup files are executed, allowing the startup files to discover whether or not a shell is restricted.
        options.insert("restricted_shell".to_string(), OptionRW { set: false, rw: false });
        // If set, the shift builtin prints an error message when the shift count exceeds the number of positional parameters.
        options.insert("shift_verbose".to_string(), OptionRW { set: false, rw: false });
        // If set, the source (.) builtin uses the value of PATH to find the directory containing the file supplied as an argument. This option is enabled by default.
        options.insert("sourcepath".to_string(), OptionRW { set: true, rw: false });
        // If set, the echo builtin expands backslash-escape sequences by default.
        options.insert("xpg_echo".to_string(), OptionRW { set: false, rw: false });
        options
    }

    /// Initialize set_options(&mut self) {
    /// according to help set and echo $-
    /// himBH. i not to be found anywhere !?
    pub fn init_set_options() -> HashMap<String, OptionRW, variables::SeaRandomState> {
        // 27 set options. Allocate a big enough HashMap.
        let mut options = HashMap::with_capacity_and_hasher(27, variables::SeaRandomState);
        options.insert("allexport".to_string(), OptionRW { set: false, rw: false });
        options.insert("braceexpand".to_string(), OptionRW { set: true, rw: false });
        options.insert("emacs".to_string(), OptionRW { set: false, rw: false });
        options.insert("errexit".to_string(), OptionRW { set: false, rw: false });
        options.insert("errtrace".to_string(), OptionRW { set: false, rw: false });
        options.insert("functrace".to_string(), OptionRW { set: false, rw: false });
        options.insert("hashall".to_string(), OptionRW { set: true, rw: false });
        options.insert("histexpand".to_string(), OptionRW { set: true, rw: false });
        options.insert("history".to_string(), OptionRW { set: false, rw: false });
        options.insert("ignoreeof".to_string(), OptionRW { set: false, rw: false });
        options.insert("interactive-comments".to_string(), OptionRW { set: false, rw: false });
        options.insert("keyword".to_string(), OptionRW { set: false, rw: false });
        options.insert("monitor".to_string(), OptionRW { set: true, rw: false });
        options.insert("noclobber".to_string(), OptionRW { set: false, rw: false });
        options.insert("noexec".to_string(), OptionRW { set: false, rw: false });
        options.insert("noglob".to_string(), OptionRW { set: false, rw: false });
        options.insert("nolog".to_string(), OptionRW { set: false, rw: false });
        options.insert("notify".to_string(), OptionRW { set: false, rw: false });
        options.insert("nounset".to_string(), OptionRW { set: false, rw: false });
        options.insert("onecmd".to_string(), OptionRW { set: false, rw: false });
        options.insert("physical".to_string(), OptionRW { set: false, rw: false });
        options.insert("pipefail".to_string(), OptionRW { set: false, rw: false });
        options.insert("posix".to_string(), OptionRW { set: false, rw: false });
        options.insert("privileged".to_string(), OptionRW { set: false, rw: false });
        options.insert("verbose".to_string(), OptionRW { set: false, rw: false });
        options.insert("vi".to_string(), OptionRW { set: false, rw: false });
        options.insert("xtrace".to_string(), OptionRW { set: false, rw: false });
        options
    }

    /// Initialize default aliases
    pub fn init_aliases () -> HashMap<String, String, variables::SeaRandomState> {
        let mut aliases = HashMap::with_capacity_and_hasher(30, variables::SeaRandomState);
        aliases.insert("egrep".to_string(), "egrep --color=auto".to_string());
        aliases.insert("fgrep".to_string(), "fgrep --color=auto".to_string());
        aliases.insert("grep".to_string(), "grep --color=auto".to_string());
        aliases.insert("l.".to_string(), "ls -d .* --color=auto".to_string());
        aliases.insert("ll".to_string(), "ls -l --color=auto".to_string());
        aliases.insert("ls".to_string(), "ls --color=auto".to_string());
        aliases.insert("which".to_string(), "alias | /usr/bin/which --tty-only --read-alias --show-dot --show-tilde".to_string());
        aliases.insert("xzegrep".to_string(), "xzegrep --color=auto".to_string());
        aliases.insert("xzfgrep".to_string(), "xzfgrep --color=auto".to_string());
        aliases.insert("xzgrep".to_string(), "xzgrep --color=auto".to_string());
        aliases.insert("zegrep".to_string(), "zegrep --color=auto".to_string());
        aliases.insert("zfgrep".to_string(), "zfgrep --color=auto".to_string());
        aliases.insert("zgrep".to_string(), "zgrep --color=auto".to_string());
        aliases
    }
}

/// main loop. the fun begins here !
fn main() {
    RuSh::default();
}

