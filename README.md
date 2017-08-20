[![Build Status](https://travis-ci.org/lwandrebeck/RuSh.svg?branch=master)](https://travis-ci.org/lwandrebeck/RuSh)
[![Coverage Status](https://coveralls.io/repos/github/lwandrebeck/RuSh/badge.svg?branch=master)](https://coveralls.io/github/lwandrebeck/RuSh?branch=master)

# RuSh: A shell written in Rust

## Quick introduction

RuSh aims to be (maybe one day) a POSIX Shell, Bash compatible (or close to) with candies. Source code is [GPL3](http://www.gnu.org/licenses/gpl-3.0.html).
Please note that this is a personal project (read not funded), in order to learn Rust language (that does not mean feedback or patches are not welcome (Thanks [Guillaume](https://github.com/GuillaumeGomez) ) and [Mathieu](https://github.com/kali) :)).

Right now, RuSh is definitely not useable. A couple little things have been done, but nothing serious.

## Uncomplete and unordered TODO list:
* Master [pest](https://github.com/dragostis/pest) to write the parser
* Have a 100% code coverage when it comes to documentation *and* testing.
* Multi-lingual support (i18n ? l20n.rs ?)
* Proper color management (using [term](https://crates.io/crates/term) crate maybe ?)
* Think of ways to get speed with RuSh (read: be faster than Bash). JIT ? Some kind of « parsed script ready to be executed » ?
* Support float processing.
* Deprecate several bashy thing (older versions compatibility etc, bash is so much of a noodles plate now due to history that I won’t be able to cover every point so I’ll have to focus).
* Use [seahash](https://crates.io/crates/seahash) instead of fnv (Kind of done, variables management needs work now).
* So many things. multidimensionnal arrays ? Use clippy, rustfmt, error-chain, tarpaulin

## Building

It’s as simple as:

    cargo build

## Running
    cargo run

