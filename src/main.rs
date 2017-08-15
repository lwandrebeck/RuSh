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

extern crate libc;
extern crate rustyline;
#[macro_use]
extern crate term;
extern crate seahash;
extern crate rand;
extern crate chrono;
#[macro_use]
extern crate pest;

use std::io;
use std::io::{stdin, stdout, Write};
use std::{env, thread, time, str};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
//use std::hash::BuildHasherDefault;
use std::hash::BuildHasher;
use libc::{geteuid, getpid, getppid, getuid, getgid, getlogin, c_char, size_t, c_int};
use std::path::PathBuf;
use self::rand::Rng;
use self::chrono::*;
use std::ffi::CStr;
use pest::inputs::{Input, Position, Span, StringInput};

/// For seahash maps.
pub struct SeaRandomState;
//type Hasher = seahash::SeaHasher;

impl BuildHasher for SeaRandomState {
    type Hasher = seahash::SeaHasher;
    fn build_hasher(&self) -> seahash::SeaHasher {
        seahash::SeaHasher::new()
    }
}

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

/// In order to get a proper execution speed, let’s have 3 default types
pub struct Floats {
    value: f32,
    rw: bool
}

pub struct Uints {
    value: u64,
    rw: bool
}

pub struct Sints {
    value: i64,
    rw: bool
}

/// Core structure containing everything needed for RuSh
//#[derive(Hash, Eq, PartialEq, Debug)]
pub struct RuSh {
    /// aliases: Stored as HashMap<&str, &str>
    aliases: HashMap<String, String, SeaRandomState>,
    /// shopt_options: autocd, etc. See man bash, shopt options. Stored as HashMap<&str, &bool>
    shopt_options: HashMap<String, OptionRW, SeaRandomState>,
    /// set_options: allexport, braceexpand, etc. See man bash, set command. Stored as HashMap<&str, &bool>
    set_options: HashMap<String, OptionRW, SeaRandomState>,
    /// shell_vars: RUSH, RUSHPID, etc. See man bash, shell variables. Stored as HashMap<&str, &str>
    shell_vars: HashMap<String, ValueRW, SeaRandomState>,
    /// shell_floats: store float vars in native format (speeeeeeeed)
    shell_floats: HashMap<String, Floats, SeaRandomState>,
    /// Shell_uints: store uints vars in native format (speeeeeeeed)
    shell_uints: HashMap<String, Uints, SeaRandomState>,
    /// Shell_sints: store sints vars in native format (speeeeeeeed)
    shell_sints: HashMap<String, Sints, SeaRandomState>,
    /// Command history. Stored as History from rustyline
    history: rustyline::history::History,
    /// line case, needed for prompt management
    line_case: u8,
    /// command number, may be needed by prompt
    cmd_nb: u64,
    /// prompt contents. Stored as a simple String.
    prompt: String,
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
            shell_vars: RuSh::init_shell_vars(),
            // Shell variables, float format.
            shell_floats: RuSh::init_shell_floats(),
            // Shell variables, unsigned int format.
            shell_uints: RuSh::init_shell_uints(),
            // Shell variables, signed int format.
            shell_sints: RuSh::init_shell_sints(),
            // commands history is stored using rustyline.
            // TODO set history size
            // rl.set_history_max_len(1000);
            history: rustyline::history::History::new(),
            // prompt management. TODO
            line_case: 1,
            cmd_nb: 0,
            prompt: "".to_string(),
        };
        let mut stdin = io::stdin();
        let mut rl = rustyline::Editor::<()>::new();
        // take care of SECOND env var
        //thread::spawn(move ||  {
        //    loop {
        //        thread::sleep(time::Duration::new(1, 0));
        //        match shell.shell_vars.entry("SECONDS".into()) {
		//			Entry::Occupied(val) =>  { let mut s:u64 = val.get().value.parse().unwrap_or(0); s += 1; shell.shell_vars.insert("SECONDS".to_string(), ValueRW { value: s.to_string(), rw: true }); },
		//			Entry::Vacant(val) => { shell.shell_vars.insert("SECONDS".to_string(), ValueRW { value: "1".to_string(), rw: true }); }
		//		}
		//	}
		//});
        loop {
            shell.prompt_update();
            let line = rl.readline(&shell.prompt);
            match line {
                Ok(input) => {
                    rl.add_history_entry(&input);
                    let mut pest = StringInput::new(input);
                    // TODO/FIXME : rewrite the basic parser
                    //if handle_command(&input) {
                    //    shell
                    shell.cmd_nb +=1;
                    },
                Err(_) => { break }
            }
        }
    shell
    }
}

impl RuSh {
    pub fn init_shopt_options() -> HashMap<String, OptionRW, SeaRandomState> {
        // 46 shopt entries. Allocate a big enough HashMap.
        let mut options = HashMap::with_capacity_and_hasher(46, SeaRandomState);
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
    pub fn init_set_options() -> HashMap<String, OptionRW, SeaRandomState> {
        // 27 set options. Allocate a big enough HashMap.
        let mut options = HashMap::with_capacity_and_hasher(27, SeaRandomState);
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

    /// Init float shell variables 
    pub fn init_shell_floats() -> HashMap<String, Floats, SeaRandomState> {
        let mut variables = HashMap::with_capacity_and_hasher(200, SeaRandomState);
        variables.insert("RUSH".to_string(), Floats { value: 1.0, rw: true });
        variables
    }

    /// Init uint shell variables
    pub fn init_shell_uints() -> HashMap<String, Uints, SeaRandomState> {
        let mut variables = HashMap::with_capacity_and_hasher(200, SeaRandomState);
        variables.insert("RUSH".to_string(), Uints { value: 1, rw: true });
        variables
    }

    /// Init sint shell variables
    pub fn init_shell_sints() -> HashMap<String, Sints, SeaRandomState> {
        let mut variables = HashMap::with_capacity_and_hasher(200, SeaRandomState);
        variables.insert("RUSH".to_string(), Sints { value: -1, rw: true });
        variables
    }

    /// Default shell variables are set here, following the bash way.
    pub fn init_shell_vars() -> HashMap<String, ValueRW, SeaRandomState> {
        let mut variables = HashMap::with_capacity_and_hasher(200, SeaRandomState);
        // see man bash (Shell variables)
        // Expands to the full filename used to invoke this instance of rush.
        match env::current_exe() {
            Ok(ce) => variables.insert("RUSH".to_string(), ValueRW { value: ce.to_str().unwrap().to_string(), rw: true }), //as_os_str().to_os_string().into_string().unwrap())
            Err(e) => variables.insert("RUSH".to_string(), ValueRW { value: "".to_string(), rw: true }),
        };
        // A colon-separated list of enabled shell options. Each word in the list is a valid argument for the -s option to the shopt builtin command. The options appearing in RUSHOPTS are those reported as on by shopt. If this variable is in the environment when rush starts up, each shell option in the list will be enabled before reading any startup files. This variable is read-only.
        // TODO RUSHOPTS
        // Expands to the process ID of the current rush process. This differs from $$ under certain circumstances, such as subshells that do not require rush to be re-initialized.
        unsafe {
            let pid = getpid();
            variables.insert("RUSHPID".to_string(), ValueRW { value: pid.to_string(), rw: true });
        }
        // An associative array variable whose members correspond to the internal list of aliases as maintained by the alias builtin. Elements added to this array appear in the alias list; unsetting array elements cause aliases to be removed from the alias list.
        // TODO RUSH_ALIASES
        // An array variable whose values are the number of parameters in each frame of the current bash execution call stack. The number of parameters to the current subroutine (shell function or script  executed with . or source) is at the top of the stack. When a subroutine is executed, the number of parameters passed is pushed onto RUSH_ARGC. The shell sets RUSH_ARGC only when in extended debugging mode (see the description of the extdebug option to the shopt builtin)
        // TODO RUSH_ARGC
        // An array variable containing all of the parameters in the current rush execution call stack. The final parameter of the last subroutine call is at the top of the stack; the first parameter of the  initial call is at the bottom. When a subroutine is executed, the parameters supplied are pushed onto RUSH_ARGV. The shell sets RUSH_ARGV only when in extended debugging mode (see the description of the extdebug option to the shopt builtin)
        // TODO RUSH_ARGV
        // An associative array variable whose members correspond to the internal hash table of commands as maintained by the hash builtin. Elements added to this array appear in the hash table; unsetting array elements cause commands to be removed from the hash table.
        // TODO RUSH_CMDS
        // The  command  currently  being  executed or about to be executed, unless the shell is executing a command as the result of a trap, in which case it is the command executing at the time of the trap.
        variables.insert("RUSH_COMMAND".to_string(), ValueRW { value: "".to_string(), rw: true });
        // The command argument to the -c invocation option.
        // TODO RUSH_EXECUTION_STRING
        // An array variable whose members are the line numbers in source files where each corresponding member of FUNCNAME was invoked.  ${RUSH_LINENO[$i]} is the line number in the source file (${RUSH_SOURCE[$i+1]}) where ${FUNCNAME[$i]} was called (or ${RUSH_LINENO[$i-1]} if referenced within another shell function). Use LINENO to obtain the current line number.
        // TODO RUSH_LINENO
        // An array variable whose members are assigned by the =~ binary operator to the [[ conditional command. The element with index 0 is the portion of the string matching the entire regular expression.  The element with index n is the portion of the string matching the nth parenthesized subexpression. This variable is read-only.
        // TODO RUSH_REMATCH
        // An array variable whose members are the source filenames where the corresponding shell function names in the FUNCNAME array variable are defined. The shell function ${FUNCNAME[$i]} is defined in the file ${RUSH_SOURCE[$i]} and called from ${RUSH_SOURCE[$i+1]}.
        // TODO RUSH_SOURCE
        // Incremented by one within each subshell or subshell environment when the shell begins executing in that environment. The initial value is 0.
        variables.insert("RUSH_SUBSHELL".to_string(), ValueRW { value: "0".to_string(), rw: true });
        // A readonly array variable whose members hold version information for this instance of rush.  The values assigned to the array members are as follows:
        // RUSH_VERSINFO[0]        The major version number (the release).
        // RUSH_VERSINFO[1]        The minor version number (the version).
        // RUSH_VERSINFO[2]        The patch level.
        // RUSH_VERSINFO[3]        The build version.
        // RUSH_VERSINFO[4]        The release status (e.g., beta1).
        // RUSH_VERSINFO[5]        The value of MACHTYPE.
        // TODO RUSH_VERSINFO -> need MACHTYPE, which needs HOSTTYPE, VENDOR, OSTYPE
        // Expands to a string describing the version of this instance of bash
        let versinfo = vec!["0", "0", "1", "1", "alpha0", "TODO"]; // FIXME -> use some global var.
        variables.insert("RUSH_VERSION".to_string(), ValueRW { value: "0.0.1-alpha0".to_string(), rw: true }); // FIXME -> use some global var.
        // An index into ${COMP_WORDS} of the word containing the current cursor position. This variable is available only in shell functions invoked by the programmable completion facilities.
        // TODO COMP_CWORD
        // The key (or final key of a key sequence) used to invoke the current completion function.
        // TODO COMP_KEY
        // The current command line.  This variable is available only in shell functions and external commands invoked by the programmable completion facilities.
        // TODO COMP_LINE
        // The index of the current cursor position relative to the beginning of the current command. If the current cursor position is at the end of the current command, the value of this variable is equal to ${#COMP_LINE}.  This variable is available only in shell functions and external commands invoked by the programmable completion facilities.
        // TODO COMP_POINT
        // Set to an integer value corresponding to the type of completion attempted that caused a completion function to be called: TAB, for normal completion, ?, for listing completions after successive tabs, !, for listing alternatives on partial word completion, @, to list completions if the word is not unmodified, or %, for menu completion. This variable is available only in shell functions and external commands invoked by the programmable completion facilities.
        // TODO COMP_TYPE
        // The  set of characters that the readline library treats as word separators when performing word completion. If COMP_WORDBREAKS is unset, it loses its special properties, even if it is subsequently reset.
        // TODO COMP_WORDBREAKS
        // An array variable consisting of the individual words in the current command line. The line is split into words as readline would split it, using COMP_WORDBREAKS as described above.  This variable is available only in shell functions invoked by the programmable completion facilities.
        // TODO COMP_WORDS
        // An array variable created to hold the file descriptors for output from and input to an unnamed coprocess.
        // TODO COPROC
        // An  array  variable (see Arrays below) containing the current contents of the directory stack. Directories appear in the stack in the order they are displayed by the dirs builtin. Assigning to members of this array variable may be used to modify directories already in the stack, but the pushd and popd builtins must be used to add and remove directories. Assignment to this variable will not change the current directory. If DIRSTACK is unset, it loses its special properties, even if it is subsequently reset.
        // TODO DIRSTACK
        // Expands to the effective user ID of the current user, initialized at shell startup. This variable is readonly.
        unsafe {
            let euid = geteuid();
            variables.insert("EUID".to_string(), ValueRW { value: euid.to_string(), rw: true });
        }
        // An array variable containing the names of all shell functions currently in the execution call stack.  The element with index 0 is the name of any currently-executing shell function.  The bottom-most element (the one with the highest index) is "main".  This variable exists only when a shell function is executing.  Assignments to FUNCNAME have no effect and return an error status. If FUNCNAME is unset, it loses its special properties, even if it is subsequently reset.
        // TODO FUNCNAME
        // An array variable containing the list of groups of which the current user is a member.  Assignments to GROUPS have no effect and return an error status.  If GROUPS is unset, it loses its special properties, even if it is subsequently reset.
        // TODO GROUPS
        // The history number, or index in the history list, of the current command.  If HISTCMD is unset, it loses its special properties, even if it is subsequently reset.
        // TODO HISTCMD
        // Automatically set to the name of the current host.
        let mut bufc = vec![0u8; 40];
        extern "C" {
            fn gethostname(bufc: *mut c_char, len: size_t) -> c_int;
        }
        unsafe {
            gethostname(bufc.as_mut_ptr() as *mut c_char, 40);
        };
        match str::from_utf8(bufc.split(|x| *x == 0).next().unwrap()) {
            Ok(h) => variables.insert("HOSTNAME".to_string(), ValueRW { value: h.to_string(), rw: true }),
            Err(e) => variables.insert("HOSTNAME".to_string(), ValueRW { value: "localhost".to_string(), rw: true }),
        };
        //variables.insert("HOSTNAME".to_string(), str::from_utf8(bufc.split(|x| *x == 0).next()).unwrap_or("wtf".to_string()));
        // Automatically set to a string that uniquely describes the type of machine on which rush is executing.  The default is system-dependent.
        // TODO HOSTTYPE
        // Each time this parameter is referenced, the shell substitutes a decimal number representing the current sequential line number (starting with 1) within a script or function. When not in a script or function, the value substituted is not guaranteed to be meaningful. If LINENO is unset, it loses its special properties, even if it is subsequently reset.
        variables.insert("LINENO".to_string(), ValueRW { value: "1".to_string(), rw: true });
        // Automatically set to a string that fully describes the system type on which rush is executing, in the standard GNU cpu-company-system format. The default is system-dependent.
        // TODO MACHTYPE
        // An array variable created to hold the text read by the mapfile builtin when no variable name is supplied.
        // TODO MAPFILE
        // The previous working directory as set by the cd command.
        variables.insert("OLDPWD".to_string(), ValueRW { value: ".".to_string(), rw: true });
        // The value of the last option argument processed by the getopts builtin command.
        // TODO OPTARG
        // The index of the next argument to be processed by the getopts builtin command.
        // TODO OPTIND
        // Automatically set to a string that describes the operating system on which rush is executing.  The default is system-dependent.
        // TODO OSTYPE
        // An array variable containing a list of exit status values from the processes in the most-recently-executed foreground pipeline (which may contain only a single command).
        // TODO PIPESTATUS
        // The process ID of the shell's parent.  This variable is readonly.
        unsafe {
            let ppid = getppid();
            variables.insert("PPID".to_string(), ValueRW { value: ppid.to_string(), rw: true });
        }
        // The current working directory as set by the cd command.
        match env::current_dir() {
            Ok(cd) => variables.insert("PWD".to_string(), ValueRW { value: cd.to_str().unwrap().to_string(), rw: true }),
            Err(e) => variables.insert("PWD".to_string(), ValueRW { value: "/".to_string(), rw: true }),
        };
        //variables.insert("PWD".to_string(), env::current_dir().unwrap_or("/".to_string()));
        // Each time this parameter is referenced, a random integer between 0 and 32767 is generated. The sequence of random numbers may be initialized by assigning a value to RANDOM. If RANDOM is unset, it loses its special properties, even if it is subsequently reset.
        let mut rng = rand::thread_rng();
        if rng.gen() {
            variables.insert("RANDOM".to_string(), ValueRW { value: rng.gen::<i16>().to_string(), rw: true });
        }
        // The contents of the readline line buffer, for use with "bind -x".
        // TODO READLINE_LINE
        // The position of the insertion point in the readline line buffer, for use with "bind -x".
        // TODO READLINE_POINT
        // Set to the line of input read by the read builtin command when no arguments are supplied.
        // TODO REPLY
        // Each time this parameter is referenced, the number of seconds since shell invocation is returned.  If a value is assigned to SECONDS, the value returned upon subsequent references is the number of seconds since the assignment plus the value assigned. If SECONDS is unset, it loses its special properties, even if it is subsequently reset.
        variables.insert("SECONDS".to_string(), ValueRW { value: "0".to_string(), rw: true });
        // The full pathname to the shell is kept in this environment variable.  If it is not set when the shell starts, rush assigns to it the full pathname of the current user's login shell.
        match env::current_exe() {
            Ok(ce) => variables.insert("SHELL".to_string(), ValueRW { value: ce.to_str().unwrap().to_string(), rw: true }),
            Err(e) => panic!("Unable to get current_exe !"),
        };
        // A colon-separated list of enabled shell options. Each word in the list is a valid argument for the -o option to the set builtin command. The options appearing in SHELLOPTS are those reported as  on by set -o. If this variable is in the environment when rush starts up, each shell option in the list will be enabled before reading any startup files. This variable is read-only.
        // TODO SHELLOPTS
        // Incremented by one each time an instance of rush is started.
        match env::var("SHLVL") {
            Ok(lvl) => { let mut level:u32 = lvl.parse().unwrap(); level +=1; variables.insert("SHLVL".to_string(), ValueRW { value: level.to_string(), rw: true }); },
            Err(e) => { variables.insert("SHLVL".to_string(), ValueRW { value: "1".to_string(), rw: true }); },
        }
        // Expands to the user ID of the current user, initialized at shell startup. This variable is readonly.
        unsafe {
            let id = getuid();
            variables.insert("UID".to_string(), ValueRW { value: id.to_string(), rw: true });
        }
        // TODO variables used by the shell, see man bash.
        unsafe {
            let id = getgid();
            variables.insert("GID".to_string(), ValueRW { value: id.to_string(), rw: true });
        }
        unsafe {
            let log = getlogin();
            variables.insert("USERNAME".to_string(), ValueRW { value: CStr::from_ptr(log).to_string_lossy().into_owned(), rw: true });
            //variables.insert("USERNAME".to_string(), ValueRW { value: log.to_string(), rw: true });
        }
        variables.insert("HISTSIZE".to_string(), ValueRW { value: "1000".to_string(), rw: true });
        variables
    }

    /// update prompt
    pub fn prompt_update(&mut self) {
        let mut aslash = false;
        let mut pt = String::new();
        let mut ps: String = match self.line_case {
            1 => self.shell_vars.entry("PS1".into())
					.or_insert_with(|| ValueRW { value: "[\\u@\\h \\W]\\$ ".to_string(), rw: true })
					.value.clone(),
            2 => self.shell_vars.entry("PS2".into())
					.or_insert_with(|| ValueRW { value: ">".to_string(), rw: true })
					.value.clone(),
            3 => self.shell_vars.entry("PS3".into())
					.or_insert_with(|| ValueRW { value: ">".to_string(), rw: true })
					.value.clone(),
            4 => self.shell_vars.entry("PS4".into())
					.or_insert_with(|| ValueRW { value: ">".to_string(), rw: true })
					.value.clone(),
            _  => { panic!("prompt env var should not have that value !"); },
        };
        let mut pr: Vec<(usize, char)> = ps.char_indices().collect();
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
                    (index, 'h') => pt.push_str(&(&self.shell_vars.get("HOSTNAME").unwrap()).value),
                    (index, 'H') => pt.push_str(&(&self.shell_vars.get("HOSTNAME").unwrap()).value),
                    (index, 'j') => unimplemented!(),
                    (index, 'l') => pt.push_str(&(&self.shell_vars.get("TERM").unwrap()).value),
                    (index, 's') => pt.push_str(&(&self.shell_vars.get("0").unwrap()).value),
                    (index, 't') => { let dt = Local::now(); pt.push_str(&dt.format("%H:%M:%S").to_string()); },
                    (index, 'T') => { let dt = Local::now(); pt.push_str(&dt.format("%I:%M:%S").to_string()); },
                    (index, '@') => { let dt = Local::now(); pt.push_str(&dt.format("%I:%M:%S%P").to_string()); },
                    (index, 'u') => pt.push_str(&(&self.shell_vars.get("USERNAME").unwrap()).value),
                    (index, 'v') => pt.push_str("0.0.1"), // FIXME
                    (index, 'V') => pt.push_str("0.0.1"), // FIXME
                    (index, 'w') => pt.push_str(&(&self.shell_vars.get("PWD").unwrap()).value),
                    (index, 'W') => pt.push_str(&(&self.shell_vars.get("PWD").unwrap()).value),
                    (index, '!') => unimplemented!(),
                    (index, '#') => unimplemented!(),
                    (index, '$') => match &(&self.shell_vars.get("UID").unwrap()).value[..] {
                        "0" => pt.push_str("#"),
                        _   => pt.push_str("$"), },
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
        self.prompt = pt;
    }

    /// Initialize default aliases
    pub fn init_aliases () -> HashMap<String, String, SeaRandomState> {
        let mut aliases = HashMap::with_capacity_and_hasher(30, SeaRandomState);
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

/// TBD
trait ShellCommand {
    fn run(&self);
}

/// To be completely overhauled when parser is implemented
fn handle_command(user_expr: &str) -> bool {
    // Clean up the string by removing the newline at the end
    let expr = user_expr.trim_matches('\n');
    // Bash (kind of) compatibility. BASH_COMMAND contains command to be executed, RUSH_COMMAND in our case.
    env::set_var("RUSH_COMMAND", expr);
    let components: Vec<&str> = expr.split(' ').collect();
    //builtins(&components)
    true
}

/// To be completely overhauled when parser is implemented
//~ fn builtins(command: &Vec<&str>) -> bool {
    //~ // TODO to be replaced by some nom magic.
    //~ match command[0] {
        //~ "" => { return false; },
        //~ "[[" => { builtins::etest(&command[1..]); },
        //~ "<" => { builtins::ltsign(&command[1..]); },
        //~ "<<" => { builtins::dltsign(&command[1..]); },
        //~ ">" => { builtins::gtsign(&command[1..]); },
        //~ ">>" => { builtins::dgtsign(&command[1..]); },
        //~ "|" => { builtins::pipe(&command[1..]); },
        //~ "||" => { builtins::dpipe(&command[1..]); },
        //~ "&" => { builtins::and(&command[1..]); },
        //~ "&&" => { builtins::dand(&command[1..]); },
        //~ "`" => { builtins::backtick(&command[1..]); },
        //~ "'" => { builtins::squote(&command[1..]); },
        //~ "\"" => { builtins::dquote(&command[1..]); },
        //~ "alias" => { builtins::alias(&command[1..]); },
        //~ "autoload" => { builtins::autoload(&command[1..]); },
        //~ "bg" => { builtins::bg(&command[1..]); },
        //~ "bind" => { builtins::bind(&command[1..]); },
        //~ "break" => { builtins::bi_break(&command[1..]); },
        //~ "builtin" => { builtins::builtin(&command[1..]); },
        //~ "caller" => { builtins::caller(&command[1..]); },
        //~ "case" => { builtins::case(&command[1..]); },
        //~ "cd" => { builtins::cd(&command[1..]); },
        //~ //"chmod" => { builtins::chmod(&command[1..]); },
        //~ //"chown" => { builtins::chown(&command[1..]); },
        //~ "clear" => { builtins::clear(&command[1..]); },
        //~ "command" => { builtins::command(&command[1..]); },
        //~ "continue" => { builtins::bi_continue(&command[1..]); },
        //~ "declare" => { builtins::declare(&command[1..]); },
        //~ "dirs" => { builtins::dirs(&command[1..]); },
        //~ "disown" => { builtins::disown(&command[1..]); },
        //~ "echo" => { builtins::echo(&command[1..]); },
        //~ "enable" => { builtins::enable(&command[1..]); },
        //~ "eval" => { builtins::eval(&command[1..]); },
        //~ "exec" => { builtins::exec(&command[1..]); },
        //~ "exit" => { return true; },
        //~ "export" => {builtins::export(&command[1..]); },
        //~ "expr" => {builtins::expr(&command[1..]); },
        //~ "false" => { builtins::bi_false(&command[1..]); },
        //~ "fg" => { builtins::fg(&command[1..]); },
        //~ "for" => { builtins::bi_for(&command[1..]); },
        //~ "getopts" => { builtins::getopts(&command[1..]); },
        //~ "hash" => { builtins::hash(&command[1..]); },
        //~ "help" => { builtins::help(&command[1..]); },
        //~ "if" => { builtins::bi_if(&command[1..]); },
        //~ "jobs" => { builtins::jobs(&command[1..]); },
        //~ "kill" => { builtins::kill(&command[1..]); },
        //~ //"killall" => { builtins::killall(&command[1..]); },
        //~ "let" => { builtins::bi_let(&command[1..]); },
        //~ //"ln" => { builtins::ln(&command[1..]); },
        //~ "logout" => { builtins::logout(&command[1..]); },
        //~ //"mkdir" => { builtins::mkdir(&command[1..]); },
        //~ "printf" => { builtins::printf(&command[1..]); },
        //~ "popd" => { builtins::popd(&command[1..]); },
        //~ "pushd" => { builtins::pushd(&command[1..]); },
        //~ "pwd" => { builtins::pwd(&command[1..]); },
        //~ "read" => { builtins::read(&command[1..]); },
        //~ "readonly" => { builtins::readonly(&command[1..]); },
        //~ //"rmdir" => { builtins::rmdir(&command[1..]); },
        //~ "select" => { builtins::select(&command[1..]); },
        //~ "set" => { builtins::set(&command[1..]); },
        //~ "shopt" => { builtins::shopt(&command[1..]); },
        //~ "source" | "." => { builtins::source(&command[1..]); },
        //~ "suspend" => { builtins::suspend(&command[1..]); },
        //~ "test" | "[" => { builtins::test(&command[1..]); },
        //~ //"touch" => { builtins::touch(&command[1..]); },
        //~ "true" => { builtins::bi_true(&command[1..]); },
        //~ "times" => { builtins::times(&command[1..]); },
        //~ "type" => { builtins::bi_type(&command[1..]); },
        //~ "typeset" => { builtins::typeset(&command[1..]); },
        //~ "unset" => { builtins::unset(&command[1..]); },
        //~ "until" => { builtins::until(&command[1..]); },
        //~ "wait" => { builtins::wait(&command[1..]); },
        //~ "while" => { builtins::bi_while(&command[1..]); },
        //~ _ => {
            //~ // execute non-builtin command here
            //~ command::execute_line(&command[0], &command);
        //~ },
    //~ }
    //~ false
//~ }

/// main loop. the fun begins here !
fn main() {
    RuSh::default();
}

