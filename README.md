[![Build Status](https://travis-ci.org/lwandrebeck/RuSh.svg?branch=master)](https://travis-ci.org/lwandrebeck/RuSh)
[![Coverage Status](https://coveralls.io/repos/github/lwandrebeck/RuSh/badge.svg?branch=master)](https://coveralls.io/github/lwandrebeck/RuSh?branch=master)

# RuSh: A shell written in Rust

## Quick introduction

RuSh aims to be (maybe one day) a POSIX Shell, Bash compatible (or close to) with candies. Source code is [GPL3](http://www.gnu.org/licenses/gpl-3.0.html).
Please note that this is a personal project (read not funded), in order to learn Rust language (that does not mean feedback or patches are not welcome (Thanks [Guillaume](https://github.com/GuillaumeGomez) ) and [Mathieu](https://github.com/kali) :)).

Right now, RuSh is definitely not useable. A couple little things have been done, but 99% (at _least_) have to be written.

## TODO:
### parser
- [x] unquoted string suppport. (espace character taken care of)
- [x] "quoted" string support. (espace character taken care of)
- [x] 'quoted' string support. (espace character taken care of)
- [x] \`backticked\` string support. (espace character taken care of)
- [x] binary numbers support (uint only for now ?). 0b mark. (i64 by default)
- [x] octal numbers support (uint only for now ?). 0 mark. (i64 by default)
- [x] hexadecimal numbers support (uint only for now ?). 0x mark. (i64 by default)
- [x] (signed) int support (i64 by default).
- [x] (signed) float support (f64 by default). exponent correctly parsed.
- [x] variable assignment. (type is autodetected)
- [x] shebang (#!) and comments are correctly parsed.
- [ ] : syntax
- [ ] compgen builtin.
- [ ] complete builtin.
- [ ] compopt builtin.
- [ ] fc builtin.
- [ ] history builtin.
- [ ] local builtin.
- [ ] mapfile builtin.
- [ ] readarray builtin.
- [ ] return builtin.
- [ ] shift builtin.
- [ ] test builtin.
- [ ] trap builtin.
- [ ] ulimit builtin.
- [ ] umask builtin.
- [ ] unalias builtin.
- [ ] use [pest](https://github.com/dragostis/pest) to parse prompt
- [ ] arrays support.
- [ ] $variable, ${variable} and parameter ($1 etc) expansion.
- [ ] ${parameter-default} ${parameter:-default} If parameter not set, use default.
- [ ] ${parameter=default}, ${parameter:=default} If parameter not set, set it to default.
- [ ] ${parameter+alt_value}, ${parameter:+alt_value} If parameter set, use alt_value, else use null string.
- [ ] ${parameter?err_msg}, ${parameter:?err_msg} If parameter set, use it, else print err_msg and abort the script with an exit status of 1.
- [ ] variable builtin ${#string} (var length).
- [ ] variable builtin ${string:pos} (extract substring at pos).
- [ ] variable builtin ${string:pos:len} (extract len substring at pos).
- [ ] variable builtin ${string#substr} (deletes shortest match of $substr from front of $string).
- [ ] variable builtin ${string##substr} (deletes longest match of $substr from front of $string).
- [ ] variable builtin ${string%substr} (deletes shortest match of $substr from back of $string).
- [ ] variable builtin ${string%%substr} (deletes longest match of $substr from back of $string).
- [ ] variable builtin ${string/substr/repl} (Replace first match of $substr with $repl).
- [ ] variable builtin ${string//substr/repl} (Replace all matches of $substr with $repl).
- [ ] variable builtin ${string/#substr/repl} (If $substr matches front end of $string, substitute $repl for $substr.).
- [ ] variable builtin ${string/%substr/repl} (If $substr matches back end of $string, substitute $repl for $substr.).
- [ ] variable builtin ${!varprefix*}, ${!varprefix@} (Matches names of all previously declared variables beginning with varprefix.).
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
- [ ] POSIX characters classes [:alnum:] matches alphabetic or numeric characters. This is equivalent to A-Za-z0-9.
- [ ] POSIX characters classes [:alpha:] matches alphabetic characters. This is equivalent to A-Za-z.
- [ ] POSIX characters classes [:blank:] matches a space or a tab.
- [ ] POSIX characters classes [:cntrl:] matches control characters.
- [ ] POSIX characters classes [:digit:] matches (decimal) digits. This is equivalent to 0-9.
- [ ] POSIX characters classes [:graph:] (graphic printable characters). Matches characters in the range of ASCII 33 - 126. This is the same as [:print:], below, but excluding the space character.
- [ ] POSIX characters classes [:lower:] matches lowercase alphabetic characters. This is equivalent to a-z.
- [ ] POSIX characters classes [:print:] (printable characters). Matches characters in the range of ASCII 32 - 126. This is the same as [:graph:], above, but adding the space character.
- [ ] POSIX characters classes [:space:] matches whitespace characters (space and horizontal tab).
- [ ] POSIX characters classes [:upper:] matches uppercase alphabetic characters. This is equivalent to A-Z.
- [ ] POSIX characters classes [:xdigit:] matches hexadecimal digits. This is equivalent to 0-9A-Fa-f.
- [ ] if elif else fi.
- [ ] case "$var" in "value") command ;; "value2") command ;; esac.
- [ ] for n in list do done { } may be used instead of do done
- [ ] for ((a=1; a<bla; a++)) do done { } may be used instead of do done
- [ ] while [condition] do done (optional brackets)
- [ ] while (( condition )) do done
- [ ] until [condition] do done
- [ ] do done
- [ ] break
- [ ] continue
- [ ] function function_name() { } and function() { }
- [ ] select variable in list (optional in list) do command break done
- [ ] command execution
- [ ] pipes
- [ ] > < >> << 2>&1 etc redirections. don’t forget <<EOF kind.
- [ ] <(command list) >(command list) process substitution.
- [ ] || && operators
- [ ] echo
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
- [ ] exit
- [ ] exec
- [ ] shopt
- [ ] caller
- [ ] true
- [ ] false
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
- [ ] logout
- [ ] times
- [ ] kill
- [ ] killall
- [ ] command
- [ ] builtin
- [ ] enable
- [ ] autoload

### core
- [ ] Take a decision about variables management implemention (I’m currently (albeit slowly, fighting the borrow checker ;)) working on it, using a hashmap with enum)
- [ ] Rewrite prompt routine once [pest](https://github.com/dragostis/pest) parser is done.
- [ ] Clean up code (commented tries here and there…)
- [ ] Split up code (variables.rs prompt.rs etc).
- [ ] Write everything linked to builtins, pipes etc (yeah, that will be *very* long)
- [ ] Have a 100% code coverage when it comes to documentation *and* testing.
- [ ] Multi-lingual support (i18n ? l20n.rs ?)
- [ ] Proper color management (using [term](https://crates.io/crates/term) crate maybe ?)
- [ ] Think of ways to get speed with RuSh (read: be faster than Bash). JIT ? Some kind of « parsed script ready to be executed » ?
- [ ] Support float processing. (kind of done, see variable management above)
- [ ] Deprecate several bashy thing (older versions compatibility etc, bash is so much of a noodles plate now due to history that I won’t be able to cover every point so I’ll have to focus).
- [x] Use [seahash](https://crates.io/crates/seahash) instead of fnv (Kind of done, variables management needs work now, see above).
- [ ] So many other things. multidimensionnal arrays ? Use of clippy, rustfmt, error-chain, tarpaulin, some fuzzer ?

## Building

It’s as simple as:

    cargo build

## Running
    cargo run

