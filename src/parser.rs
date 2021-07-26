use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, space0},
    combinator::{all_consuming, cut, map},
    error::{convert_error, VerboseError},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::typeck::{ty, Type};

#[derive(Debug, Clone)]
pub enum InputTerm {
    TmUnit,
    TmVar(String),
    TmAbs(String, Type, Box<InputTerm>),
    TmApp(Box<InputTerm>, Box<InputTerm>),
}

pub fn parse(text: &str) -> Result<InputTerm, String> {
    use nom::Err::*;
    match all_consuming(parse_term)(text) {
        Ok((_, term)) => Ok(term),
        Err(Error(err) | Failure(err)) => Err(convert_error(text, err)),
        Err(Incomplete(_)) => unreachable!(),
    }
}

type Res<'a, T = InputTerm> = IResult<&'a str, T, VerboseError<&'a str>>;

fn parse_term(text: &str) -> Res {
    map(
        many1(spaced(alt((
            parse_lambda,
            parens(or(parse_term, inp::unit)),
            map(lex_var, inp::var),
        )))),
        |terms| terms.into_iter().fold1(inp::app).unwrap(),
    )(text)
}

fn parse_lambda(text: &str) -> Res {
    map(
        preceded(
            char('\\'),
            cut(tuple((
                spaced(lex_var),
                preceded(char(':'), parse_type),
                preceded(char('.'), parse_term),
            ))),
        ),
        |(param_name, param_type, body)| inp::abs(param_name, param_type, body),
    )(text)
}

fn parse_type(text: &str) -> Res<Type> {
    map(
        separated_list1(tag("->"), spaced(parens(or(parse_type, ty::unit)))),
        |types| rfold1(types.into_iter(), ty::arr).unwrap(),
    )(text)
}

fn lex_var(text: &str) -> Res<String> {
    map(take_while1(char::is_alphabetic), String::from)(text)
}

fn spaced<'a, T>(
    parser: impl FnMut(&'a str) -> Res<'a, T>,
) -> impl FnMut(&'a str) -> Res<'a, T> {
    delimited(space0, parser, space0)
}

fn parens<'a, T>(
    parser: impl FnMut(&'a str) -> Res<'a, T>,
) -> impl FnMut(&'a str) -> Res<'a, T> {
    delimited(char('('), parser, char(')'))
}

fn or<'a, T>(
    parser: impl FnMut(&'a str) -> Res<'a, T>,
    mut default: impl FnMut() -> T,
) -> impl FnMut(&'a str) -> Res<'a, T> {
    alt((parser, move |input| Ok((input, default()))))
}

fn rfold1<T>(
    iter: impl DoubleEndedIterator<Item = T>,
    mut fold: impl FnMut(T, T) -> T,
) -> Option<T> {
    iter.rev().fold1(move |y, x| fold(x, y))
}

pub mod inp {
    use super::InputTerm;
    use crate::typeck::Type;

    pub fn unit() -> InputTerm {
        InputTerm::TmUnit
    }

    pub fn var(name: impl Into<String>) -> InputTerm {
        InputTerm::TmVar(name.into())
    }

    pub fn abs(par: impl Into<String>, ty: Type, body: InputTerm) -> InputTerm {
        InputTerm::TmAbs(par.into(), ty, Box::new(body))
    }

    pub fn app(f: InputTerm, x: InputTerm) -> InputTerm {
        InputTerm::TmApp(Box::new(f), Box::new(x))
    }
}
