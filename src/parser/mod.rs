pub mod error;
pub mod inp;
pub mod ty;

use std::collections::VecDeque;

use itertools::Itertools;

pub use error::*;
pub use inp::*;
pub use ty::*;

use crate::{
    atoms::{Delta, Position, Range},
    multi_result::MultiResult,
};

pub fn parse(text: &str) -> (InputTerm, ParseErrors) {
    let (tokens, mut errs): (Vec<_>, VecDeque<_>) =
        Tokenizer::from(text).partition_result();
    let ParseResult {
        result,
        state: _,
        mut errors,
    } = Parser::from(&tokens[..]).parse();
    errs.append(&mut errors);
    (result, errs.into())
}

type Token<'a> = (TokenData<'a>, Range);

#[derive(Clone, Copy)]
enum TokenData<'a> {
    OpenParen(ParenKind),
    CloseParen(ParenKind),
    Ident(&'a str),
    Colon,
    Arrow(ArrowKind),
    Space(usize),
    Newline,
    Semicolon,
}

#[derive(Clone, Copy)]
enum ParenKind {
    Round,
    Square,
    Curly,
}

#[derive(Clone, Copy)]
enum ArrowKind {
    Thin,
    Fat,
}

struct Tokenizer<'a>(&'a str, Position);

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(s: &'a str) -> Self {
        Self(s, Position::default())
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        use ArrowKind::*;
        use ParenKind::*;
        use TokenData::*;
        let (pref, range) = self.take_while(|c| c == ' ');
        if pref.len() > 0 {
            return Some(Ok((Space(pref.len()), range)));
        }
        let options = [
            ("(", OpenParen(Round)),
            (")", CloseParen(Round)),
            ("[", OpenParen(Square)),
            ("]", CloseParen(Square)),
            ("{", OpenParen(Curly)),
            ("}", CloseParen(Curly)),
            (":", Colon),
            ("->", Arrow(Thin)),
            ("=>", Arrow(Fat)),
            ("\n", Newline),
            (";", Semicolon),
        ];
        for (pref, data) in options {
            let n = pref.len();
            if self.0.get(..n) == Some(pref) {
                let (_, range) = self.commit(n);
                return Some(Ok((data, range)));
            }
        }
        let c = self.0.chars().next()?;
        if c.is_alphabetic() || c == '_' {
            let (pref, range) =
                self.take_while(|c| c.is_alphanumeric() || c == '_');
            Some(Ok((Ident(pref), range)))
        } else {
            let (_, Range { from, until: _ }) = self.commit(c.len_utf8());
            Some(Err(UnknownSymbol(c, from)))
        }
    }
}

impl<'a> Tokenizer<'a> {
    fn commit(&mut self, n: usize) -> (&'a str, Range) {
        let (pref, tail) = self.0.split_at(n);
        let from = self.1;
        let until = Delta::from(pref);
        self.0 = tail;
        self.1 = from + until;
        (pref, Range { from, until })
    }

    fn take_while(&mut self, pred: impl Fn(char) -> bool) -> (&'a str, Range) {
        self.commit(
            self.0
                .chars()
                .take_while(|&c| pred(c))
                .map(|c| c.len_utf8())
                .sum(),
        )
    }
}

#[derive(Clone, Copy)]
struct Parser<'a> {
    tokens: &'a [Token<'a>],
    position: usize,
}

impl<'a> From<&'a [Token<'a>]> for Parser<'a> {
    fn from(tokens: &'a [Token<'a>]) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
}

type ParseResult<T> = MultiResult<T, (), VecDeque<ParseError>>;

impl<'a> Parser<'a> {
    fn parse(&mut self) -> ParseResult<InputTerm> {
        todo!()
    }
}
