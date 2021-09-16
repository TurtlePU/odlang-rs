use std::{
    borrow::Cow,
    collections::VecDeque,
    convert::{TryFrom, TryInto},
    error::Error,
    fmt::{Debug, Display},
    iter::Peekable,
};

use itertools::Itertools;

use crate::{input::*, prelude::*};

pub fn parse(text: &str) -> Result<InputTerm, ParseErrors> {
    parse_term(build_token_tree(tokenize(text)?)?).try_into()
}

#[derive(Clone, Copy)]
struct Tok<T> {
    data: T,
    range: Range,
}

impl<T> Tok<T> {
    fn map<U>(self, f: impl FnOnce(T) -> U) -> Tok<U> {
        let Tok { data, range } = self;
        Tok {
            data: f(data),
            range,
        }
    }

    fn indent(&self) -> usize {
        self.range.from.column
    }
}

type Token<'a> = Tok<TokenData<'a>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenData<'a> {
    OpenParen(ParenKind, SkipWS, Indent),
    CloseParen(ParenKind),
    Tifier(&'a str, Indent),
    Colon,
    ThinArrow,
    FatArrow,
    Semicolon,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParenKind {
    Paren,
    Bracket,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkipWS {
    DoSkipWS,
    NoSkipWS,
}

impl From<Delta> for SkipWS {
    fn from(delta: Delta) -> Self {
        if delta.nonzero() {
            DoSkipWS
        } else {
            NoSkipWS
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Indent {
    DoIndent,
    NoIndent,
}

impl From<Delta> for Indent {
    fn from(Delta { lines, columns: _ }: Delta) -> Self {
        if lines > 0 {
            DoIndent
        } else {
            NoIndent
        }
    }
}

use Indent::*;
use ParenKind::*;
use SkipWS::*;
use TokenData::*;

impl<'a> Display for TokenData<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OpenParen(Paren, _, _) => "'('",
                CloseParen(Paren) => "')'",
                OpenParen(Bracket, _, _) => "'['",
                CloseParen(Bracket) => "']'",
                Tifier(name, _) => name,
                Colon => "':'",
                ThinArrow => "'->'",
                FatArrow => "'=>'",
                Semicolon => "';'",
            }
        )
    }
}

fn tokenize(text: &str) -> Result<Vec<Token>, ParseErrors> {
    let (tokens, errors): (Vec<_>, VecDeque<_>) =
        Tokenizer::from(text).partition_result();
    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(ParseErrors(errors))
    }
}

struct Tokenizer<'a> {
    stream: &'a str,
    current: Position,
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(stream: &'a str) -> Self {
        Self {
            stream,
            current: Position::default(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let ws = self.take_while(|c| c == b' ' || c == b'\n').range.until;
        let skip_ws = SkipWS::from(ws);
        let indent = Indent::from(ws);
        let options = [
            ("(", OpenParen(Paren, skip_ws, indent)),
            (")", CloseParen(Paren)),
            ("[", OpenParen(Bracket, skip_ws, indent)),
            ("]", CloseParen(Bracket)),
            (":", Colon),
            ("->", ThinArrow),
            ("=>", FatArrow),
            (";", Semicolon),
        ];
        for (pref, data) in options {
            if self.stream.starts_with(pref) {
                return Some(Ok(self.commit(pref.len()).map(|_| data)));
            }
        }
        self.stream.chars().next().map(|c| {
            if c.is_ascii_alphabetic() || c == '_' {
                Ok(self
                    .take_while(|c| c.is_ascii_alphanumeric() || c == b'_')
                    .map(|name| Tifier(name, indent)))
            } else {
                let range = self.commit(c.len_utf8()).range;
                Err(error("Unknown token", range))
            }
        })
    }
}

impl<'a> Tokenizer<'a> {
    fn commit(&mut self, n: usize) -> Tok<&'a str> {
        let (data, tail) = self.stream.split_at(n);
        let from = self.current;
        let until = Delta::from(data);
        self.stream = tail;
        self.current = from + until;
        let range = Range { from, until };
        Tok { data, range }
    }

    fn take_while(&mut self, pred: impl Fn(u8) -> bool) -> Tok<&'a str> {
        self.commit(self.stream.bytes().take_while(|&c| pred(c)).count())
    }
}

#[derive(Clone, Copy)]
struct Operator<'a> {
    at: TokenData<'a>,
    repr: bool,
}

impl<'a> From<TokenData<'a>> for Operator<'a> {
    fn from(at: TokenData<'a>) -> Self {
        let repr = match at {
            OpenParen(_, _, _) | Tifier(_, _) => false,
            _ => true,
        };
        Self { at, repr }
    }
}

impl Operator<'_> {
    fn powers(self) -> (Power, Power) {
        todo!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Power {
    End,
    Begin,
    Colon,
    Space,
    NoSpace,
}

struct TokenTreeRec<'a, T> {
    operator: Operator<'a>,
    operands: Vec<T>,
    range: Range,
}

impl<'a, T> TokenTreeRec<'a, T> {
    fn atom(at: TokenData<'a>, range: Range) -> Self {
        Self {
            operator: at.into(),
            operands: vec![],
            range,
        }
    }
}

struct PreTokenTree<'a>(Result<TokenTreeRec<'a, PreTokenTree<'a>>, ParseError>);

struct TokenTree<'a>(TokenTreeRec<'a, TokenTree<'a>>);

impl<'a> TryFrom<PreTokenTree<'a>> for TokenTree<'a> {
    type Error = ParseErrors;

    fn try_from(tree: PreTokenTree<'a>) -> Result<Self, Self::Error> {
        let TokenTreeRec {
            operator,
            operands,
            range,
        } = tree.0?;
        let (operands, errors): (Vec<_>, Vec<_>) = operands
            .into_iter()
            .map(TokenTree::try_from)
            .partition_result();
        if errors.is_empty() {
            Ok(TokenTree(TokenTreeRec {
                operator,
                operands,
                range,
            }))
        } else {
            Err(errors.into_iter().fold1(Semigroup::app).unwrap())
        }
    }
}

fn build_token_tree(tokens: Vec<Token>) -> Result<TokenTree, ParseErrors> {
    let mut builder = TreeBuilder::from(tokens);
    builder
        .pratt(0, Power::End)
        .map(TokenTree::try_from)
        .unwrap_or(Err(error("empty program", Range::default()).into()))
        .pair(builder.eof().map_err(Into::into))
        .map(|(tree, _)| tree)
}

struct TreeBuilder<'a, I: Iterator<Item = Token<'a>>> {
    stream: Peekable<I>,
}

type TreeResult<'a> = Option<PreTokenTree<'a>>;

impl<'a, I, J> From<J> for TreeBuilder<'a, I>
where
    J: IntoIterator<IntoIter = I>,
    I: Iterator<Item = Token<'a>>,
{
    fn from(iter: J) -> Self {
        Self {
            stream: iter.into_iter().peekable(),
        }
    }
}

impl<'a, I> TreeBuilder<'a, I>
where
    I: Iterator<Item = Token<'a>>,
{
    fn eof(&self) -> Result<(), ParseError> {
        match self.stream.peek() {
            Some(token) => Err(error("Redundant tokens", token.range.from)),
            None => Ok(()),
        }
    }

    fn pratt(&mut self, indent: usize, min_bp: Power) -> TreeResult<'a> {
        let mut lhs = self.word()?;
        while let Some(&token) = self.stream.peek() {
            let operator = Operator::from(token.data);
            let (l_bp, r_bp) = operator.powers();
            if l_bp < min_bp {
                break;
            }
        }
        Some(lhs)
    }

    fn word(&mut self) -> TreeResult<'a> {
        todo!()
    }
}

struct PreInputTerm(
    Result<(InputTermRec<Box<PreInputTerm>, PreInputType>, Range), ParseError>,
);

impl TryFrom<PreInputTerm> for InputTerm {
    type Error = ParseErrors;

    fn try_from(value: PreInputTerm) -> Result<Self, Self::Error> {
        let (tree, range) = value.0?;
        let rec = match tree {
            TmUnit => TmUnit,
            TmVar(var) => TmVar(var),
            TmAbs(var, ty, body) => {
                let (ty, body) = ty.try_into().pair((*body).try_into())?;
                TmAbs(var, ty, Box::new(body))
            }
            TmApp(f, x) => {
                let (f, x) = (*f).try_into().pair((*x).try_into())?;
                TmApp(Box::new(f), Box::new(x))
            }
            TmTyAbs(var, x) => TmTyAbs(var, Box::new((*x).try_into()?)),
            TmTyApp(f, ty) => {
                let (f, ty) = (*f).try_into().pair(ty.try_into())?;
                TmTyApp(Box::new(f), ty)
            }
        };
        Ok(InputTerm(rec, range))
    }
}

struct PreInputType(
    Result<(InputTypeRec<Box<PreInputType>>, Range), ParseError>,
);

impl TryFrom<PreInputType> for InputType {
    type Error = ParseErrors;

    fn try_from(value: PreInputType) -> Result<Self, Self::Error> {
        let (tree, range) = value.0?;
        let rec = match tree {
            TyUnit => TyUnit,
            TyHole => TyHole,
            TyVar(var) => TyVar(var),
            TyArrow(from, to) => {
                let (from, to) = (*from).try_into().pair((*to).try_into())?;
                TyArrow(Box::new(from), Box::new(to))
            }
            TyForall(var, ty) => TyForall(var, Box::new((*ty).try_into()?)),
        };
        Ok(InputType(rec, range))
    }
}

fn parse_term(tree: TokenTree) -> PreInputTerm {
    todo!()
}

#[derive(Default)]
pub struct ParseErrors(VecDeque<ParseError>);

impl Error for ParseErrors {}

impl Debug for ParseErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for ParseErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.0 {
            writeln!(f, "{}", error)?;
        }
        Ok(())
    }
}

impl Semigroup for ParseErrors {
    fn app(self, other: Self) -> Self {
        Self(self.0.app(other.0))
    }
}

impl Singleton<ParseError> for ParseErrors {
    fn single(elem: ParseError) -> Self {
        Self(VecDeque::single(elem))
    }

    fn push(&mut self, elem: ParseError) {
        self.0.push(elem);
    }
}

impl From<ParseError> for ParseErrors {
    fn from(err: ParseError) -> Self {
        Self::single(err)
    }
}

impl Extend<ParseError> for ParseErrors {
    fn extend<T: IntoIterator<Item = ParseError>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

#[derive(Debug)]
pub struct ParseError {
    reason: Cow<'static, str>,
    range: Range,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.range, self.reason)
    }
}

fn error(
    reason: impl Into<Cow<'static, str>>,
    range: impl Into<Range>,
) -> ParseError {
    ParseError {
        reason: reason.into(),
        range: range.into(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn id() {
        let src = include_str!("../examples/id.od");
        let tokens = tokenize(src).unwrap();
        let tree = build_token_tree(tokens).unwrap();
        let _ = InputTerm::try_from(parse_term(tree)).unwrap();
    }
}
