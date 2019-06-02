//
// variables.rs
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

//! RuSh variables management is located in this module.
//!
//! variables.rs contains variables structures and affiliated methods.
//! `Variable` and `Variables` are defined here.
//! variables (un)setting, update methods for classical variables.

use libc::{c_char, c_int, geteuid, getgid, getlogin, getpid, getppid, getuid, size_t};
use rand::Rng;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::ffi::CStr;
use std::{env, str};

/// Access can be ReadWrite or ReadOnly
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Access {
    ReadWrite,
    ReadOnly,
}

/// Value contains variable value, be it a i64, f64 or String, defined as an enum.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    I(i64),
    F(f64),
    S(String),
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
/// Variable Structure.
pub struct Variable {
    /// Variable value is stored as Value enum.
    pub value: Value,
    /// Is the variable rw or ro.
    pub access: Access,
}

/// Methods for Variable structure.
impl Variable {
    /// Extract `i64` from Variable.value
    ///
    /// # Examples
    /// ```rust
    /// use rush::variables::{Access, Variable, Value};
    /// let var = Variable { value: Value::I(-42), access: Access::ReadWrite };
    /// assert_eq!(var.geti(), -42);
    /// ```
    pub fn geti(&self) -> i64 {
        match self.value {
            Value::I(i) => i,
            _ => panic!("Trying to retrieve wrong kind of Value (geti)"),
        }
    }

    /// Extract `f64` from Variable.value
    ///
    /// # Examples
    /// ```rust
    /// use rush::variables::{Access, Variable, Value};
    /// let var = Variable { value: Value::F(-42.5), access: Access::ReadWrite };
    /// assert_eq!(var.getf(), -42.5);
    /// ```
    pub fn getf(&self) -> f64 {
        match self.value {
            Value::F(f) => f,
            _ => panic!("Trying to retrieve wrong kind of Value (getf)"),
        }
    }

    /// Extract `String` from Variable.value
    ///
    /// # Examples
    /// ```rust
    /// use rush::variables::{Access, Variable, Value};
    /// let var = Variable { value: Value::S("Forty two".to_string()), access: Access::ReadWrite };
    /// assert_eq!(var.gets(), "Forty two");
    /// ```
    pub fn gets(&self) -> String {
        match self.value {
            Value::S(ref s) => s.to_string(),
            _ => panic!("Trying to retrieve wrong kind of Value (gets)"),
        }
    }
}

/// Public structure for `Variables` management.
pub struct Variables {
    /// variables are stored in a HashMap<String, `Variable`>. First String being the variable name (key), the second the value and rw state.
    vars: HashMap<String, Variable>,
}

/// Methods for `Variables`.
impl Variables {
    /// Get `Variable` from its name. Returns `Variable` as `Option`.
    ///
    /// # Examples
    /// ```rust
    /// use rush::variables::{Access, Variable, Variables, Value};
    ///
    /// let mut vars = Variables::init_shell_vars();
    /// match vars.get("RUSH_COMMAND") {
    ///     Some(v) => assert_eq!(v.gets(), ""),
    ///     None => panic!("RUSH_COMMAND should be defined.")
    /// }
    /// match vars.get("HISTSIZE") {
    ///     Some(v) => assert_eq!(v.geti(), 1000),
    ///     None => panic!("HISTSIZE should be defined.")
    /// }
    /// vars.set(String::from("TEST"), Variable { value: Value::F(-49.3), access: Access::ReadWrite });
    /// match vars.get("TEST") {
    ///     Some(v) => assert_eq!(v.getf(), -49.3),
    ///     None => panic!("TEST variable should be defined.")
    /// }
    /// ```
    pub fn get(&self, key: &str) -> Option<Variable> {
        match self.vars.get(key) {
            Some(val) => {
                let var = Variable {
                    value: val.value.clone(),
                    access: val.access.clone(),
                };
                Some(var)
            }
            None => None,
        }
    }

    /// Get Access from Variable
    ///
    /// # Examples
    /// ```rust
    /// use rush::variables::{Access, Variable, Variables, Value};
    ///
    /// let mut vars = Variables::init_shell_vars();
    /// match vars.get_access("RUSH_COMMAND") {
    ///     Some(v) => assert_eq!(v, Access::ReadWrite),
    ///     None => panic!("RUSH_COMMAND should be defined and Access::ReadWrite.")
    /// }
    /// match vars.get("EUID") {
    ///     Some(v) => assert_eq!(v.access, Access::ReadOnly),
    ///     None => panic!("EUID should be defined and Access::ReadOnly.")
    /// }
    /// match vars.get("nonexistingvar") {
    ///     Some(v) => panic!("nonexistantvar should not give back {:?}", v),
    ///     None => assert!(true),
    /// }
    /// ```
    pub fn get_access(&self, key: &str) -> Option<Access> {
        match self.vars.get(key) {
            Some(val) => Some(val.access.clone()),
            None => None,
        }
    }

    /// Set a variable value for a given name. Variable is created if needed, otherwise value is updated if rw.
    ///
    /// # Examples
    /// ```rust
    /// use rush::variables::{Access, Variable, Variables, Value};
    ///
    /// let mut vars = Variables::init_shell_vars();
    /// vars.set(String::from("TESTF"), Variable { value: Value::F(-49.3), access: Access::ReadWrite });
    /// match vars.get("TESTF") {
    ///     Some(v) => assert_eq!(v.getf(), -49.3),
    ///     None => panic!("TESTF should be defined.")
    /// }
    /// vars.set(String::from("TESTI"), Variable { value: Value::I(-42), access: Access::ReadWrite });
    /// match vars.get("TESTI") {
    ///     Some(v) => assert_eq!(v.geti(), -42),
    ///     None => panic!("TESTI should be defined.")
    /// }
    /// vars.set(String::from("TESTS"), Variable { value: Value::S(String::from("RuSh will rock (one day)")), access: Access::ReadWrite });
    /// match vars.get("TESTS") {
    ///     Some(v) => assert_eq!(v.gets(), "RuSh will rock (one day)"),
    ///     None => panic!("TESTS variable should be defined.")
    /// }
    /// ```
    pub fn set(&mut self, key: String, v: Variable) {
        // does the var already exist ?
        match self.vars.entry(key) {
            Occupied(mut entry) => {
                let contents = entry.get_mut();
                match contents.access {
                    Access::ReadWrite => *contents = v,
                    Access::ReadOnly => println!("readonly variable"),
                }
            }
            Vacant(entry) => {
                entry.insert(v);
            }
        }
    }
    /// Set variable access status. If variable does not exist, it is created as Value::S("")
    ///
    /// # Examples
    /// ```rust
    /// use rush::variables::{Access, Variable, Variables, Value};
    ///
    /// let mut vars = Variables::init_shell_vars();
    /// vars.set_access("TEST".to_string(), Access::ReadWrite);
    /// match vars.get_access("TEST") {
    ///     Some(v) => assert_eq!(v, Access::ReadWrite),
    ///     None => panic!("TEST should be defined."),
    /// }
    /// vars.set_access("TEST".to_string(), Access::ReadOnly);
    /// match vars.get_access("TEST") {
    ///     Some(v) => assert_eq!(v, Access::ReadOnly),
    ///     None => panic!("TEST should be defined."),
    /// }
    /// match vars.get_access("doesnotexist") {
    ///     Some(v) => panic!("doesnotexist variable should not be defined"),
    ///     None => assert!(true),
    /// }
    /// match vars.get_access("doesnotexist2") {
    ///     Some(v) => panic!("doesnotexist2 variable should not be defined"),
    ///     None => assert!(true),
    /// }
    pub fn set_access(&mut self, key: String, v: Access) {
        // does the var already exist ?
        match self.vars.entry(key) {
            Occupied(mut entry) => {
                let contents = entry.get_mut();
                contents.access = v;
            }
            Vacant(entry) => {
                entry.insert(Variable {
                    value: Value::S("".to_string()),
                    access: v,
                });
            }
        }
    }

    /// Unset a variable name and its value. So is the associated environment variable and value.
    ///
    /// # Examples
    /// ```rust
    /// use rush::variables::{Variable, Variables, Value};
    ///
    /// let mut vars = Variables::init_shell_vars();
    /// match vars.get("RUSH_COMMAND") {
    ///     Some(v) => assert_eq!(v.gets(), ""),
    ///     None => panic!("RUSH_COMMAND should be defined.")
    /// }
    /// vars.unset(String::from("RUSH_COMMAND"));
    /// match vars.get("RUSH_COMMAND") {
    ///     Some(v) => panic!("RUSH_COMMAND should have been unset."),
    ///     None => println!("RUSH_COMMAND is not set.")
    /// }
    /// ```
    pub fn unset(&mut self, key: String) {
        self.vars.remove(&key);
        env::remove_var(key);
    }

    /// Default shell variables are set here, following the bash way.
    ///
    /// # Examples
    /// ```rust
    /// use rush::variables::{Access, Variable, Variables, Value};
    ///
    /// let mut vars = Variables::init_shell_vars();
    /// match vars.get("RUSH") {
    ///     Some(val) => println!("RUSH var value is: {}", val.gets()),
    ///     None => println!("RUSH variable does not exist.")
    /// }
    /// ```
    pub fn init_shell_vars() -> Variables {
        let mut vars = Variables {
            vars: HashMap::with_capacity(200),
        };
        // see man bash (Shell vars)
        // Expands to the full filename used to invoke this instance of rush.
        match env::current_exe() {
            Ok(ce) => vars.set(
                String::from("RUSH"),
                Variable {
                    value: Value::S(ce.into_os_string().into_string().unwrap()),
                    access: Access::ReadWrite,
                },
            ),
            Err(e) => panic!("Unable to get current_exe ! {}", e),
        }
        // A colon-separated list of enabled shell options. Each word in the list is a valid argument for the -s option to the shopt builtin command. The options appearing in RUSHOPTS are those reported as on by shopt. If this variable is in the environment when rush starts up, each shell option in the list will be enabled before reading any startup files. This variable is read-only.
        // TODO RUSHOPTS
        // Expands to the process ID of the current rush process. This differs from $$ under certain circumstances, such as subshells that do not require rush to be re-initialized.
        unsafe {
            let pid = getpid();
            vars.set(
                String::from("RUSHPID"),
                Variable {
                    value: Value::I(i64::from(pid)),
                    access: Access::ReadWrite,
                },
            );
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
        vars.set(
            String::from("RUSH_COMMAND"),
            Variable {
                value: Value::S(String::from("")),
                access: Access::ReadWrite,
            },
        );
        // The command argument to the -c invocation option.
        // TODO RUSH_EXECUTION_STRING
        // An array variable whose members are the line numbers in source files where each corresponding member of FUNCNAME was invoked.  ${RUSH_LINENO[$i]} is the line number in the source file (${RUSH_SOURCE[$i+1]}) where ${FUNCNAME[$i]} was called (or ${RUSH_LINENO[$i-1]} if referenced within another shell function). Use LINENO to obtain the current line number.
        // TODO RUSH_LINENO
        // An array variable whose members are assigned by the =~ binary operator to the [[ conditional command. The element with index 0 is the portion of the string matching the entire regular expression.  The element with index n is the portion of the string matching the nth parenthesized subexpression. This variable is read-only.
        // TODO RUSH_REMATCH
        // An array variable whose members are the source filenames where the corresponding shell function names in the FUNCNAME array variable are defined. The shell function ${FUNCNAME[$i]} is defined in the file ${RUSH_SOURCE[$i]} and called from ${RUSH_SOURCE[$i+1]}.
        // TODO RUSH_SOURCE
        // Incremented by one within each subshell or subshell environment when the shell begins executing in that environment. The initial value is 0.
        vars.set(
            String::from("RUSH_SUBSHELL"),
            Variable {
                value: Value::I(0),
                access: Access::ReadWrite,
            },
        );
        // A readonly array variable whose members hold version information for this instance of rush.  The values assigned to the array members are as follows:
        // RUSH_VERSINFO[0]        The major version number (the release).
        // RUSH_VERSINFO[1]        The minor version number (the version).
        // RUSH_VERSINFO[2]        The patch level.
        // RUSH_VERSINFO[3]        The build version.
        // RUSH_VERSINFO[4]        The release status (e.g., beta1).
        // RUSH_VERSINFO[5]        The value of MACHTYPE.
        // TODO RUSH_VERSINFO -> need MACHTYPE, which needs HOSTTYPE, VENDOR, OSTYPE
        // Expands to a string describing the version of this instance of bash
        let _versinfo = vec!["0", "0", "0", "0", "alpha0", "x86_64-redhat-linux-gnu"]; // FIXME -> needs internal array support which is not yet implemented
        vars.set(
            String::from("RUSH_VERSION"),
            Variable {
                value: Value::S(String::from("0.0.0.0-alpha0-x86_64-redhat-linux-gnu")),
                access: Access::ReadOnly,
            },
        ); // FIXME -> use some global var.
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
            vars.set(
                String::from("EUID"),
                Variable {
                    value: Value::I(i64::from(euid)),
                    access: Access::ReadOnly,
                },
            );
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
        vars.set(
            String::from("HOSTNAME"),
            Variable {
                value: Value::S(
                    String::from_utf8(bufc.split(|x| *x == 0).next().unwrap().to_vec())
                        .unwrap_or_else(|_| String::from("wtf")),
                ),
                access: Access::ReadWrite,
            },
        );
        // Automatically set to a string that uniquely describes the type of machine on which rush is executing.  The default is system-dependent.
        // TODO HOSTTYPE
        // Each time this parameter is referenced, the shell substitutes a decimal number representing the current sequential line number (starting with 1) within a script or function. When not in a script or function, the value substituted is not guaranteed to be meaningful. If LINENO is unset, it loses its special properties, even if it is subsequently reset.
        vars.set(
            String::from("LINENO"),
            Variable {
                value: Value::I(1),
                access: Access::ReadWrite,
            },
        );
        // Automatically set to a string that fully describes the system type on which rush is executing, in the standard GNU cpu-company-system format. The default is system-dependent.
        // TODO MACHTYPE
        // An array variable created to hold the text read by the mapfile builtin when no variable name is supplied.
        // TODO MAPFILE
        // The previous working directory as set by the cd command.
        vars.set(
            String::from("OLDPWD"),
            Variable {
                value: Value::S(String::from(".")),
                access: Access::ReadWrite,
            },
        );
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
            vars.set(
                String::from("PPID"),
                Variable {
                    value: Value::I(i64::from(ppid)),
                    access: Access::ReadOnly,
                },
            );
        }
        // The current working directory as set by the cd command.
        let pwd = match env::current_dir() {
            Ok(path) => String::from(path.to_str().unwrap_or("/")),
            Err(_e) => String::from("/"),
        };
        vars.set(
            String::from("PWD"),
            Variable {
                value: Value::S(pwd),
                access: Access::ReadWrite,
            },
        );
        // Each time this parameter is referenced, a random integer between 0 and 32767 is generated. The sequence of random numbers may be initialized by assigning a value to RANDOM. If RANDOM is unset, it loses its special properties, even if it is subsequently reset.
        let mut rng = rand::thread_rng();
        if rng.gen() {
            vars.set(
                String::from("RANDOM"),
                Variable {
                    value: Value::I(i64::from(rng.gen::<i16>())),
                    access: Access::ReadWrite,
                },
            );
        }
        // The contents of the readline line buffer, for use with "bind -x".
        // TODO READLINE_LINE
        // The position of the insertion point in the readline line buffer, for use with "bind -x".
        // TODO READLINE_POINT
        // Set to the line of input read by the read builtin command when no arguments are supplied.
        // TODO REPLY
        // Each time this parameter is referenced, the number of seconds since shell invocation is returned.  If a value is assigned to SECONDS, the value returned upon subsequent references is the number of seconds since the assignment plus the value assigned. If SECONDS is unset, it loses its special properties, even if it is subsequently reset.
        vars.set(
            String::from("SECONDS"),
            Variable {
                value: Value::I(0),
                access: Access::ReadWrite,
            },
        );
        // The full pathname to the shell is kept in this environment variable.  If it is not set when the shell starts, rush assigns to it the full pathname of the current user's login shell.
        let cexe = match env::current_exe() {
            Ok(ce) => String::from(ce.to_str().unwrap_or("/")),
            // FIXME - should be current user’s login shell.
            Err(_e) => String::from("/"),
        };
        vars.set(
            String::from("SHELL"),
            Variable {
                value: Value::S(cexe),
                access: Access::ReadWrite,
            },
        );
        // A colon-separated list of enabled shell options. Each word in the list is a valid argument for the -o option to the set builtin command. The options appearing in SHELLOPTS are those reported as  on by set -o. If this variable is in the environment when rush starts up, each shell option in the list will be enabled before reading any startup files. This variable is read-only.
        // TODO SHELLOPTS
        // Incremented by one each time an instance of rush is started.
        match vars.get(&String::from("SHLVL")) {
            Some(lvl) => {
                if let Variable {
                    value: Value::I(mut val),
                    access: Access::ReadWrite,
                } = lvl
                {
                    val += 1;
                    vars.set(
                        String::from("SHLVL"),
                        Variable {
                            value: Value::I(val),
                            access: Access::ReadWrite,
                        },
                    );
                }
            }
            None => vars.set(
                String::from("SHLVL"),
                Variable {
                    value: Value::I(1),
                    access: Access::ReadWrite,
                },
            ),
        };
        // Expands to the user ID of the current user, initialized at shell startup. This variable is readonly.
        unsafe {
            let id = getuid();
            vars.set(
                String::from("UID"),
                Variable {
                    value: Value::I(i64::from(id)),
                    access: Access::ReadOnly,
                },
            );
        }
        // TODO vars used by the shell, see man bash.
        unsafe {
            let id = getgid();
            vars.set(
                String::from("GID"),
                Variable {
                    value: Value::I(i64::from(id)),
                    access: Access::ReadOnly,
                },
            );
        }
        unsafe {
            let log = getlogin();
            vars.set(
                String::from("USERNAME"),
                Variable {
                    value: Value::S(
                        String::from_utf8(CStr::from_ptr(log).to_bytes().to_owned())
                            .unwrap_or_else(|_| "no login".to_owned()),
                    ),
                    access: Access::ReadOnly,
                },
            );
        }
        vars.set(
            String::from("HISTSIZE"),
            Variable {
                value: Value::I(1000),
                access: Access::ReadWrite,
            },
        );
        vars
    }
}
