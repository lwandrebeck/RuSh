[![Build Status](https://travis-ci.org/lwandrebeck/RuSh.svg?branch=master)](https://travis-ci.org/lwandrebeck/RuSh)

# RuSh: A shell written in Rust

## Quick introduction

RuSh aims to be (maybe one day) a POSIX Shell, Bash compatible with candies. RuSh is GPL3.
Please note that this is a personal project (read not funded), in order to learn Rust language (that does not mean feedback or patches are not welcome (Thanks [Guillaume](https://github.com/GuillaumeGomez) !) :)).

Right now, RuSh is definitely not useable on a day to day basis. Only pwd, cd commands, and classical (external) commands are implemented (and cd is still buggy). Prompt is partially implemented too. Don’t even think of running some kind of script, internals (if, while, pipe, you name it) are not yet implemented. Only a single command may (sort of) work.

Parser is definitely über basic and far from finished nor in its definitive shape. It does *not* allow for now a complete implementation of POSIX shell syntax. We plan to use « [nom](https://github.com/Geal/nom) » at some time to have a fully functionnal lexer/parser. Localization is TBD. Color management is to be implemented using « [term](https://crates.io/crates/term) crate.

The TODO list is so huge I don’t even dare to begin to write it. unimplemented!() macro will (partly) show you where work is needed. Just keep in mind function prototypes are not in definitive shape. Don’t forget too that parser needs a complete overhaul, that will (may ?) imply to change quite a bit of…everything. But work done before parser change will help to sort out bugs, so most work already done won’t be lost.

##What features RuSh will have (maybe one day) that bash doesn’t ?

* implement some coreutils commands as internals (no, I don’t think I’ll implement sed or awk, that would be too much work). Avoiding some forks, we’ll gain speed.
* allow float processing.
* Add some kind of JIT (speed anyone ?).
* Last but not least, I *may* begin (as a standalone project) a GTK3 interface à la gnome-terminal, with some GNU Screen features (here won’t be any work on it until RuSh is - kind of - finished).

## Building

It’s as simple as:
    cargo build

## Running
    cargo run

