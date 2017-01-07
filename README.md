[![Build Status](https://travis-ci.org/lwandrebeck/RuSh.svg?branch=master)](https://travis-ci.org/lwandrebeck/RuSh)
[![Coverage Status](https://coveralls.io/repos/github/lwandrebeck/RuSh/badge.svg?branch=master)](https://coveralls.io/github/lwandrebeck/RuSh?branch=master)

# RuSh: A shell written in Rust

## Quick introduction

RuSh aims to be (maybe one day) a POSIX Shell, Bash compatible (or close to) with candies. Source code is [GPL3](http://www.gnu.org/licenses/gpl-3.0.html).
Please note that this is a personal project (read not funded), in order to learn Rust language (that does not mean feedback or patches are not welcome (Thanks [Guillaume](https://github.com/GuillaumeGomez) !) :)).

Right now, RuSh is definitely not useable. A couple little things have been done, but nothing serious. To be fair, this is still the drawing board step. The first few tens of lines were written like good ol’ C. And it may not be the brightest idea given features brought by Rust. So it’s time to think about Trait and such so code becomes more modern and maintenable. Please be patient (and/or provide patches) if you’re eager to try RuSh :)

20160821: No, this project is not dead. Work is (slowly) ongoing to switch to impl. Please see traits branch (not yet pushed, it is quite a complete mess right now).
20161208: OK, it is for now stalled, but not yet abandonned. I *may* be able to work again on it somewhere in 2017.

## Uncomplete and unordered TODO list:
* Fill up the drawing board with trait, impl etc, so code organization is more or less fixed until code is once again written.
* Master [nom](https://github.com/Geal/nom) to write the parser *or* use [LALRPOP](https://github.com/nikomatsakis/lalrpop) which seems easier to dive in and time passing by, I’m more and more inclined to use it.
* Have a 100% code coverage when it comes to documentation *and* testing.
* Multi-lingual support (i18n ? l20n.rs ?)
* Proper color management (using [term](https://crates.io/crates/term) crate maybe ?)
* Think of ways to get speed with RuSh (read: be faster than Bash). JIT ? Some kind of « parsed script ready to be executed » ?
* Support float processing.
* Deprecate several bashy thing (older versions compatibility etc, bash is so much of a noodles plate now due to history that I won’t be able to cover every point so I’ll have to focus).
* Use [seahash](https://crates.io/crates/seahash) instead of fnv.
* So many things.

## Building

It’s as simple as:

    cargo build

## Running
    cargo run

