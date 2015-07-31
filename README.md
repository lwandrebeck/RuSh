# RuSh: A shell written in Rust

RuSh aims to be (maybe one day) a POSIX Shell, Bash compatible with candies.
Please note that it is a personnal project, in order to learn Rust language (that does not mean feedback or patches are not welcome :)).

Right now, RuSh is definitely not useable. Only pwd and cd commands are implemented (and cd is still buggy).

Parser is basic and far from finished or in its definitive shape. Localization is TBD.

The TODO list is so huge I don’t even dare to begin to write it.

What features RuSh will have (maybe one day) that bash doesn’t ?
- implement some coreutils commands as internals (no, I don’t think I’ll implement sed or awk, that would be too much work). Avoiding some forks, we’ll gain speed.
- allow float processing.
- Add some kind of JIT (speed anyone ?).
- Last but not least, a GTK3 interface à la gnome-terminal, with some GNU Screen features (which is some kind of a standalone project, I agree. There won’t be any work on it until RuSh is - kind of - finished)

RuSh is GPL3.
