use crate::{Equation, Expr};
use nom::{
  branch::alt,
  character::complete::{char, digit1, space0},
  combinator::{map, map_res, opt, recognize},
  multi::many0,
  sequence::{delimited, preceded, terminated, tuple},
  IResult,
};

/// Parse an [Equation] from a string slice.
pub fn parse_equation(input: &str) -> IResult<&str, Equation> {
  let mut parser = tuple((parse_expression, rhs));
  let (rest, (expression, expected)) = parser(input)?;
  Ok((
    rest,
    Equation {
      expected,
      expression,
    },
  ))
}

/// Parse an optionally signed number.
fn parse_number(input: &str) -> IResult<&str, f32> {
  let fraction_parse = recognize(tuple((digit1, char('.'), digit1)));
  let num_parse = delimited(
    space0,
    tuple((opt(char('-')), alt((fraction_parse, digit1)))),
    space0,
  );
  let mut parser = map_res(num_parse, |(neg, n): (Option<_>, &str)| {
    n.parse::<f32>()
      .map(|num| if neg.is_some() { -num } else { num })
  });
  parser(input)
}

/// Parse the right hand side of the equation.
fn rhs(input: &str) -> IResult<&str, f32> {
  preceded(terminated(char('='), space0), parse_number)(input)
}

/// Parse an expression within parenthesis.
fn parse_parens(input: &str) -> IResult<&str, Expr> {
  delimited(
    space0,
    delimited(char('('), parse_expression, char(')')),
    space0,
  )(input)
}

/// Parse a single term or an expression within parenthesis.
fn parse_operation(input: &str) -> IResult<&str, Expr> {
  alt((parse_parens, map(parse_number, Expr::Num)))(input)
}

/// Take two expressions [Expr] with an infix operator and return an [Expr]
/// operation variant.
fn parse_op((op, expr2): (char, Expr), expr1: Expr) -> Expr {
  use crate::Expr::*;
  match op {
    '+' => Add(Box::new(expr1), Box::new(expr2)),
    '-' => Subtract(Box::new(expr1), Box::new(expr2)),
    '*' => Multiply(Box::new(expr1), Box::new(expr2)),
    '/' => Divide(Box::new(expr1), Box::new(expr2)),
    '^' => Exponent(Box::new(expr1), Box::new(expr2)),
    _ => unreachable!(),
  }
}

/// Build a single recursive [Expr] from a list of individual [Expr] expressions.
fn combine_exprs(expr: Expr, exprs: Vec<(char, Expr)>) -> Expr {
  exprs.into_iter().fold(expr, |acc, val| parse_op(val, acc))
}

/// Parse expressions with factor/power of.
fn parse_factor(input: &str) -> IResult<&str, Expr> {
  let (input, num) = parse_operation(input)?;
  let (input, exprs) = many0(tuple((char('^'), parse_factor)))(input)?;
  Ok((input, combine_exprs(num, exprs)))
}

/// Parse factor then division / multiplication.
fn parse_term(input: &str) -> IResult<&str, Expr> {
  let (input, num) = parse_factor(input)?;
  let (input, exprs) =
    many0(tuple((alt((char('/'), char('*'))), parse_factor)))(input)?;
  Ok((input, combine_exprs(num, exprs)))
}

/// Parse factor then division / multiplication then addition subtraction expressions.
fn parse_expression(input: &str) -> IResult<&str, Expr> {
  let (input, num) = parse_term(input)?;
  let (input, exprs) =
    many0(tuple((alt((char('+'), char('-'))), parse_term)))(input)?;
  Ok((input, combine_exprs(num, exprs)))
}

#[cfg(test)]
mod test {
  use super::*;

  /// Test success by passing expected parsed value otherwise test failure.
  macro_rules! test_parse_number {
    ($input:literal, $expected:expr) => {
      let (_, n) = parse_number($input).unwrap();
      assert_eq!(n, $expected);
    };
    ($input:literal) => {
      let result = parse_number($input);
      assert!(result.is_err());
    };
  }

  #[test]
  fn test_number() {
    test_parse_number!("10.5", 10.5);
    test_parse_number!("3.141592653589793", std::f32::consts::PI);
    test_parse_number!(" 12345 --", 12345.);
    test_parse_number!("12345 and", 12345.);
    test_parse_number!(" 12345blah", 12345.);
    test_parse_number!("-20.5-3", -20.5);
    test_parse_number!("-10", -10.);
  }

  #[test]
  fn test_parse_fail() {
    test_parse_number!("abc10.5");
    test_parse_number!("+10");
  }
}
