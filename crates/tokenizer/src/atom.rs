use logos::{Lexer, Logos};

pub(crate) fn get_equal_brackets(slice: &str) -> Option<usize> {
    if !slice.starts_with('[') {
        return None;
    }

    let count = slice.chars().skip(1).take_while(|v| v == &'=').count();
    if !matches!(slice.chars().nth(count + 1), Some('[')) {
        return None;
    }

    Some(count)
}

fn deal_lua_brackets(lexer: &mut Lexer<Atom>, skips: usize) -> bool {
    let equals = match lexer.slice().get(skips..).and_then(get_equal_brackets) {
        Some(v) => v,
        None => return false,
    };

    let mut is_tail = false;
    let mut tail_equals = 0;

    for (pos, char) in lexer.remainder().char_indices() {
        match (is_tail, char) {
            // check the amount of equals compared to initial one
            (true, ']') if tail_equals == equals => {
                lexer.bump(pos + 1);
                return true;
            }
            (true, '=') => tail_equals += 1,
            (_, ']') => {
                is_tail = true;
                tail_equals = 0;
            }
            _ => is_tail = false,
        }
    }

    false
}

fn parse_comment(lexer: &mut Lexer<Atom>) -> bool {
    if lexer
        .slice()
        .get(2..)
        .and_then(get_equal_brackets)
        .is_some()
    {
        deal_lua_brackets(lexer, 2)
    } else {
        for (pos, char) in lexer.remainder().char_indices() {
            if matches!(char, '\n' | '\0') {
                lexer.bump(pos + 1);
                break;
            }
        }
        true
    }
}

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Atom {
    #[token("and")]
    And,

    #[token("break")]
    Break,

    #[token("do")]
    Do,

    #[token("else")]
    Else,

    #[token("elseif")]
    ElseIf,

    #[token("end")]
    End,

    #[token("false")]
    False,

    #[token("for")]
    For,

    #[token("function")]
    Function,

    #[token("if")]
    If,

    #[token("in")]
    In,

    #[token("local")]
    Local,

    #[token("nil")]
    Nil,

    #[token("not")]
    Not,

    #[token("or")]
    Or,

    #[token("repeat")]
    Repeat,

    #[token("return")]
    Return,

    #[token("then")]
    Then,

    #[token("true")]
    True,

    #[token("until")]
    Until,

    #[token("while")]
    While,

    #[token("^")]
    Caret,

    #[token("::")]
    DoubleColon,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("...")]
    Ellipse,

    #[token("..")]
    TwoDots,

    #[token(".")]
    Dot,

    #[token("==")]
    TwoEqual,

    #[token("=")]
    Equal,

    #[token(">=")]
    GreaterThanEqual,

    #[token(">")]
    GreaterThan,

    #[token("#")]
    Hash,

    #[token("[")]
    LeftBracket,

    #[token("{")]
    LeftBrace,

    #[token("(")]
    LeftParen,

    #[token("<=")]
    LessThanEqual,

    #[token("<")]
    LessThan,

    #[token("-")]
    Minus,

    #[token("%")]
    Percent,

    #[token("+")]
    Plus,

    #[token("}")]
    RightBrace,

    #[token("]")]
    RightBracket,

    #[token(")")]
    RightParen,

    #[token(";")]
    Semicolon,

    #[token("/")]
    Slash,

    #[token("*")]
    Asterisk,

    #[token("~=")]
    TildeEqual,

    #[regex("@metatable")]
    MetatableTag,

    #[regex(r"--((\[=*\[)|\[)?", |l| parse_comment(l))]
    Comment,

    #[regex(r"0[xX][A-Fa-f0-9]+")]
    #[regex(r"\.[0-9]+([eE][+-]?[0-9]+)?")]
    #[regex(r"[0-9]+(.[0-9]+)?([eE][-+]?[0-9]+)?")]
    Number,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r"[ \t]*(\r?\n)?")]
    Whitespace,

    #[regex(r"#!.*\n")]
    Shebang,

    #[regex(r"\[(=*)\[", |l| deal_lua_brackets(l, 0))]
    BracketString,

    #[regex(r#""((\\")|[^"])*""#)]
    QuoteString,

    #[regex(r#"'((\\')|[^'])*'"#)]
    ApostropheString,

    #[error]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! tokenize_cmp {
        ($output:expr) => {
            let mut lexer = Atom::lexer($output);
            assert_eq!(lexer.nth(0), None);
        };
        ($output:expr, $cmp:expr) => {
            let mut lexer = Atom::lexer($output);
            assert_eq!(lexer.nth(0), Some($cmp));
        };
    }

    #[test]
    fn tokenize_number() {
        tokenize_cmp!("3", Atom::Number);
        tokenize_cmp!("3.0", Atom::Number);
        tokenize_cmp!("3.1416", Atom::Number);
        tokenize_cmp!("314.16e-2", Atom::Number);
        tokenize_cmp!("0.31416E1", Atom::Number);
        tokenize_cmp!("0xff", Atom::Number);
        tokenize_cmp!("0X56", Atom::Number);
    }

    #[test]
    fn tokenize_string() {
        tokenize_cmp!("\"Hello\"", Atom::QuoteString);
        tokenize_cmp!("'Hi'", Atom::ApostropheString);
        tokenize_cmp!("\"Hello\\\\\\n\"", Atom::QuoteString);
        tokenize_cmp!("'Hi\\\\\\n'", Atom::ApostropheString);
        tokenize_cmp!("[[alo\n123\"]]", Atom::BracketString);
        tokenize_cmp!("[==[alo\n123\"]==]", Atom::BracketString);

        tokenize_cmp!("'Hin", Atom::Unknown);
        tokenize_cmp!("[===[alo\n123\"]=]", Atom::Unknown);
        tokenize_cmp!("[=[alo\n123\"]===]", Atom::Unknown);
    }

    #[test]
    fn tokenize_shebang() {
        tokenize_cmp!("#!/usr/bin\n", Atom::Shebang);
        tokenize_cmp!("#!/usr/bin", Atom::Unknown);
    }

    #[test]
    fn tokenize_whitespace() {
        tokenize_cmp!(" ", Atom::Whitespace);
        tokenize_cmp!("\r\n", Atom::Whitespace);
        tokenize_cmp!("\t", Atom::Whitespace);
        tokenize_cmp!("\n", Atom::Whitespace);
        tokenize_cmp!("\r", Atom::Unknown);
    }

    #[test]
    fn tokenize_comment() {
        tokenize_cmp!("--Hello", Atom::Comment);
        tokenize_cmp!("--[Test1", Atom::Comment);
        tokenize_cmp!("--[[Complete comment!]]", Atom::Comment);
        tokenize_cmp!("--[==[Real one]==]", Atom::Comment);
        tokenize_cmp!("--[==[]===]", Atom::Unknown);
    }
}
