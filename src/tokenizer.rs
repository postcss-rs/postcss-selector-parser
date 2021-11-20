use std::mem;

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

///
/// Returns the last index of the escape seq.
///
fn consume_escape(css: &str, start: usize) -> usize {
    todo!()
}

///
/// Returns the last index of the bar css word.
///
fn consume_word(css: &str, start: usize) -> usize {
    todo!()
}
