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

pub fn init_options(options: &mut HashSet<Opt>) {
    // initialize default options.
    options.insert( Opt { name: "autocd", status: false } );
    options.insert( Opt { name: "cdable_vars", status: false } );
    options.insert( Opt { name: "cdspell", status: false } );
    options.insert( Opt { name: "checkhash", status: false } );
    options.insert( Opt { name: "checkjobs", status: false } );
    options.insert( Opt { name: "checkwinsize", status: true } );
    options.insert( Opt { name: "cmdhist", status: true } );
    options.insert( Opt { name: "compat31", status: false } );
    options.insert( Opt { name: "compat32", status: false } );
    options.insert( Opt { name: "compat40", status: false } );
    options.insert( Opt { name: "compat41", status: false } );
    options.insert( Opt { name: "compat42", status: false } );
    options.insert( Opt { name: "complete_fullquote", status: true } );
    options.insert( Opt { name: "direxpand", status: false } );
    options.insert( Opt { name: "dirspell", status: false } );
    options.insert( Opt { name: "dotglob", status: false } );
    options.insert( Opt { name: "execfail", status: false } );
    options.insert( Opt { name: "expand_aliases", status: true } );
    options.insert( Opt { name: "extdebug", status: false } );
    options.insert( Opt { name: "extglob", status: true } );
    options.insert( Opt { name: "extquote", status: true } );
    options.insert( Opt { name: "failglob", status: false } );
    options.insert( Opt { name: "force_fignore", status: true } );
    options.insert( Opt { name: "globstar", status: false } );
    options.insert( Opt { name: "globasciiranges", status: false } );
    options.insert( Opt { name: "gnu_errfmt", status: false } );
    options.insert( Opt { name: "histappend", status: true } );
    options.insert( Opt { name: "histreedit", status: false } );
    options.insert( Opt { name: "histverify", status: false } );
    options.insert( Opt { name: "histappend", status: false } );
    options.insert( Opt { name: "hostcomplete", status: false } );
    options.insert( Opt { name: "huponexit", status: false } );
    options.insert( Opt { name: "interactive_comments", status: true } );
    options.insert( Opt { name: "lastpipe", status: false } );
    options.insert( Opt { name: "lithist", status: false } );
    options.insert( Opt { name: "login_shell", status: false } );
    options.insert( Opt { name: "mailwarn", status: false } );
    options.insert( Opt { name: "no_empty_cmd_completion", status: false } );
    options.insert( Opt { name: "nocaseglob", status: false } );
    options.insert( Opt { name: "nocasematch", status: false } );
    options.insert( Opt { name: "nullglob", status: false } );
    options.insert( Opt { name: "progcomp", status: true } );
    options.insert( Opt { name: "promptvars", status: true } );
    options.insert( Opt { name: "restricted_shell", status: false } );
    options.insert( Opt { name: "shift_verbose", status: false } );
    options.insert( Opt { name: "sourcepath", status: true } );
    options.insert( Opt { name: "xpg_echo", status: false } );
}

pub fn init_env() {
	// see man bash (Shell variables)
	match env::current_exe() {
		Ok(ce) => env::set_var("RUSH", ce),
		Err(e) => panic!("Unable to get current_exe !"),
	}
	// TODO RUSHOPTS
    unsafe {
        let pid = getpid();
        env::set_var("RUSHPID", pid.to_string());
    }
    // TODO RUSH_ALIASES
    // TODO RUSH_ARGC
    // TODO RUSH_ARGV
    // TODO RUSH_CMDS
    env::set_var("RUSH_COMMAND", "");
    // TODO RUSH_EXECUTION_STRING
    // TODO RUSH_LINENO
    // TODO RUSH_REMATCH
    // TODO RUSH_SOURCE
    env::set_var("RUSH_SUBSHELL", "0");
    // TODO RUSH_VERSINFO -> need MACHTYPE, which needs HOSTTYPE, VENDOR, OSTYPE
    let versinfo = vec!["0", "0", "1", "1", "alpha0", "TODO"]; // FIXME -> use some global var.
    env::set_var("RUSH_VERSION", "0.0.1-alpha0"); // FIXME -> use some global var.
    // TODO COMP_CWORD
    // TODO COMP_KEY
    // TODO COMP_LINE
    // TODO COMP_POINT
    // TODO COMP_TYPE
    // TODO COMP_WORDBREAKS
    // TODO COMP_WORDS
    // TODO COPROC
    // TODO DIRSTACK
    unsafe {
        let euid = geteuid();
        env::set_var("RUSHPID", euid.to_string());
    }
    // TODO FUNCNAME
    // TODO GROUPS
    // TODO HISTCMD
    let mut bufc = vec![0u8; 40];
    extern "C" {
        fn gethostname(bufc: *mut c_char, len: size_t) -> c_int;
    }
    unsafe {
        gethostname(bufc.as_mut_ptr() as *mut c_char, 40);
    };
    env::set_var("HOSTNAME", str::from_utf8(bufc.split(|x| *x == 0).next().unwrap()).unwrap_or("wtf"));
    // TODO HOSTTYPE
    env::set_var("LINENO", "1");
    // TODO MACHTYPE
    // TODO MAPFILE
    env::set_var("OLDPWD", ".");
    // TODO OPTARG
    // TODO OPTIND
    // TODO OSTYPE
    // TODO PIPESTATUS
    unsafe {
        let ppid = getppid();
        env::set_var("PPID", ppid.to_string());
    }
    env::set_var("PWD", env::current_dir().unwrap_or(PathBuf::from("/")));
    let mut rng = rand::thread_rng();
    if rng.gen() {
		env::set_var("RANDOM", rng.gen::<i16>().to_string());
	}
	// TODO READLINE_LINE
	// TODO READLINE_POINT
	// TODO REPLY
	// TODO SECONDS
	env::set_var("SECONDS", "0");
	match env::current_exe() {
		Ok(ce) => env::set_var("SHELL", ce),
		Err(e) => panic!("Unable to get current_exe !"),
	}
	// TODO SHELLOPTS
	// TODO SHLVL
	match env::var("SHLVL") {
		Ok(lvl) => { let mut level:u32 = lvl.parse().unwrap(); level +=1; env::set_var("SHLVL", level.to_string()); },
		Err(e) => env::set_var("SHLVL", "1"),
	}
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

pub fn load_config() {
    match env::home_dir() {
        Some(ref p) => println!("{}", p.display()),
        None => println!("Impossible to get your home dir!")
    }
    // TODO read and parse ~/.rushrc
}

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
