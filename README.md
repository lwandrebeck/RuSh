[![Build Status](https://travis-ci.org/lwandrebeck/RuSh.svg?branch=master)](https://travis-ci.org/lwandrebeck/RuSh)
[![Coverage Status](https://coveralls.io/repos/github/lwandrebeck/RuSh/badge.svg?branch=master)](https://coveralls.io/github/lwandrebeck/RuSh?branch=master)
[![codecov](https://codecov.io/gh/lwandrebeck/RuSh/branch/master/graph/badge.svg)](https://codecov.io/gh/lwandrebeck/RuSh)
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.37+-blue.svg)](#rust-version-requirements)
[![dependency status](https://deps.rs/repo/github/lwandrebeck/RuSh/status.svg)](https://deps.rs/repo/github/lwandrebeck/RuSh)

# RuSh: A shell written in Rust

## Quick introduction

RuSh aims to be (maybe one day) a POSIX Shell, Bash compatible (or close to) with candies. Source code is [GPL3](http://www.gnu.org/licenses/gpl-3.0.html).
Please note that this is a personal project (read not funded), in order to learn Rust language (that does not mean feedback or patches are not welcome (Thanks [Guillaume](https://github.com/GuillaumeGomez) and [Mathieu](https://github.com/kali) :)).

Right now, RuSh is definitely not useable. A couple things have been done, but 99% (at _least_) have to be written.
Anyway, work has been ongoing under the hood, and RuSh will soon be able to manage echo command and variables.
Parsing loop still have to be rewritten, then 0.0.1 or 0.1.0 or something will be released (yay !). Things should evolve a (little) bit faster after that.

## TODO:
### parser
- [x] unquoted string suppport. (space character taken care of) (that one may still be a bit buggy)
- [x] "quoted" string support. (space character taken care of)
- [x] 'quoted' string support. (space character taken care of)
- [x] \`backticked\` string support. (space character taken care of)
- [x] binary numbers support (uint only for now ?). 0b mark. (i64 by default)
- [x] octal numbers support (uint only for now ?). 0 mark. (i64 by default)
- [x] hexadecimal numbers support (uint only for now ?). 0x mark. (i64 by default)
- [x] (signed) int support (i64 by default).
- [x] (signed) float support (f64 by default). exponent correctly parsed.
- [x] variable assignment. (type is autodetected)
- [x] array assignment. (type is autodetected)
- [x] shebang (#!) and comments are correctly parsed.
- [x] : syntax
- [ ] compgen builtin.
- [ ] complete builtin.
- [ ] compopt builtin.
- [ ] fc builtin.
- [ ] history builtin.
- [x] local builtin.
- [ ] mapfile builtin.
- [ ] readarray builtin.
- [ ] return builtin.
- [ ] shift builtin.
- [ ] test builtin.
- [ ] trap builtin.
- [ ] ulimit builtin.
- [ ] umask builtin.
- [ ] unalias builtin.
- [x] use [pest](https://github.com/pest-parser/pest) to parse prompt
- [x] arrays support (single dimension, can be both associative and indexed, and store int, float or string.
- [x] $variable, ${variable} and parameter ($1 etc).
- [x] ${parameter-default} ${parameter:-default} If parameter not set, use default.
- [x] ${parameter=default}, ${parameter:=default} If parameter not set, set it to default.
- [x] ${parameter+alt_value}, ${parameter:+alt_value} If parameter set, use alt_value, else use null string.
- [x] ${parameter?err_msg}, ${parameter:?err_msg} If parameter set, use it, else print err_msg and abort the script with an exit status of 1.
- [x] variable builtin ${#string} (var length).
- [x] variable builtin ${string:pos} (extract substring at pos).
- [x] variable builtin ${string:pos:len} (extract len substring at pos).
- [x] variable builtin ${string#substr} (deletes shortest match of $substr from front of $string).
- [x] variable builtin ${string##substr} (deletes longest match of $substr from front of $string).
- [x] variable builtin ${string%substr} (deletes shortest match of $substr from back of $string).
- [x] variable builtin ${string%%substr} (deletes longest match of $substr from back of $string).
- [x] variable builtin ${string/substr/repl} (Replace first match of $substr with $repl).
- [x] variable builtin ${string//substr/repl} (Replace all matches of $substr with $repl).
- [x] variable builtin ${string/#substr/repl} (If $substr matches front end of $string, substitute $repl for $substr.).
- [x] variable builtin ${string/%substr/repl} (If $substr matches back end of $string, substitute $repl for $substr.).
- [x] variable builtin ${!varprefix*}, ${!varprefix@} (Matches names of all previously declared variables beginning with varprefix.).
- [ ] alias substitution and builtin.
- [ ] $(command) substitution (kind of similar to backtick).
- [ ] ~ expansion.
- [ ] !! expansion (history).
- [ ] {} expansion.
- [ ] $(( )) arithmetic expansion.
- [ ] [[ ]] expansion.
- [ ] [ ] expansion.
- [ ] * ? etc expansion.
- [ ] regexp support =~
- [x] POSIX characters classes [:alnum:] matches alphabetic or numeric characters. This is equivalent to A-Za-z0-9.
- [x] POSIX characters classes [:alpha:] matches alphabetic characters. This is equivalent to A-Za-z.
- [x] POSIX characters classes [:blank:] matches a space or a tab.
- [x] POSIX characters classes [:cntrl:] matches control characters.
- [x] POSIX characters classes [:digit:] matches (decimal) digits. This is equivalent to 0-9.
- [x] POSIX characters classes [:graph:] (graphic printable characters). Matches characters in the range of ASCII 33 - 126. This is the same as [:print:], below, but excluding the space character.
- [x] POSIX characters classes [:lower:] matches lowercase alphabetic characters. This is equivalent to a-z.
- [x] POSIX characters classes [:print:] (printable characters). Matches characters in the range of ASCII 32 - 126. This is the same as [:graph:], above, but adding the space character.
- [x] POSIX characters classes [:space:] matches whitespace characters (space and horizontal tab).
- [x] POSIX characters classes [:upper:] matches uppercase alphabetic characters. This is equivalent to A-Z.
- [x] POSIX characters classes [:xdigit:] matches hexadecimal digits. This is equivalent to 0-9A-Fa-f.
- [ ] if elif else fi.
- [ ] case "$var" in "value") command ;; "value2") command ;; esac
- [ ] for n in list do done { } may be used instead of do done
- [ ] for ((a=1; a<bla; a++)) do done { } may be used instead of do done
- [ ] while [condition] do done (optional brackets)
- [ ] while (( condition )) do done
- [ ] until [condition] do done
- [ ] do done
- [x] break
- [x] continue
- [ ] function function_name() { } and function() { }
- [ ] select variable in list (optional in list) do command break done
- [ ] command execution
- [ ] pipes
- [ ] > < >> << 2>&1 etc redirections. don’t forget <<EOF kind.
- [ ] <(command list) >(command list) process substitution.
- [ ] || && operators
- [x] echo (complete support)
- [ ] printf
- [ ] read
- [ ] cd
- [ ] pwd
- [ ] popd
- [ ] pushd
- [ ] dirs
- [ ] let += -= /= *= %=
- [ ] eval
- [ ] set
- [ ] unset
- [ ] export
- [ ] declare
- [ ] typeset
- [ ] readonly
- [ ] getopts
- [ ] source .
- [x] exit
- [ ] exec
- [ ] shopt
- [ ] caller
- [x] true
- [x] false
- [ ] type
- [ ] hash
- [ ] bind
- [ ] help
- [ ] jobs
- [ ] disown
- [ ] bg
- [ ] fg
- [ ] wait
- [ ] suspend
- [x] logout
- [ ] times
- [ ] kill
- [ ] killall
- [ ] command
- [ ] builtin
- [ ] enable
- [ ] autoload
- [ ] .pest files will need a complete overhaul to be cleaner and better structured.

### core
- [ ] Parsing loop needs to be completely overhauled / rewritten.
- [ ] Make simple variables and arrays methods prototypes more similar (get rid of geti, gets… ?).
- [ ] Add some kind of Error return type for methods so we can follow properly the way things run.
- [x] Variables management (simple variables, aliases and single dimension arrays).
- [ ] Variables assignment.
- [ ] Arrays assignment.
- [ ] $variable, ${variable} and parameter ($1 etc) expansion.
- [ ] ${parameter-default} ${parameter:-default} expansion.
- [ ] ${parameter=default}, ${parameter:=default} expansion.
- [ ] ${parameter+alt_value}, ${parameter:+alt_value} expansion.
- [ ] ${parameter?err_msg}, ${parameter:?err_msg} expansion.
- [ ] variable builtin ${#string} expansion.
- [ ] variable builtin ${string:pos} expansion.
- [ ] variable builtin ${string:pos:len} expansion.
- [ ] variable builtin ${string#substr} expansion.
- [ ] variable builtin ${string##substr} expansion.
- [ ] variable builtin ${string%substr} expansion.
- [ ] variable builtin ${string%%substr} expansion.
- [ ] variable builtin ${string/substr/repl} expansion.
- [ ] variable builtin ${string//substr/repl} expansion.
- [ ] variable builtin ${string/#substr/repl} expansion.
- [ ] variable builtin ${string/%substr/repl}  expansion.
- [ ] variable builtin ${!varprefix*}, ${!varprefix@} expansion.
- [ ] Complete prompt codes support (partly done).
- [ ] Clean up code (commented tries here and there…).
- [ ] Split up code (variables.rs prompt.rs etc. Partly done).
- [ ] Write everything linked to builtins, pipes etc (yeah, that will be *very* long).
- [ ] Have a 100% code coverage when it comes to documentation *and* testing.
- [ ] Multi-lingual support (i18n ? l20n.rs ?).
- [ ] Proper color management (using [term](https://crates.io/crates/term) crate maybe ?).
- [ ] Think of ways to get speed with RuSh (read: be faster than Bash). JIT ? Some kind of « parsed script ready to be executed » ?
- [ ] Support float processing. (kind of done, see variable management above).
- [ ] Deprecate several bashy thing (older versions compatibility etc, bash is so much of a noodles plate now due to history that I won’t be able to cover every point so I’ll have to focus).
- [x] Use [hashbrown](https://github.com/Amanieu/hashbrown) instead of [seahash](https://crates.io/crates/seahash). It will be used by [default](https://github.com/rust-lang/rust/pull/58623) in std once rust 1.36 is out.
- [x] Use of cargo clippy.
- [x] Use of cargo fmt.
- [x] Use of tarpaulin.
- [ ] Use of criterion for benchmarking.
- [ ] Put tests and benches in their own directories.
- [ ] Use of error-chain, some fuzzer ?
- [ ] So many other things. multidimensionnal arrays ?

## Building

You need Rust 1.37.0 or newer. I do use latest stable versions and 2018 edition.

It’s as simple as:

    cargo build

## Running
    cargo run

## To quit
    ctrl+d
