/*
 * parser.rs
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

 //! RuSh parser
 //!
 //! Every functions related to parsing of shell input and files are located in that file
 //! nom 1.2 is used.

use nom::*;

// Control operators
// newline, ||, &&, &, ;, ;;, |, |&, (, ).

// Metacharacters (when unquoted)
// blank, |, &, ;, (, ), <, >.

// 13:24 <geal> escaped prend d'abor un parser pour les caractères "normaux", puis le caractère de contrôle, puis les caractères échappés
// 13:25 <geal> escaped_transform prend le même genre d'argument, mais construit un nouveau résultat, en enlevant le caractère de contrôle et en ajoutant le résultat du dernier parser

/// As defined in some bash doc, returns true is current character is |, &, ;, (, ), < or >
pub fn is_metacharacter(chr: char) -> bool {
    chr == '|' || chr == '&' || chr == ';' || chr == '(' || chr == ')' || chr == '<' || chr == '>'
}

named!(metacharacter<&str, &str>, take_till_s!(is_metacharacter));

/// Returns true if current character is a dot.
pub fn is_dot(chr: char) -> bool {
    chr == '.'
}

named!(dot<&str, &str>, take_till_s!(is_dot));

/// Returns true if current character is a star.
pub fn is_star(chr: char) -> bool {
    chr == '*'
}

named!(star<&str, &str>, take_till_s!(is_star));

/// Returns true if current character is an at.
pub fn is_at(chr: char) -> bool {
    chr == '@'
}

named!(at<&str, &str>, take_till_s!(is_at));

/// Returns true if current character is a closing parenthesis.
pub fn is_cparenthesis(chr: char) -> bool {
    chr == ')'
}

named!(cparenthesis<&str, &str>, take_till_s!(is_cparenthesis));

/// Returns true if current character is an opening parenthesis.
pub fn is_oparenthesis(chr: char) -> bool {
    chr == '('
}

named!(oparenthesis<&str, &str>, take_till_s!(is_oparenthesis));

/// Returns true if current character is a closing bracket.
pub fn is_cbracket(chr: char) -> bool {
    chr == '}'
}

named!(cbracket<&str, &str>, take_till_s!(is_cbracket));

/// Returns true if current character is an opening bracket.
pub fn is_obracket(chr: char) -> bool {
    chr == '{'
}

named!(obracket<&str, &str>, take_till_s!(is_obracket));

//named!(script_to_parse<&str, &str>, alt_complete!(take_until_s(eof) | take_until_s(line_ending)));
// Take care of variable definition
/* chain!(key_value, <&[u8], (&str, &str)>,
    key:    parameter_parser    ~
            equal               ~
    value:  value_parser        ~
            space?              ~
            comment_body?       ~
            line_ending?        ~
            backslash?); */

//named!(variable, delimited!(char!('$', alt!(call!(alphanumeric) | call!(

//named!(blank, call!(nom::space()));
//named!(line_to_parse, take_till!(line_ending));

named!(shebang_or_comment<&str, &str>, alt!(tag_s!("#!") | tag_s!("#")));
named!(escape_character<&str, &str>, tag_s!("\\"));
named!(double_quote, delimited!(char!('\"'), is_not!("\""), char!('\"')));
named!(single_quote, delimited!(char!('\''), is_not!("\'"), char!('\'')));
named!(ansi_c_quote, delimited!(tag!("$'"), is_not!("\'"), char!('\'')));
named!(locale_specific_translation, delimited!(tag!("$\""), is_not!("\""), char!('\"')));

//pub fn is_eof(chr: u8) -> bool {
//    eof(chr)
//}

//pub fn is_line_ending(chr: u8) -> bool {
//    line_ending(chr)
//}

//named!(script_to_parse, alt_complete!(take_till_s!(is_eof) | take_till_s!(is_line_ending)));
//named!(line_to_parse, take_until!("\n"));

/* fn escape_transform() {
    use std::string;

    named!(esc< String >, map!(escaped_transform!(call!(alpha), '\\',
      alt!(
          tag!("\\")       => { |_| &b"\\"[..] }
        | tag!("\"")       => { |_| &b"\""[..] }
        | tag!("n")        => { |_| &b"\n"[..] }
        | tag!("t")        => { |_| &b"\t"[..] }
      )), to_string())
    );
} */

// From Bash reference manual
// 1) Reads its input from a file (see Shell Scripts), from a string supplied as an argument to the -c invocation option (see Invoking Bash), or from the user’s terminal.
// 2) Breaks the input into words and operators, obeying the quoting rules described in Quoting. These tokens are separated by metacharacters. Alias expansion is performed by this step (see Aliases).
// 3) Parses the tokens into simple and compound commands (see Shell Commands).
// 4) Performs the various shell expansions (see Shell Expansions), breaking the expanded tokens into lists of filenames (see Filename Expansion) and commands and arguments.
// 5) Performs any necessary redirections (see Redirections) and removes the redirection operators and their operands from the argument list.
// 6) Executes the command (see Executing Commands).
// 7) Optionally waits for the command to complete and collects its exit status (see Exit Status).

// from http://mywiki.wooledge.org/BashParser
// 1) Read data to execute. Bash always reads your script or commands on the bash command prompt line by line. If your line ends with a backslash character, bash reads another line before processing the command and appends that other line to the current, with a literal newline inbetween.
// 2) Process quotes. Once Bash has read in your line of data, it looks through it in search of quotes. The first quote it finds triggers a quoted state for all characters that follow up until the next quote of the same type. If the quoted state was triggered by a double quote ("..."), all characters except for $, " and \ lose any special meaning they might have. That includes single quotes, spaces and newlines, etc. If the quoted state was triggered by a single quote ('...'), all characters except for ' lose their special meaning. Yes, also $ and \. Therefore, the following command will produce literal output
// 3) Step 3: Split the read data into commands. Our line is now split up into separate commands using ; as a command separator. Remember from the previous step that any ; characters that were quoted or escaped do not have their special meaning anymore and will not be used for command splitting. They will just appear in the resulting command line literally
// The following steps are executed for each command that resulted from splitting up the line of data
// 4) Parse special operators. Look through the command to see whether there are any special operators such as {..}, <(..), < ..., <<< .., .. | .., etc. These are all processed in a specific order. Redirection operators are removed from the command line, other operators are replaced by their resulting expression (eg. {a..c} is replaced by a b c).
// 5) Perform Expansions. Bash has many operators that involve expansion. The simplest of these is $parameter. The dollar sign followed by the name of a parameter, which optionally might be surrounded by braces, is called Parameter Expansion. What Bash does here is basically just replace the Parameter Expansion operator with the contents of that parameter. As such, the command echo $USER will in this step be converted to echo lhunath with me. Other expansions include Pathname Expansion (echo *.txt), Command Substitution (rm "$(which nano)"), etc.
// 6) Split the command into a command name and arguments. The name of the command Bash has to execute is always the first word in the line. The rest of the command data is split into words which make the arguments. This process is called Word Splitting. Bash basically cuts the command line into pieces wherever it sees whitespace. This whitespace is completely removed and the pieces are called words. Whitespace in this context means: Any spaces, tabs or newlines that are not escaped. (Escaped spaces, such as spaces inside quotes, lose their special meaning of whitespace and are not used for splitting up the command line. They appear literally in the resulting arguments.) As such, if the name of the command that you want to execute or one of the arguments you want to pass contains spaces that you don't want bash to use for cutting the command line into words, you can use quotes or the backslash character
// 7) Execute the command. Now that the command has been parsed into a command name and a set of arguments, Bash executes the command and sets the command's arguments to the list of words it has generated in the previous step. If the command type is a function or builtin, the command is executed by the same Bash process that just went through all these steps. Otherwise, Bash will first fork off (create a new bash process), initialize the new bash processes with the settings that were parsed out of this command (redirections, arguments, etc.) and execute the command in the forked off bash process (child process). The parent (the Bash that did these steps) waits for the child to complete the command.
// After these steps, the next command, or next line is processed. Once the end of the file is reached (end of the script or the interactive bash session is closed) bash stops and returns the exit code of the last command it has executed.

// from processing the command line, http://www.informit.com/articles/article.aspx?p=441605&seqNum=9
// 1) history expansion in case of interactive shell (!! command, set +o histexpand to disable, automatically disabled for scripts.)
// 2) alias substitution (on for interactive shell, off for scripts, shopt –u expand_aliases to disable)
// 3) parse/isolate tokens/words.
// 4) command line expansion: parse each token for special characters.
    // 4.1) brace expansion. on by default on interactive and scripts. set +o braceexpand to disable. echo chap_{one,two,three}.txt et cp /usr/local/src/C/{main,f1,f2,tmp}.c .
    // 4.2) tilde expansion. ~/ ~login ~invalidlogin -> ~invalidlogin, ~+ -> PWD ~- -> OLDPWD
    // 4.3) parameter and variable expansion: $BLA $1, not if \$BLA, '$BLA', $(
    // 4.4) arithmetic expansion $(( expression )). $ is not mandatory. let expression does the same thing. let a=5+3 b=7+2
    // 4.5) command substitution $( command ) `command`
    // 4.6) word splitting on $IFS (defaut value if unset, space, tab, newline.
    // 4.7) pathname expansion. unquoted * ? [ ]. if no file/path matches, token is left as is. See noglob, nullglob, dotglob, nocaseglob
    // 4.8) process substitution. <(command)
// 5) quote removal (', ", \ being not the result of expansion)

