use crate::tokenizer::{Token, TokenType, Tokenizer};
use crate::util;

pub struct Parser<'a> {
    css: &'a str,

    // options
    lossy: bool,
    safe: bool, // always false

    tokens: Vec<Token>,

    position: usize,
}

// TODO: use this instead
#[allow(dead_code)]
pub struct ParserOptions {
    lossy: bool,
    safe: Option<bool>, // always false
}

pub struct Parse {}

pub struct Node {}

impl<'a> Parser<'a> {
    pub fn new(css: &'a str, lossy: bool) -> Parser {
        Parser {
            // input
            css,

            // options
            lossy,
            safe: false,

            // tokens generated from input
            tokens: Tokenizer::new(css, None).tokenize(),
            position: 0,
        }
    }

    pub fn parse(mut self) -> Parse {
        while self.position < self.tokens.len() {
            self.parse_token();
        }

        todo!("constructing Parse");
    }

    fn parse_token(&mut self /*throw on missing parenthesis*/) -> Node {
        let current_token = &self.tokens[self.position];
        match current_token.ty {
            TokenType::Space => self.parse_space(),
            TokenType::Comment => self.parse_comment(),
            TokenType::OpenParenthesis => self.parse_parenthesis(),
            TokenType::CloseParenthesis => {
                // if throw on missing parenthesis; then
                util::expected(vec!["opening parenthesis"], current_token.pos.0, None);
            }
            TokenType::OpenSquare => self.parse_attribute(),

            ty @ _
                if ty == TokenType::Dollar
                    || ty == TokenType::Caret
                    || ty == TokenType::Equals
                    || ty == TokenType::Word =>
            {
                self.parse_word();
            }

            TokenType::Colon => {
                self.parse_pseudo();
            }
            TokenType::Comma => {
                self.parse_comma();
            }
            TokenType::Asterisk => {
                self.parse_universal();
            }
            TokenType::Ampersand => {
                self.parse_ampersand();
            }

            ty @ _ if ty == TokenType::Slash || ty == TokenType::Combinator => {
                self.parse_combinator();
            }
            TokenType::SingleQuote => {
                self.parse_string();
            }
            TokenType::CloseSquare => {}
            TokenType::Semicolon => {}

            // TODO(CGQAQ): Why these will panic?
            TokenType::At => {}
            TokenType::Tilde => {}
            TokenType::Plus => {}
            TokenType::Pipe => {}
            TokenType::Greater => {}
            TokenType::DoubleQuote => {}
            TokenType::Bang => {}
            TokenType::Backslash => {}
            TokenType::Cr => {}
            TokenType::Feed => {}
            TokenType::Newline => {}
            TokenType::Tab => {}
        }

        todo!("Maybe not returning a Node?")
    }

    fn parse_space() {
        todo!("impl parse_space")
    }
    fn parse_comment() {
        todo!("impl parse_comment")
    }
    fn parse_parenthesis() {
        todo!("impl parse_parenthesis")
    }
    fn parse_attribute() {
        todo!("impl parse_attribute")
    }
    fn parse_word() {
        todo!("impl parse_word")
    }
    fn parse_pseudo() {
        todo!("impl parse_pseudo")
    }
    fn parse_comma() {
        todo!("impl parse_comma")
    }
    fn parse_universal() {
        todo!("impl parse_universal")
    }
    fn parse_nesting() {
        todo!("impl parse_nesting")
    }
    fn parse_combinator() {
        todo!("impl parse_combinator")
    }
    fn parse_string() {
        todo!("impl parse_string")
    }
}
