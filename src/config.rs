/*
 * config.rs
 *
 * Copyright 2015-2016 Laurent Wandrebeck <l.wandrebeck@quelquesmots.fr>
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

//! Configurable parts of RuSh.
//!
//! This is where default options and shell variables are set when launching a RuSh instance.
//! You will find every options from autocd to xpg_echo, and shell variables based on the same scheme as bash.
//!

extern crate chrono;
extern crate libc;
extern crate linenoise;
extern crate rand;

use self::chrono::*;
use self::libc::{c_char, c_int, size_t};
use self::rand::Rng;
use std::ffi::CStr;
use std::{env,  io, str};
use std::path::PathBuf;
use std::collections::HashSet;
use libc::{geteuid, getpid, getppid, getuid, getgid, getlogin};

use Opt;

/// Options are set to their default values, following those used in bash.
pub fn init_options(options: &mut HashSet<Opt>) {
    // initialize default options.
    // If set, a command name that is the name of a directory is executed as if it were the argument to the cd command.  This option is only used by interactive shells.
    options.insert( Opt { name: "autocd", status: false } );
    // If set, an argument to the cd builtin command that is not a directory is assumed to be the name of a variable whose value is the directory to change to.
    options.insert( Opt { name: "cdable_vars", status: false } );
    // If  set, minor errors in the spelling of a directory component in a cd command will be corrected.  The errors checked for are transposed characters, a missing character, and one character too many.  If a correction is found, the corrected file name is printed, and the command proceeds.  This option is only used by interactive shells.
    options.insert( Opt { name: "cdspell", status: false } );
    // If set, rush checks that a command found in the hash table exists before trying to execute it.  If a hashed command no longer exists, a normal path search is performed.
    options.insert( Opt { name: "checkhash", status: false } );
    // If set, rush lists the status of any stopped and running jobs before exiting an interactive shell.  If any jobs are running, this causes the exit to be deferred until a second exit is attempted without an intervening command (see JOB CONTROL above).  The shell always postpones exiting if any jobs are stopped.
    options.insert( Opt { name: "checkjobs", status: false } );
    // If set, rush checks the window size after each command and, if necessary, updates the values of LINES and COLUMNS.
    options.insert( Opt { name: "checkwinsize", status: true } );
    // If set, rush attempts to save all lines of a multiple-line command in the same history entry.  This allows easy re-editing of multi-line commands.
    options.insert( Opt { name: "cmdhist", status: true } );
    // If set, rush changes its behavior to that of bash version 3.1 with respect to quoted arguments to the [[ conditional command's =~ operator. HIGHLY LIKELY NEVER TO BE TAKEN CARE OF.
    options.insert( Opt { name: "compat31", status: false } );
    // If set, rush  changes its behavior to that of bash version 3.2 with respect to locale-specific string comparison when using the [[ conditional command's < and > operators. HIGHLY LIKELY NEVER TO BE TAKEN CARE OF.
    options.insert( Opt { name: "compat32", status: false } );
    // If set, rush changes its behavior to that of bash version 4.0 with respect to locale-specific string comparison when using the [[ conditional command's <  and  > operators (see previous item) and the effect of interrupting a command list. HIGHLY LIKELY NEVER TO BE TAKEN CARE OF.
    options.insert( Opt { name: "compat40", status: false } );
    // If set, rush, when in posix mode, treats a single quote in a double-quoted parameter expansion as a special character. The single quotes must match (an even number) and the characters between the single quotes are considered quoted.  This is the behavior of posix mode through version 4.1. The default bash behavior remains as in previous versions. HIGHLY LIKELY NEVER TO BE TAKEN CARE OF.
    options.insert( Opt { name: "compat41", status: false } );
    // If set, rush does not process the replacement string in the pattern substitution word expansion using quote removal. HIGHLY LIKELY NEVER TO BE TAKEN CARE OF.
    options.insert( Opt { name: "compat42", status: false } );
    // If set, rush quotes all shell metacharacters in filenames and directory names when performing completion. If not set, bash removes metacharacters such as the dollar sign from the set of characters that will be quoted in completed filenames when these metacharacters appear in shell variable references in words to be completed. This means that dollar signs in variable names that expand to directories will not be quoted; however, any dollar signs appearing in filenames will not be quoted, either. This is active only when bash is using backslashes to quote completed filenames. This variable is set by default, which is the default bash behavior in versions through 4.2.
    options.insert( Opt { name: "complete_fullquote", status: true } );
    // If  set, rush replaces directory names with the results of word expansion when performing filename completion. This changes the contents of the readline editing buffer. If not set, rush attempts to preserve what the user typed.
    options.insert( Opt { name: "direxpand", status: false } );
    // If set, rush attempts spelling correction on directory names during word completion if the directory name initially supplied does not exist.
    options.insert( Opt { name: "dirspell", status: false } );
    // If set, rush includes filenames beginning with a `.' in the results of pathname expansion.
    options.insert( Opt { name: "dotglob", status: false } );
    // If set, a non-interactive shell will not exit if it cannot execute the file specified as an argument to the exec builtin command. An interactive shell does not exit if exec fails.
    options.insert( Opt { name: "execfail", status: false } );
    // If set, aliases are expanded as described above under ALIASES.  This option is enabled by default for interactive shells.
    options.insert( Opt { name: "expand_aliases", status: true } );
    // If set, behavior intended for use by debuggers is enabled:
    // 1.     The -F option to the declare builtin displays the source file name and line number corresponding to each function name supplied as an argument.
    // 2.     If the command run by the DEBUG trap returns a non-zero value, the next command is skipped and not executed.
    // 3.     If the command run by the DEBUG trap returns a value of 2, and the shell is executing in a subroutine (a shell function or a shell script executed by the . or source builtins), a call to return is simulated.
    // 4.     BASH_ARGC and BASH_ARGV are updated as described in their descriptions above.
    // 5.     Function tracing is enabled:  command substitution, shell functions, and subshells invoked with ( command ) inherit the DEBUG and RETURN traps.
    // 6.     Error tracing is enabled:  command substitution, shell functions, and subshells invoked with ( command ) inherit the ERR trap.
    options.insert( Opt { name: "extdebug", status: false } );
    // If set, the extended pattern matching features described above under Pathname Expansion are enabled.
    options.insert( Opt { name: "extglob", status: true } );
    // If set, $'string' and $"string" quoting is performed within ${parameter} expansions enclosed in double quotes.  This option is enabled by default.
    options.insert( Opt { name: "extquote", status: true } );
    // If set, patterns which fail to match filenames during pathname expansion result in an expansion error.
    options.insert( Opt { name: "failglob", status: false } );
    // If  set,  the  suffixes  specified by the FIGNORE shell variable cause words to be ignored when performing word completion even if the ignored words are the only possible completions. See SHELL VARIABLES above for a description of FIGNORE.  This option is enabled by default.
    options.insert( Opt { name: "force_fignore", status: true } );
    // If set, range expressions used in pattern matching bracket expressions (see Pattern Matching above) behave as if in the traditional C locale when performing comparisons.  That is, the current locale's collating sequence is not taken into account, so b will not collate between A and B, and upper-case and lower-case ASCII characters will collate together. FIXME: Rust does use UTF-8.
    options.insert( Opt { name: "globasciiranges", status: false } );
    // If  set, the pattern ** used in a pathname expansion context will match all files and zero or more directories and subdirectories.  If the pattern is followed by a /, only directories and subdirectories match.
    options.insert( Opt { name: "globstar", status: false } );
    // If set, shell error messages are written in the standard GNU error message format.
    options.insert( Opt { name: "gnu_errfmt", status: false } );
    // If set, the history list is appended to the file named by the value of the HISTFILE variable when the shell exits, rather than overwriting the file.
    options.insert( Opt { name: "histappend", status: true } );
    // If set, and readline is being used, a user is given the opportunity to re-edit a failed history substitution.
    options.insert( Opt { name: "histreedit", status: false } );
    // If set, and readline is being used, the results of history substitution are not immediately passed to the shell parser. Instead, the resulting line is loaded into the readline editing buffer, allowing further modification.
    options.insert( Opt { name: "histverify", status: false } );
    // If set, and readline is being used, rush will attempt to perform hostname completion when a word containing a @ is being completed. This is enabled by default.
    options.insert( Opt { name: "hostcomplete", status: true } );
    // If set, rush will send SIGHUP to all jobs when an interactive login shell exits.
    options.insert( Opt { name: "huponexit", status: false } );
    // If set, allow a word beginning with # to cause that word and all remaining characters on that line to be ignored in an interactive shell. This option is enabled by default.
    options.insert( Opt { name: "interactive_comments", status: true } );
    // If set, and job control is not active, the shell runs the last command of a pipeline not executed in the background in the current shell environment.
    options.insert( Opt { name: "lastpipe", status: false } );
    // If set, and the cmdhist option is enabled, multi-line commands are saved to the history with embedded newlines rather than using semicolon separators where possible.
    options.insert( Opt { name: "lithist", status: false } );
    // The shell sets this option if it is started as a login shell (see INVOCATION above). The value may not be changed.
    options.insert( Opt { name: "login_shell", status: false } );
    // If set, and a file that rush is checking for mail has been accessed since the last time it was checked, the message ``The mail in mailfile has been read'' is displayed.
    options.insert( Opt { name: "mailwarn", status: false } );
    // If set, and readline is being used, rush will not attempt to search the PATH for possible completions when completion is attempted on an empty line.
    options.insert( Opt { name: "no_empty_cmd_completion", status: false } );
    // If set, rush matches filenames in a case-insensitive fashion when performing pathname expansion.
    options.insert( Opt { name: "nocaseglob", status: false } );
    // If set, rush matches patterns in a case-insensitive fashion when performing matching while executing case or [[ conditional commands.
    options.insert( Opt { name: "nocasematch", status: false } );
    // If set, rush allows patterns which match no files to expand to a null string, rather than themselves.
    options.insert( Opt { name: "nullglob", status: false } );
    // If set, the programmable completion facilities are enabled.  This option is enabled by default.
    options.insert( Opt { name: "progcomp", status: true } );
    // If set, prompt strings undergo parameter expansion, command substitution, arithmetic expansion, and quote removal after being expanded. This option is enabled by default.
    options.insert( Opt { name: "promptvars", status: true } );
    // The shell sets this option if it is started in restricted mode. The value may not be changed. This is not reset when the startup files are executed, allowing the startup files to discover whether or not a shell is restricted.
    options.insert( Opt { name: "restricted_shell", status: false } );
    // If set, the shift builtin prints an error message when the shift count exceeds the number of positional parameters.
    options.insert( Opt { name: "shift_verbose", status: false } );
    // If set, the source (.) builtin uses the value of PATH to find the directory containing the file supplied as an argument. This option is enabled by default.
    options.insert( Opt { name: "sourcepath", status: true } );
    // If set, the echo builtin expands backslash-escape sequences by default.
    options.insert( Opt { name: "xpg_echo", status: false } );
}

/// Shell variables are set here, following the bash way.
pub fn init_env() {
    // see man bash (Shell variables)
    // Expands to the full filename used to invoke this instance of rush.
    match env::current_exe() {
        Ok(ce) => env::set_var("RUSH", ce),
        Err(e) => panic!("Unable to get current_exe !"),
    }
    // A colon-separated list of enabled shell options. Each word in the list is a valid argument for the -s option to the shopt builtin command. The options appearing in RUSHOPTS are those reported as on by shopt. If this variable is in the environment when rush starts up, each shell option in the list will be enabled before reading any startup files. This variable is read-only.
    // TODO RUSHOPTS
    // Expands to the process ID of the current rush process. This differs from $$ under certain circumstances, such as subshells that do not require rush to be re-initialized.
    unsafe {
        let pid = getpid();
        env::set_var("RUSHPID", pid.to_string());
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
    env::set_var("RUSH_COMMAND", "");
    // The command argument to the -c invocation option.
    // TODO RUSH_EXECUTION_STRING
    // An array variable whose members are the line numbers in source files where each corresponding member of FUNCNAME was invoked.  ${RUSH_LINENO[$i]} is the line number in the source file (${RUSH_SOURCE[$i+1]}) where ${FUNCNAME[$i]} was called (or ${RUSH_LINENO[$i-1]} if referenced within another shell function). Use LINENO to obtain the current line number.
    // TODO RUSH_LINENO
    // An array variable whose members are assigned by the =~ binary operator to the [[ conditional command. The element with index 0 is the portion of the string matching the entire regular expression.  The element with index n is the portion of the string matching the nth parenthesized subexpression. This variable is read-only.
    // TODO RUSH_REMATCH
    // An array variable whose members are the source filenames where the corresponding shell function names in the FUNCNAME array variable are defined. The shell function ${FUNCNAME[$i]} is defined in the file ${RUSH_SOURCE[$i]} and called from ${RUSH_SOURCE[$i+1]}.
    // TODO RUSH_SOURCE
    // Incremented by one within each subshell or subshell environment when the shell begins executing in that environment. The initial value is 0.
    env::set_var("RUSH_SUBSHELL", "0");
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
    env::set_var("RUSH_VERSION", "0.0.1-alpha0"); // FIXME -> use some global var.
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
        env::set_var("EUID", euid.to_string());
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
    env::set_var("HOSTNAME", str::from_utf8(bufc.split(|x| *x == 0).next().unwrap()).unwrap_or("wtf"));
    // Automatically set to a string that uniquely describes the type of machine on which rush is executing.  The default is system-dependent.
    // TODO HOSTTYPE
    // Each time this parameter is referenced, the shell substitutes a decimal number representing the current sequential line number (starting with 1) within a script or function. When not in a script or function, the value substituted is not guaranteed to be meaningful. If LINENO is unset, it loses its special properties, even if it is subsequently reset.
    env::set_var("LINENO", "1");
    // Automatically set to a string that fully describes the system type on which rush is executing, in the standard GNU cpu-company-system format. The default is system-dependent.
    // TODO MACHTYPE
    // An array variable created to hold the text read by the mapfile builtin when no variable name is supplied.
    // TODO MAPFILE
    // The previous working directory as set by the cd command.
    env::set_var("OLDPWD", ".");
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
        env::set_var("PPID", ppid.to_string());
    }
    // The current working directory as set by the cd command.
    env::set_var("PWD", env::current_dir().unwrap_or(PathBuf::from("/")));
    // Each time this parameter is referenced, a random integer between 0 and 32767 is generated. The sequence of random numbers may be initialized by assigning a value to RANDOM. If RANDOM is unset, it loses its special properties, even if it is subsequently reset.
    let mut rng = rand::thread_rng();
    if rng.gen() {
        env::set_var("RANDOM", rng.gen::<i16>().to_string());
    }
    // The contents of the readline line buffer, for use with "bind -x".
    // TODO READLINE_LINE
    // The position of the insertion point in the readline line buffer, for use with "bind -x".
    // TODO READLINE_POINT
    // Set to the line of input read by the read builtin command when no arguments are supplied.
    // TODO REPLY
    // Each time this parameter is referenced, the number of seconds since shell invocation is returned.  If a value is assigned to SECONDS, the value returned upon subsequent references is the number of seconds since the assignment plus the value assigned. If SECONDS is unset, it loses its special properties, even if it is subsequently reset.
    env::set_var("SECONDS", "0");
    // The full pathname to the shell is kept in this environment variable.  If it is not set when the shell starts, rush assigns to it the full pathname of the current user's login shell.
    match env::current_exe() {
        Ok(ce) => env::set_var("SHELL", ce),
        Err(e) => panic!("Unable to get current_exe !"),
    }
    // A colon-separated list of enabled shell options. Each word in the list is a valid argument for the -o option to the set builtin command. The options appearing in SHELLOPTS are those reported as  on by set -o. If this variable is in the environment when rush starts up, each shell option in the list will be enabled before reading any startup files. This variable is read-only.
    // TODO SHELLOPTS
    // Incremented by one each time an instance of rush is started.
    match env::var("SHLVL") {
        Ok(lvl) => { let mut level:u32 = lvl.parse().unwrap(); level +=1; env::set_var("SHLVL", level.to_string()); },
        Err(e) => env::set_var("SHLVL", "1"),
    }
    // Expands to the user ID of the current user, initialized at shell startup. This variable is readonly.
    unsafe {
        let id = getuid();
        env::set_var("UID", id.to_string());
    }
    // TODO variables used by the shell, see man bash.
    unsafe {
        let id = getgid();
        env::set_var("GID", id.to_string());
    }
    unsafe {
        let log = getlogin();
        env::set_var("USERNAME",
            String::from_utf8(CStr::from_ptr(log).to_bytes().to_owned()).unwrap_or("no login".to_owned()));
    }
    env::set_var("HISTSIZE", "1000");
    linenoise::history_set_max_len(1000);
}

/// Applies ~/.rushrc.
pub fn load_config() {
    match env::home_dir() {
        Some(ref p) => println!("{}", p.display()),
        None => println!("Impossible to get your home dir!")
    }
    // TODO read and parse ~/.rushrc
}

/// Prompt management.
pub fn prompt(p: &str) -> String {
    let mut aslash = false;
    let mut pt = String::new();
    let ps: String = match p {
        "PS1" => { match env::var(p) {
            Ok(ps1) => { ps1 }
            Err(e) => { let ps1 = String::from("[\\u@\\h \\W]\\$ "); env::set_var("PS1", ps1); String::from("[\\u@\\h \\W]\\$ ") }
            } },
        "PS2" => { match env::var(p) {
            Ok(ps2) => { ps2 }
            Err(e) => { let ps2 = String::from(">"); env::set_var("PS2", ps2); String::from(">") }
            } },
        "PS3" => { match env::var(p) {
            Ok(ps3) => { ps3 }
            Err(e) => { let ps3 = String::from(">"); env::set_var("PS3", ps3); String::from(">") }
            } },
        "PS4" => { match env::var(p) {
            Ok(ps4) => { ps4 }
            Err(e) => { let ps4 = String::from(">"); env::set_var("PS4", ps4); String::from(">") }
            } },
        _     => { panic!("prompt env var should not have that value !"); },
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
                (index, 'h') => pt.push_str(&env::var("HOSTNAME").unwrap_or("localhost".to_owned()).split('.').next().unwrap()),
                (index, 'H') => pt.push_str(&env::var("HOSTNAME").unwrap_or("localhost.localdomain".to_owned())),
                (index, 'j') => unimplemented!(),
                (index, 'l') => pt.push_str(&env::var("TERM").unwrap_or("unknown term".to_owned())),
                (index, 's') => pt.push_str(&env::var("0").unwrap_or("rush".to_owned()).split('/').last().unwrap()),
                (index, 't') => { let dt = Local::now(); pt.push_str(&dt.format("%H:%M:%S").to_string()); },
                (index, 'T') => { let dt = Local::now(); pt.push_str(&dt.format("%I:%M:%S").to_string()); },
                (index, '@') => { let dt = Local::now(); pt.push_str(&dt.format("%I:%M:%S%P").to_string()); },
                (index, 'u') => pt.push_str(&env::var("USERNAME").unwrap_or("unknown user".to_owned())),
                (index, 'v') => pt.push_str("0.0.1"), // FIXME
                (index, 'V') => pt.push_str("0.0.1"), // FIXME
                (index, 'w') => pt.push_str(&env::var("PWD").unwrap()),
                (index, 'W') => pt.push_str(&env::var("PWD").unwrap().split('/').last().unwrap()),
                (index, '!') => unimplemented!(),
                (index, '#') => unimplemented!(),
                (index, '$') => match &env::var("UID").unwrap()[..] {
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
    return pt;
}
