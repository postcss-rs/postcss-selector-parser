#![allow(unused_variables, dead_code)]

use std::{mem, vec};

#[derive(Copy, Clone)]
#[repr(u8)]
pub(crate) enum TokenType {
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
    match ch {
        _ if ch == b'&' => true,
        _ if ch == b'*' => true,
        _ if ch == b'@' => true,
        _ if ch == b',' => true,
        _ if ch == b':' => true,
        _ if ch == b';' => true,
        _ if ch == b'(' => true,
        _ if ch == b')' => true,
        _ if ch == b'[' => true,
        _ if ch == b']' => true,
        _ if ch == b'$' => true,
        _ if ch == b'~' => true,
        _ if ch == b'^' => true,
        _ if ch == b'+' => true,
        _ if ch == b'=' => true,
        _ if ch == b'|' => true,
        _ if ch == b'>' => true,
        _ if ch == b' ' => true,
        _ if ch == b'\'' => true,
        _ if ch == b'"' => true,
        _ if ch == b'/' => true,
        _ if ch == b'!' => true,

        _ if ch == b'\\' => true,
        _ if ch == b'\r' => true,
        _ if ch == 0x0C => true, // \f
        _ if ch == b'\n' => true,
        _ if ch == b'\t' => true,

        // TODO(CGQAQ): Maybe not return true?
        _ if ch == u8::MAX => true,
        _ if ch == u8::MAX - 1 => true,
        _ if ch == u8::MAX - 2 => true,

        _ => false,
    }
}

///
/// Check if a char is not allowed to escape
/// * return true if it's not allowed to escape.
///
#[inline(always)]
pub(crate) const fn is_unescapable(t: TokenType) -> bool {
    match t {
        TokenType::Tab => true,
        TokenType::Newline => true,
        TokenType::Cr => true,
        TokenType::Feed => true,
        _ => false,
    }
}

#[inline(always)]
pub(crate) const fn is_word_delimiter(t: TokenType) -> bool {
    match t {
        // whitespaces
        TokenType::Space => true,
        TokenType::Tab => true,
        TokenType::Newline => true,
        TokenType::Cr => true,
        TokenType::Feed => true,

        // non-whitespaces
        TokenType::Ampersand => true,
        TokenType::Asterisk => true,
        TokenType::Bang => true,
        TokenType::Comma => true,
        TokenType::Colon => true,
        TokenType::Semicolon => true,
        TokenType::OpenParenthesis => true,
        TokenType::CloseParenthesis => true,
        TokenType::OpenSquare => true,
        TokenType::CloseSquare => true,
        TokenType::SingleQuote => true,
        TokenType::DoubleQuote => true,
        TokenType::Plus => true,
        TokenType::Pipe => true,
        TokenType::Tilde => true,
        TokenType::Greater => true,
        TokenType::Equals => true,
        TokenType::Dollar => true,
        TokenType::Caret => true,
        TokenType::Slash => true,

        // others all considered not word delimiters.
        _ => false,
    }
}

#[inline(always)]
pub(crate) const fn is_hex(ch: u8) -> bool {
    match ch {
        b'0'..=b'9' => true,
        b'a'..=b'f' => true,
        b'A'..=b'F' => true,
        _ => false,
    }
}

#[inline(always)]
pub(crate) fn is_whitespace(ch: u8) -> bool {
    match ch.into() {
        TokenType::Space => true,
        TokenType::Tab => true,
        TokenType::Newline => true,
        TokenType::Cr => true,
        TokenType::Feed => true,
        _ => false,
    }
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
    if is_unescapable(code.into()) {
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
        if is_word_delimiter(code.into()) {
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

struct Token {
    r#type: TokenType,
    line: Range,
    col: Range,
    pos: Range,
}

pub(crate) struct Tokenizer<'a> {
    input: &'a str,
    css: &'a [u8],

    safe: bool,

    line: usize,
    start: usize,
    offset: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str, safe: Option<bool>) -> Self {
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
    fn unclosed(&mut self, what: &str, fix: &str) {
        if self.safe {
            // fyi: this is never set to true.
            // css += fix;
            // next = css.length - 1;
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

    pub(crate) fn tokenize(&mut self) {
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

            match <TokenType>::from(code) {
                _ if is_whitespace(code.into()) => {
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

                TokenType::Plus | TokenType::Greater | TokenType::Tilde | TokenType::Pipe => {
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
                TokenType::Asterisk
                | TokenType::Ampersand
                | TokenType::Bang
                | TokenType::Comma
                | TokenType::Equals
                | TokenType::Dollar
                | TokenType::Caret
                | TokenType::OpenSquare
                | TokenType::CloseSquare
                | TokenType::Colon
                | TokenType::Semicolon
                | TokenType::OpenParenthesis
                | TokenType::CloseParenthesis => {
                    next = self.start;
                    token_type = code.into();
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
                        token_type = code.into();
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
                r#type: token_type,
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
