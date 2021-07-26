use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, space0},
    combinator::{all_consuming, cut, map, value},
    error::{convert_error, VerboseError},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub enum InputTerm {
    TmUnit,
    TmVar(String),
    TmAbs(String, InputType, Box<InputTerm>),
    TmApp(Box<InputTerm>, Box<InputTerm>),
    TmTyAbs(String, Box<InputTerm>),
    TmTyApp(Box<InputTerm>, InputType),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    TyUnit,
    TyHole,
    TyVar(String),
    TyArrow(Box<InputType>, Box<InputType>),
    TyForall(String, Box<InputType>),
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

enum Either {
    Type(InputType),
    Term(InputTerm),
}

fn parse_term(text: &str) -> Res {
    use Either::*;
    map(
        tuple((
            spaced(parse_app),
            many0(spaced(alt((map(parse_app, Term), map(parse_tyapp, Type))))),
        )),
        |(head, tail)| tail.into_iter().fold(head, inp::gen_app),
    )(text)
}

fn parse_app(text: &str) -> Res {
    alt((
        parse_lambda,
        parse_tyabs,
        parens(or(parse_term, inp::unit)),
        map(lex_var, inp::var),
    ))(text)
}

fn parse_tyapp(text: &str) -> Res<InputType> {
    delimited(char('['), parse_type, char(']'))(text)
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

fn parse_tyabs(text: &str) -> Res {
    map(
        preceded(
            tag(r"/\"),
            cut(tuple((spaced(lex_var), preceded(char('.'), parse_term)))),
        ),
        |(param_name, body)| inp::tyabs(param_name, body),
    )(text)
}

fn parse_type(text: &str) -> Res<InputType> {
    use inp::ty;
    map(
        separated_list1(
            tag("->"),
            spaced(alt((
                parse_forall,
                parens(or(parse_type, ty::unit)),
                map(lex_var, ty::var),
                value(ty::hole(), char('_')),
            ))),
        ),
        |types| rfold1(types.into_iter(), ty::arr).unwrap(),
    )(text)
}

fn parse_forall(text: &str) -> Res<InputType> {
    map(
        preceded(
            tag(r"/\"),
            cut(tuple((spaced(lex_var), preceded(char('.'), parse_type)))),
        ),
        |(param_name, body_type)| inp::ty::forall(param_name, body_type),
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
    use super::{Either, InputTerm, InputType};

    pub fn unit() -> InputTerm {
        InputTerm::TmUnit
    }

    pub fn var(name: impl Into<String>) -> InputTerm {
        InputTerm::TmVar(name.into())
    }

    pub fn abs(
        par: impl Into<String>,
        ty: InputType,
        body: InputTerm,
    ) -> InputTerm {
        InputTerm::TmAbs(par.into(), ty, Box::new(body))
    }

    pub fn tyabs(par: impl Into<String>, body: InputTerm) -> InputTerm {
        InputTerm::TmTyAbs(par.into(), Box::new(body))
    }

    pub(super) fn gen_app(f: InputTerm, x: Either) -> InputTerm {
        use Either::*;
        match x {
            Term(x) => app(f, x),
            Type(x) => tyapp(f, x),
        }
    }

    pub fn app(f: InputTerm, x: InputTerm) -> InputTerm {
        InputTerm::TmApp(Box::new(f), Box::new(x))
    }

    pub fn tyapp(f: InputTerm, x: InputType) -> InputTerm {
        InputTerm::TmTyApp(Box::new(f), x)
    }

    pub mod ty {
        use super::InputType;

        pub fn unit() -> InputType {
            InputType::TyUnit
        }

        pub fn hole() -> InputType {
            InputType::TyHole
        }

        pub fn var(name: impl Into<String>) -> InputType {
            InputType::TyVar(name.into())
        }

        pub fn arr(from: InputType, to: InputType) -> InputType {
            InputType::TyArrow(Box::new(from), Box::new(to))
        }

        pub fn forall(par: impl Into<String>, body: InputType) -> InputType {
            InputType::TyForall(par.into(), Box::new(body))
        }
    }
}
