#![allow(unused_variables, dead_code)]

use std::fmt::{Formatter, Pointer, Write};
use std::{mem, vec};

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum TokenType {
    Ampersand = b'&',
    Asterisk = b'*',
    At = b'@',
    Comma = b',',
    Colon = b':',
    Semicolon = b';',
    OpenParenthesis = b'(',
    CloseParenthesis = b')',
    OpenSquare = b'[',
    CloseSquare = b']',
    Dollar = b'$',
    Tilde = b'~',
    Caret = b'^',
    Plus = b'+',
    Equals = b'=',
    Pipe = b'|',
    Greater = b'>',
    Space = b' ',
    SingleQuote = b'\'',
    DoubleQuote = b'"',
    Slash = b'/',
    Bang = b'!',

    Backslash = b'\\',
    Cr = b'\r',
    Feed = 0x0C, // \f
    Newline = b'\n',
    Tab = b'\t',

    Comment = u8::MAX,
    Word = u8::MAX - 1,
    Combinator = u8::MAX - 2,
}

impl From<u8> for TokenType {
    fn from(ch: u8) -> Self {
        assert!(is_char_valid_token(ch));
        // SAFETY: Always valid 'cause we assert it above
        unsafe { mem::transmute(ch) }
    }
}

impl From<TokenType> for u8 {
    fn from(ty: TokenType) -> Self {
        // SAFETY: Always valid 'cause it's memory repr is u8
        unsafe { mem::transmute(ty) }
    }
}

const fn is_char_valid_token(ch: u8) -> bool {
    let comment = u8::MAX;
    let word = u8::MAX - 1;
    let combinator = u8::MAX - 2;
    matches!(
        ch,
        b'&' | b'*'
            | b'@'
            | b','
            | b':'
            | b';'
            | b'('
            | b')'
            | b'['
            | b']'
            | b'$'
            | b'~'
            | b'^'
            | b'+'
            | b'='
            | b'|'
            | b'>'
            | b' '
            | b'\''
            | b'"'
            | b'/'
            | b'!'
            | b'\\'
            | b'\r'
            | 0x0C // \f
            | b'\n'
            | b'\t',
    )
        // TODO(CGQAQ): Maybe not return true?
        || ch == comment
        || ch == word
        || ch == combinator
}

///
/// Check if a char is not allowed to escape
/// * return true if it's not allowed to escape.
///
#[inline]
pub(crate) fn is_unescapable(ch: u8) -> bool {
    ch == TokenType::Tab.into()
        || ch == TokenType::Newline.into()
        || ch == TokenType::Cr.into()
        || ch == TokenType::Tab.into()
        || ch == TokenType::Newline.into()
        || ch == TokenType::Cr.into()
}

#[inline]
pub(crate) fn is_word_delimiter(t: u8) -> bool {
    is_whitespace(t)
        || t == TokenType::Ampersand.into()
        || t == TokenType::Asterisk.into()
        || t == TokenType::Bang.into()
        || t == TokenType::Comma.into()
        || t == TokenType::Colon.into()
        || t == TokenType::Semicolon.into()
        || t == TokenType::OpenParenthesis.into()
        || t == TokenType::CloseParenthesis.into()
        || t == TokenType::OpenSquare.into()
        || t == TokenType::CloseSquare.into()
        || t == TokenType::SingleQuote.into()
        || t == TokenType::DoubleQuote.into()
        || t == TokenType::Plus.into()
        || t == TokenType::Pipe.into()
        || t == TokenType::Tilde.into()
        || t == TokenType::Greater.into()
        || t == TokenType::Equals.into()
        || t == TokenType::Dollar.into()
        || t == TokenType::Caret.into()
        || t == TokenType::Slash.into()
    // others all considered not word delimiters.
}

#[inline]
pub(crate) const fn is_hex(ch: u8) -> bool {
    match ch {
        b'0'..=b'9' => true,
        b'a'..=b'f' => true,
        b'A'..=b'F' => true,
        _ => false,
    }
}

#[inline]
pub(crate) fn is_whitespace(ch: u8) -> bool {
    ch == TokenType::Space.into()
        || ch == TokenType::Tab.into()
        || ch == TokenType::Newline.into()
        || ch == TokenType::Cr.into()
        || ch == TokenType::Feed.into()
}

#[inline(always)]
pub(crate) const fn is_quote(ch: u8) -> bool {
    match ch {
        b'\'' | b'"' => true,
        _ => false,
    }
}

///
/// Returns the last index of the escape seq.
///
fn consume_escape(css: &[u8], start: usize) -> usize {
    let mut next = start;
    let char_codes = css;
    assert!(char_codes.len() > next + 1);
    let mut code = char_codes[next + 1];
    if is_unescapable(code.try_into().unwrap()) {
        // just consume the escape char
    } else if is_hex(code) {
        let mut hex_digits = 0;
        // consume up to 6 hex chars
        loop {
            next += 1;
            hex_digits += 1;
            code = char_codes[next + 1];
            if !(is_hex(code) && hex_digits < 6) {
                break;
            }
        }

        // if fewer than 6 hex chars, a trailing space ends the escape
        if hex_digits < 6 && code == TokenType::Space.into() {
            next += 1;
        }
    } else {
        // the next char is part of the current word.
        next += 1;
    }

    next
}

///
/// Returns the last index of the bar css word.
///
fn consume_word(css: &[u8], start: usize) -> usize {
    let mut next = start;
    let char_codes = css;

    loop {
        let code = char_codes[next];
        if is_word_delimiter(code) {
            return next - 1;
        } else if code == TokenType::Backslash.into() {
            next = consume_escape(css, next) + 1;
        } else {
            // All other characters are part of the word
            next += 1;
        }
        if next >= char_codes.len() {
            break;
        }
    }

    next - 1
}

pub(crate) type Range = (usize, usize);

pub struct Token {
    pub ty: TokenType,
    pub line: Range,
    pub col: Range,
    pub pos: Range,
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "[{:?}, ({:?}), ({:?}), ({:?})]",
                self.ty, self.line, self.col, self.pos,
            )
            .as_str(),
        )
    }
}

pub struct Tokenizer<'a> {
    input: &'a str,
    css: &'a [u8],

    safe: bool,

    line: usize,
    start: usize,
    offset: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str, safe: Option<bool>) -> Self {
        Tokenizer {
            input,
            css: input.as_bytes(),
            safe: safe.unwrap_or(false),

            line: 0,
            offset: 0,
            start: 0,
        }
    }

    #[inline(always)]
    pub(crate) fn unclosed(&mut self, what: &str, fix: &str) {
        if self.safe {
            // fyi: this is never set to true.
            // css += fix;
            // next = css.length - 1;
            // TODO(CGQAQ): Maybe some pretty print? or better error handling.
            unreachable!("Tokenizer safe option should never set to true.")
        } else {
            panic!(
                "Unclosed {}. {} {} {}",
                what,
                self.line,
                self.start - self.offset,
                self.start
            );
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let length = self.css.len();
        let mut end = 0;

        let mut code;
        let mut end_column = 0;
        let mut end_line = 0;
        let mut escaped;
        let mut escape_pos;
        let mut last;
        let mut next = 0;
        let mut next_line;
        let mut next_offset: Option<usize> = None;
        let mut token_type = TokenType::At;

        while self.start < length {
            code = self.css[self.start];

            if code == TokenType::Newline.into() {
                self.offset = self.start;
                self.line += 1;
            }

            match code {
                _ if is_whitespace(code) => {
                    next = self.start;

                    loop {
                        next += 1;
                        code = self.css[next];
                        if code == TokenType::Newline.into() {
                            self.offset = next;
                            self.line += 1;
                        }
                        if !is_whitespace(code) {
                            break;
                        }
                    }

                    token_type = TokenType::Space;
                    end_line = self.line;
                    end_column = next - self.offset - 1;
                    end = next;
                }

                ch @ _
                    if ch == TokenType::Plus.into()
                        || ch == TokenType::Greater.into()
                        || ch == TokenType::Tilde.into()
                        || ch == TokenType::Pipe.into() =>
                {
                    next = self.start;
                    loop {
                        next += 1;
                        code = self.css[next];

                        if !(code == TokenType::Plus.into()
                            || code == TokenType::Greater.into()
                            || code == TokenType::Tilde.into()
                            || code == TokenType::Pipe.into())
                        {
                            break;
                        }
                    }

                    token_type = TokenType::Combinator;
                    end_line = self.line;
                    end_column = self.start - self.offset;
                    end = next;
                }
                // Consume these characters as single tokens.
                ch @ _
                    if ch == TokenType::Asterisk.into()
                        || ch == TokenType::Ampersand.into()
                        || ch == TokenType::Bang.into()
                        || ch == TokenType::Comma.into()
                        || ch == TokenType::Equals.into()
                        || ch == TokenType::Dollar.into()
                        || ch == TokenType::Caret.into()
                        || ch == TokenType::OpenSquare.into()
                        || ch == TokenType::CloseSquare.into()
                        || ch == TokenType::Colon.into()
                        || ch == TokenType::Semicolon.into()
                        || ch == TokenType::OpenParenthesis.into()
                        || ch == TokenType::CloseParenthesis.into() =>
                {
                    next = self.start;
                    token_type = code.try_into().unwrap();
                    end_line = self.line;
                    end_column = self.start - self.offset;
                    end = next + 1;
                }

                ch @ _ if is_quote(ch.into()) => {
                    let quote: u8 = code.into();
                    next = self.start;
                    loop {
                        escaped = false;
                        let mut n = 1;
                        while next + n < self.css.len() && self.css[next + n] != quote {
                            n += 1;
                        }

                        if next + n >= self.css.len() {
                            self.unclosed("quote", format!("{}", quote as char).as_str());
                        }
                        next = next + n;
                        escape_pos = next;
                        while self.css[escape_pos - 1] == TokenType::Backslash.into() {
                            escape_pos -= 1;
                            escaped = !escaped;
                        }

                        if !escaped {
                            break;
                        }

                        token_type = TokenType::SingleQuote;
                        end_line = self.line;
                        end_column = self.start - self.offset;
                        end = next + 1;
                    }
                }
                _ => {
                    if code == TokenType::Slash.into()
                        && self.css[self.start + 1] == TokenType::Asterisk.into()
                    {
                        next =
                            find_str(self.css, self.start + 2, "*/").unwrap_or(usize::MAX - 1) + 1;

                        if next == usize::MAX {
                            self.unclosed("comment", "*/");
                        }

                        // SAFETY: This is always valid 'cause in css comment (chars between /* and */) is always valid utf-8 characters.
                        let content = unsafe {
                            String::from_utf8_unchecked(self.css[self.start..next + 1].to_vec())
                        };
                        let x = content.split('\n');

                        let lines = x.collect::<Vec<&str>>();
                        last = lines.len() - 1;
                        if last > 0 {
                            next_line = self.line + last;
                            next_offset = Some(next - lines[last].len());
                        } else {
                            next_line = self.line;
                            next_offset = Some(self.offset);
                        }
                        drop(lines);

                        token_type = TokenType::Comment;
                        self.line = next_line;
                        end_line = next_line;
                        end_column = next - next_offset.unwrap();
                    } else if code == TokenType::Slash.into() {
                        next = self.start;
                        token_type = code.try_into().unwrap();
                        end_line = self.line;
                        end_column = self.start - self.offset;
                        end = next + 1;
                    } else {
                        next = consume_word(self.css, self.start);
                        token_type = TokenType::Word;
                        end_line = self.line;
                        end_column = next - self.offset;
                    }

                    end = next + 1;
                }
            }

            // Ensure that the token structure remains consistent
            tokens.push(Token {
                ty: token_type,
                line: (self.line, end_line),
                col: (self.start - self.offset, end_column),
                pos: (self.start, end),
            });

            // Reset offset for the next token
            if let Some(o) = next_offset {
                self.offset = o;
                next_offset = None;
            }

            self.start = end;
        }

        tokens
    }
}

fn find_str(h: &[u8], start: usize, x: &str) -> Option<usize> {
    let mut pos = 0;
    let xs = x.as_bytes();

    loop {
        let find = memchr::memchr(xs[0], &h[start + pos..])?;

        let mut matched = true;
        if find + xs.len() < h.len() {
            for i in 1..xs.len() {
                if h[start + pos + find + i] != xs[i] {
                    matched = false;
                    break;
                }
            }

            if matched {
                return Some(pos + find);
            } else {
                pos = find + 1;
            }
        } else {
            return None;
        }
    }
}

#[test]
fn test_find_str() {
    let x = "xxhhwwbb";
    assert_eq!(find_str(x.as_bytes(), 2, "wb"), Some(3))
}
