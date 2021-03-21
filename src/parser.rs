use nom::IResult;
use nom::{
    branch::alt,
    bytes::complete::{take_while, take_while1},
    character::complete::{char, one_of},
    combinator::{map, opt, recognize},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{terminated, tuple},
};

use crate::expr::{Expr, Term};

pub fn parse(input: String) -> Expr {
    expr(&input).unwrap().1
}

fn expr(input: &str) -> IResult<&str, Expr> {
    let (input, _) = spaces(input)?;
    let (input, code) = separated_list1(tuple((spaces, char('|'), spaces)), term)(input)?;
    let (input, _) = spaces(input)?;
    Ok((input, Expr { code }))
}

fn term(input: &str) -> IResult<&str, Term> {
    let args = separated_list0(char(','), number);
    alt((
        map(
            tuple((identifier, char('('), args, char(')'))),
            |(name, _, args, _)| Term(name, args),
        ),
        map(identifier, |name| Term(name, vec![])),
    ))(input)
}

pub fn spaces(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_whitespace())(input)
}

pub fn identifier(input: &str) -> IResult<&str, String> {
    fn head(c: char) -> bool {
        c.is_alphabetic() || c == '_' || c == '#' || c == '@'
    }
    fn tail(c: char) -> bool {
        c.is_alphanumeric() || head(c)
    }
    let (input, _) = spaces(input)?;
    let (input, s) = take_while1(head)(input)?;
    let (input, t) = take_while(tail)(input)?;
    let (input, _) = spaces(input)?;
    let mut name = String::new();
    name.push_str(s);
    name.push_str(t);
    Ok((input, name))
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

pub fn number(input: &str) -> IResult<&str, f64> {
    let (input, _) = spaces(input)?;
    let (input, x) = map(
        alt((
            recognize(tuple((opt(char('-')), decimal))),
            recognize(tuple((opt(char('-')), char('.'), decimal))),
            recognize(tuple((opt(char('-')), decimal, char('.'), decimal))),
        )),
        |num_str: &str| {
            let num: String = num_str.chars().filter(|&c| c != '_').collect();
            let x: f64 = num.parse().unwrap();
            x
        },
    )(input)?;
    let (input, _) = spaces(input)?;
    Ok((input, x))
}

#[cfg(test)]
mod test_parser {
    use crate::expr::*;
    use crate::parser::*;

    macro_rules! assert_expr {
        ($input:expr, [ $( $code:expr ),* ]) => {
            assert_eq!(expr($input), Ok(("", Expr { code: vec![$($code),*] })));
        };
    }

    #[test]
    fn it_works() {
        assert_expr!("gauss", [Term("gauss".to_string(), vec![])]);
        assert_expr!("gauss()", [Term("gauss".to_string(), vec![])]);
        assert_expr!("gauss(0)", [Term("gauss".to_string(), vec![0.0])]);
        assert_expr!("gauss(0, 1)", [Term("gauss".to_string(), vec![0.0, 1.0])]);
        assert_expr!(
            "seed(42) | gauss(0, 1)",
            [
                Term("seed".to_string(), vec![42.0]),
                Term("gauss".to_string(), vec![0.0, 1.0])
            ]
        );
        assert_expr!(
            "seed(42) | gauss(0, 1) |     int ",
            [
                Term("seed".to_string(), vec![42.0]),
                Term("gauss".to_string(), vec![0.0, 1.0]),
                Term("int".to_string(), vec![])
            ]
        );
    }
}
