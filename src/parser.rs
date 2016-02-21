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
use std::str;
use nom::*;
use nom::IResult::*;
use error::*;
use std::fs::File;
use builtins;

// Control operators
// newline, ||, &&, &, ;, ;;, |, |&, (, ).

// Metacharacters (when unquoted)
// blank, |, &, ;, (, ), <, >.

// 13:24 <geal> escaped prend d'abor un parser pour les caractères "normaux", puis le caractère de contrôle, puis les caractères échappés
// 13:25 <geal> escaped_transform prend le même genre d'argument, mais construit un nouveau résultat, en enlevant le caractère de contrôle et en ajoutant le résultat du dernier parser

/// Function to check if character is a valid first variable name (alphabetic or _ only)
#[inline]
pub fn is_alphabetic_or_underscore(chr: char) -> bool {
    is_alphabetic(chr as u8) || chr == '_'
}

/// Helper for is_alphabetic_or_underscore
named!(alphabetic_or_underscore<&str, &str>, take_while_s!(is_alphabetic_or_underscore));

/// Function to check if non first variable name character is valid (alphanumeric or _ only)
#[inline]
pub fn is_alphanumeric_or_underscore(chr: char) -> bool {
    is_alphanumeric(chr as u8) || chr == '_'
}

/// Helper for is_alphanumeric_or_underscore
named!(alphanumeric_or_underscore<&str, &str>, take_while_s!(is_alphanumeric_or_underscore));

/// As defined in some bash doc, returns true if current character is |, &, ;, (, ), < or >
#[inline]
pub fn is_metacharacter(chr: char) -> bool {
    chr == '|' || chr == '&' || chr == ';' || chr == '(' || chr == ')' || chr == '<' || chr == '>'
}

/// Helper for is_metacharacter
named!(metacharacter<&str, &str>, take_while_s!(is_metacharacter));

/// Returns true if current character is a dot.
#[inline]
pub fn is_dot(chr: char) -> bool {
    chr == '.'
}

/// Helper for is_dot
named!(dot<&str, &str>, take_while_s!(is_dot));

/// Returns true if current character is a star.
#[inline]
pub fn is_star(chr: char) -> bool {
    chr == '*'
}

/// Helper for is_star
named!(star<&str, &str>, take_while_s!(is_star));

/// Returns true if current character is an at.
#[inline]
pub fn is_at(chr: char) -> bool {
    chr == '@'
}

/// Helper for is_at
named!(at<&str, &str>, take_while_s!(is_at));

/// Returns true if current character is a closing parenthesis.
#[inline]
pub fn is_cparenthesis(chr: char) -> bool {
    chr == ')'
}

/// Helper for is_cparenthesis
named!(cparenthesis<&str, &str>, take_while_s!(is_cparenthesis));

/// Returns true if current character is an opening parenthesis.
#[inline]
pub fn is_oparenthesis(chr: char) -> bool {
    chr == '('
}

/// Helper for is_oparenthesis
named!(oparenthesis<&str, &str>, take_while_s!(is_oparenthesis));

/// Returns true if current character is a closing brace.
#[inline]
pub fn is_cbrace(chr: char) -> bool {
    chr == '}'
}

/// Helper for is_cbrace
named!(cbrace<&str, &str>, take_while_s!(is_cbrace));

/// Returns true if current character is an opening brace.
#[inline]
pub fn is_obrace(chr: char) -> bool {
    chr == '{'
}

/// Helper for is_obrace
named!(obrace<&str, &str>, take_while_s!(is_obrace));

/// Returns true if current character is a closing bracket.
#[inline]
pub fn is_cbracket(chr: char) -> bool {
    chr == ']'
}

/// Helper for is_cbracket
named!(cbracket<&str, &str>, take_while_s!(is_cbracket));

/// Returns true if current character is an opening bracket ([).
#[inline]
pub fn is_obracket(chr: char) -> bool {
    chr == '['
}

/// Helper for is_obracket
named!(obracket<&str, &str>, take_while_s!(is_obracket));

/// Returns true if current character is a closing chevron.
#[inline]
pub fn is_cchevron(chr: char) -> bool {
    chr == '>'
}

/// Helper for is_cchevron
named!(cchevron<&str, &str>, take_while_s!(is_cchevron));

/// Returns true if current character is an opening chevron (<).
#[inline]
pub fn is_ochevron(chr: char) -> bool {
    chr == '<'
}

/// Helper for is_ochevron
named!(ochevron<&str, &str>, take_while_s!(is_ochevron));

/// variable prefix
named!(variable_prefix, tag!("$"));

/// valid first character for a variable
named!(variable_first_char, tag!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_"));

/*
/// valid non first characters for a variable
named!(variable_car, many0!(tag!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890_"))); */

/*
/// Returns true if current character is the one we’re searching for.
#[inline]
pub fn is_in(chr: char, searched: char) -> bool {
    chr == searched
}

///Function helper take_while_s a given character is in a given &str
#[inline]
pub fn is_a_in(chr: char, input: &str) -> IResult<&str, &str> {
    take_while_s!(input, is_a!(chr as u8) )
} */

// TODO: only two macros for now for testing
/* #[derive(Debug,PartialEq,Eq)]
    enum Builtins {
        echo,
        pwd
    } */
//named!(echo, tag!("echo"));
//named!(pwd, tag!("pwd"));

/// try with alt!
/* named!(pub parse_shell_line<Builtins>, alt!(
      tag!("echo ") => { |_| Builtins::echo }
    | tag!("pwd") => { |_| Builtins::pwd }
    )); */

/// Very beginning of parser.
/*pub fn parse_shell_line(line: &str) -> Result<Vec<Vec<String>>, ShellError> {
    let components: Vec<&str> = line.split(' ').collect();
    alt_complete!( components[0],
        tag_s!(":") => { |_| builtins::colon(&components[1..]) }
      | tag_s!("pwd")  => { |_| builtins::pwd(&components[1..])  }
    )
}*/

/* pub fn parse_shell_file(filename: &str) -> Result<Vec<Vec<String>>, ShellError> {
    let mut f = File::open(filename).unwrap();
    let mut buffer = vec!();

    f.read_to_end(&mut buffer).unwrap();
    parse_shell_line(&buffer)
} */

pub fn parse_shell(line: &str) -> Result<String, ShellError> {
    //parse_shell_line(line)
    Ok(line.to_string())
}

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

named!(is_shebang_or_comment<&str, &str>, alt!(tag_s!("#!") | tag_s!("#")));
named!(is_escape_character<&str, &str>, tag_s!("\\"));
named!(is_double_quote, delimited!(char!('\"'), is_not!("\""), char!('\"')));
named!(is_single_quote, delimited!(char!('\''), is_not!("\'"), char!('\'')));
named!(is_ansi_c_quote, delimited!(tag!("$'"), is_not!("\'"), char!('\'')));
named!(is_locale_specific_translation, delimited!(tag!("$\""), is_not!("\""), char!('\"')));

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

/*
/// recognize a basic variable name
named!(basicvar <&str, &str>,
    recognize!(
            tuple!(
                one_of!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_"),
                take_while!(is_alphanumeric_or_underscore)
                  )
              )
); */

/*named!(basicvar <&str, &str>,
    recognize!(
        tuple!(
            one_of!(is_alphabetic || '_'),
                take_while_s!(is_alphanumeric || '_')
        )
    )
);*/

/*
/// manage variables format
/// -> switch
/// 01 none or one
/// 0+ 0 or more
/// 1+ 1 or more
/// | or
/// ab_or_us: alphabetic or underscore
/// an_or_us: alphanumeric or underscore
/// basicvar 1+ ab_or_us 0+ an_or_us
/// extendedvar  basicvar 01 [ -> 1+ Digit ]
///                            -> another var (recursive call) ]
/// $ -> Digit
      -> basicvar
      -> # Digit
      -> # basicvar
      -> $
      -> *
      -> @
      -> !
      -> ' -> \ -> 1+ OctDigit '
                -> x 1+ HexDigit '
           -> string (globs are not interpreted) '
      -> { -> $        |
              *        |
              @        |
              !        |
              #        |
              # extendedvar |
              extendedvar
                     -> }
                     -> : -> 1+ Digit -> }
                                    -> : -> 1+ Digit }
                                         -> another var (recursive call) }
                          -> another var (recursive call) -> }
                                                          -> : -> 1+ Digit }
                                                               -> another var (recursive call) }
                     -> # -> string (possible globs * etc) }
                          -> another var (recursive call) }
                     -> ## -> string (possible globs * etc) }
                           -> another var (recursive call) }
                     -> % -> string (possible globs * etc) }
                          -> another var (recursive call) }
                     -> %% -> string (possible globs * etc) }
                           -> another var (recursive call) }
                     -> / -> string (possible globs) 01 / -> string (globs are not interpreted) }
                                                          -> another var (recursive call) }
                          -> another var (recursive call) 01 / -> string (globs are not interpreted) }
                                                               -> another var (recursive call) }
                     -> // -> string (possible globs) 01 / -> string (globs are not interpreted) }
                                                           -> another var (recursive call) }
                           -> another var (recursive call) 01 / -> string (globs are not interpreted) }
                                                                -> another var (recursive call) }
                     -> /# -> string (possible globs) 01 / -> string (globs are not interpreted) }
                                                           -> another var (recursive call) }
                           -> another var (recursive call) 01 / -> string (globs are not interpreted) }
                                                                -> another var (recursive call) }
                     -> /% -> string (possible globs) 01 / -> string (globs are not interpreted) }
                                                           -> another var (recursive call) }
                           -> another var (recursive call) 01 / -> string (globs are not interpreted) }
                                                                -> another var (recursive call) }

pub fn variable(var: &str) -> Result<String, ShellError> {
    delimited!(
        tag_s!("${"),
} */

#[cfg(test)]

mod tests {
    use parser;
    use nom;

    #[test]
    fn is_ab_or_us() {
        let none: &str = "0'@-";
        let all: &str = "_aZ";
        let beg: &str = "aZ09";
        let mid: &str = "09a90";
        let end: &str = "0990a";
        assert_eq!(parser::alphabetic_or_underscore(none), nom::IResult::Done("0'@-", ""));
        assert_eq!(parser::alphabetic_or_underscore(all), nom::IResult::Done("", "_aZ"));
        assert_eq!(parser::alphabetic_or_underscore(beg), nom::IResult::Done("09", "aZ"));
        assert_eq!(parser::alphabetic_or_underscore(mid), nom::IResult::Done("09a90", ""));
        assert_eq!(parser::alphabetic_or_underscore(end), nom::IResult::Done("0990a", ""));
    }

    #[test]
    fn is_an_or_us() {
        let none: &str = "'@-<";
        let all: &str = "_aZ09";
        let beg: &str = "_09aZ+/@";
        let mid: &str = "+/@_0Z++";
        let end: &str = "()[];0";
        assert_eq!(parser::alphanumeric_or_underscore(none), nom::IResult::Done("'@-<", ""));
        assert_eq!(parser::alphanumeric_or_underscore(all), nom::IResult::Done("", "_aZ09"));
        assert_eq!(parser::alphanumeric_or_underscore(beg), nom::IResult::Done("+/@", "_09aZ"));
        assert_eq!(parser::alphanumeric_or_underscore(mid), nom::IResult::Done("+/@_0Z++", ""));
        assert_eq!(parser::alphanumeric_or_underscore(end), nom::IResult::Done("()[];0", ""));
    }

    #[test]
    fn is_mc() {
        let none: &str = "09aZ-+";
        let all: &str = "|&;()<>";
        let beg: &str = "|&;()<>aZ";
        let mid: &str = "09|&;()<>aZ_";
        let end: &str = "09aZ_|&;()<>";
        assert_eq!(parser::metacharacter(none), nom::IResult::Done("09aZ-+", ""));
        assert_eq!(parser::metacharacter(all), nom::IResult::Done("", "|&;()<>"));
        assert_eq!(parser::metacharacter(beg), nom::IResult::Done("aZ", "|&;()<>"));
        assert_eq!(parser::metacharacter(mid), nom::IResult::Done("09|&;()<>aZ_", ""));
        assert_eq!(parser::metacharacter(end), nom::IResult::Done("09aZ_|&;()<>", ""));
    }

    #[test]
    fn is_do() {
        let none: &str = "aZ09@+";
        let all: &str = "...";
        let beg: &str = "../";
        let mid: &str = "/../";
        let end: &str = "/..";
        assert_eq!(parser::dot(none), nom::IResult::Done("aZ09@+", ""));
        assert_eq!(parser::dot(all), nom::IResult::Done("", "..."));
        assert_eq!(parser::dot(beg), nom::IResult::Done("/", ".."));
        assert_eq!(parser::dot(mid), nom::IResult::Done("/../", ""));
        assert_eq!(parser::dot(end), nom::IResult::Done("/..", ""));
    }

    #[test]
    fn is_sta() {
        let none: &str = "..";
        let all: &str = "**";
        let beg: &str = "**/";
        let mid: &str = "/*/";
        let end: &str = "/**";
        assert_eq!(parser::star(none), nom::IResult::Done("..", ""));
        assert_eq!(parser::star(all), nom::IResult::Done("", "**"));
        assert_eq!(parser::star(beg), nom::IResult::Done("/", "**"));
        assert_eq!(parser::star(mid), nom::IResult::Done("/*/", ""));
        assert_eq!(parser::star(end), nom::IResult::Done("/**", ""));
    }

    #[test]
    fn is_at_test() {
        let none: &str = "..";
        let all: &str = "@@";
        let beg: &str = "@@/";
        let mid: &str = "/@/";
        let end: &str = "/@@";
        assert_eq!(parser::at(none), nom::IResult::Done("..", ""));
        assert_eq!(parser::at(all), nom::IResult::Done("", "@@"));
        assert_eq!(parser::at(beg), nom::IResult::Done("/", "@@"));
        assert_eq!(parser::at(mid), nom::IResult::Done("/@/", ""));
        assert_eq!(parser::at(end), nom::IResult::Done("/@@", ""));
    }

    #[test]
    fn is_cpar() {
        let none: &str = "..";
        let all: &str = "))";
        let beg: &str = "))/";
        let mid: &str = "/)/";
        let end: &str = "/))";
        assert_eq!(parser::cparenthesis(none), nom::IResult::Done("..", ""));
        assert_eq!(parser::cparenthesis(all), nom::IResult::Done("", "))"));
        assert_eq!(parser::cparenthesis(beg), nom::IResult::Done("/", "))"));
        assert_eq!(parser::cparenthesis(mid), nom::IResult::Done("/)/", ""));
        assert_eq!(parser::cparenthesis(end), nom::IResult::Done("/))", ""));
    }

    #[test]
    fn is_opar() {
        let none: &str = "..";
        let all: &str = "((";
        let beg: &str = "((/";
        let mid: &str = "/(/";
        let end: &str = "/((";
        assert_eq!(parser::oparenthesis(none), nom::IResult::Done("..", ""));
        assert_eq!(parser::oparenthesis(all), nom::IResult::Done("", "(("));
        assert_eq!(parser::oparenthesis(beg), nom::IResult::Done("/", "(("));
        assert_eq!(parser::oparenthesis(mid), nom::IResult::Done("/(/", ""));
        assert_eq!(parser::oparenthesis(end), nom::IResult::Done("/((", ""));
    }

    #[test]
    fn is_cbra() {
        let none: &str = "..";
        let all: &str = "}}";
        let beg: &str = "}}/";
        let mid: &str = "/}/";
        let end: &str = "/}}";
        assert_eq!(parser::cbrace(none), nom::IResult::Done("..", ""));
        assert_eq!(parser::cbrace(all), nom::IResult::Done("", "}}"));
        assert_eq!(parser::cbrace(beg), nom::IResult::Done("/", "}}"));
        assert_eq!(parser::cbrace(mid), nom::IResult::Done("/}/", ""));
        assert_eq!(parser::cbrace(end), nom::IResult::Done("/}}", ""));
    }

    #[test]
    fn is_obra() {
        let none: &str = "..";
        let all: &str = "{{";
        let beg: &str = "{{/";
        let mid: &str = "/{/";
        let end: &str = "/{{";
        assert_eq!(parser::obrace(none), nom::IResult::Done("..", ""));
        assert_eq!(parser::obrace(all), nom::IResult::Done("", "{{"));
        assert_eq!(parser::obrace(beg), nom::IResult::Done("/", "{{"));
        assert_eq!(parser::obrace(mid), nom::IResult::Done("/{/", ""));
        assert_eq!(parser::obrace(end), nom::IResult::Done("/{{", ""));
    }

    #[test]
    fn is_cbrak() {
        let none: &str = "..";
        let all: &str = "]]";
        let beg: &str = "]]/";
        let mid: &str = "/]/";
        let end: &str = "/]]";
        assert_eq!(parser::cbracket(none), nom::IResult::Done("..", ""));
        assert_eq!(parser::cbracket(all), nom::IResult::Done("", "]]"));
        assert_eq!(parser::cbracket(beg), nom::IResult::Done("/", "]]"));
        assert_eq!(parser::cbracket(mid), nom::IResult::Done("/]/", ""));
        assert_eq!(parser::cbracket(end), nom::IResult::Done("/]]", ""));
    }

    #[test]
    fn is_obrak() {
        let none: &str = "..";
        let all: &str = "[[";
        let beg: &str = "[[/";
        let mid: &str = "/[/";
        let end: &str = "/[[";
        assert_eq!(parser::obracket(none), nom::IResult::Done("..", ""));
        assert_eq!(parser::obracket(all), nom::IResult::Done("", "[["));
        assert_eq!(parser::obracket(beg), nom::IResult::Done("/", "[["));
        assert_eq!(parser::obracket(mid), nom::IResult::Done("/[/", ""));
        assert_eq!(parser::obracket(end), nom::IResult::Done("/[[", ""));
    }

    #[test]
    fn is_cchev() {
        let none: &str = "..";
        let all: &str = ">>";
        let beg: &str = ">>/";
        let mid: &str = "/>/";
        let end: &str = "/>>";
        assert_eq!(parser::cchevron(none), nom::IResult::Done("..", ""));
        assert_eq!(parser::cchevron(all), nom::IResult::Done("", ">>"));
        assert_eq!(parser::cchevron(beg), nom::IResult::Done("/", ">>"));
        assert_eq!(parser::cchevron(mid), nom::IResult::Done("/>/", ""));
        assert_eq!(parser::cchevron(end), nom::IResult::Done("/>>", ""));
    }

    #[test]
    fn is_ochev() {
        let none: &str = "..";
        let all: &str = "<<";
        let beg: &str = "<</";
        let mid: &str = "/</";
        let end: &str = "/<<";
        assert_eq!(parser::ochevron(none), nom::IResult::Done("..", ""));
        assert_eq!(parser::ochevron(all), nom::IResult::Done("", "<<"));
        assert_eq!(parser::ochevron(beg), nom::IResult::Done("/", "<<"));
        assert_eq!(parser::ochevron(mid), nom::IResult::Done("/</", ""));
        assert_eq!(parser::ochevron(end), nom::IResult::Done("/<<", ""));
    }
}

