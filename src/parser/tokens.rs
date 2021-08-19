use crate::atoms::*;

use super::*;

pub type Token<'a> = (TokenData<'a>, Range);

#[derive(Clone, Copy)]
pub enum TokenData<'a> {
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
pub enum ParenKind {
    Round,
    Square,
    Curly,
}

#[derive(Clone, Copy)]
pub enum ArrowKind {
    Thin,
    Fat,
}

pub struct Tokenizer<'a>(&'a str, Position);

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
        let (pref, range) = self.take_while(|c| c == b' ');
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
        if c.is_ascii_alphabetic() || c == '_' {
            let (pref, range) =
                self.take_while(|c| c.is_ascii_alphanumeric() || c == b'_');
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

    fn take_while(&mut self, pred: impl Fn(u8) -> bool) -> (&'a str, Range) {
        self.commit(
            self.0
                .bytes()
                .take_while(|&c| pred(c))
                .count()
        )
    }
}
