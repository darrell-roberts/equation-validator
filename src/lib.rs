use std::str::FromStr;

use nom::{
  bytes::complete::tag,
  character::complete::{one_of, space0},
  combinator::{map, map_res, recognize},
  error::context,
  multi::many1,
  sequence::{preceded, tuple},
  IResult,
};

pub enum Operator {
  Add,
  Subtract,
  Multiply,
  Divide,
}

pub struct Equation {
  pub expected: i64,
  pub term1: i64,
  pub term2: i64,
  pub operator: Operator,
}

impl Equation {
  pub fn eval(&self) -> i64 {
    match self.operator {
      Operator::Add => self.term1 + self.term2,
      Operator::Subtract => self.term1 - self.term2,
      Operator::Multiply => self.term1 * self.term2,
      Operator::Divide => self.term1 / self.term2,
    }
  }

  pub fn is_correct(&self) -> bool {
    self.eval() == self.expected
  }
}

#[derive(Debug)]
pub struct InvalidEquation;

impl FromStr for Equation {
  type Err = InvalidEquation;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match parse_equation(s) {
      Ok((_, p)) => Ok(p),
      Err(_) => Err(InvalidEquation),
    }
  }
}

fn decimal(input: &str) -> IResult<&str, i64> {
  context(
    "decimal",
    map_res(
      preceded(space0, recognize(many1(one_of("-0123456789")))),
      |i: &str| i.parse::<i64>(),
    ),
  )(input)
}

fn operator(input: &str) -> IResult<&str, Operator> {
  context(
    "operator",
    map(preceded(space0, one_of("+-*/")), |i| match i {
      '+' => Operator::Add,
      '-' => Operator::Subtract,
      '*' => Operator::Multiply,
      '/' => Operator::Divide,
      _ => unreachable!(),
    }),
  )(input)
}

fn rhs(input: &str) -> IResult<&str, i64> {
  context("rhs", preceded(preceded(space0, tag("=")), decimal))(input)
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
  let mut parser = tuple((decimal, operator, decimal, rhs));
  let (rest, (d1, op, d2, expected)) = parser(input)?;
  Ok((
    rest,
    Equation {
      expected,
      term1: d1,
      term2: d2,
      operator: op,
    },
  ))
}

#[cfg(test)]
mod test {
  use crate::*;

  fn test_parser(input: &str, correct: bool) {
    let equation = input.parse::<Equation>().unwrap();
    assert_eq!(equation.is_correct(), correct);
  }

  #[test]
  fn test_parse() {
    test_parser("5 + -2 = 3", true);
    test_parser("10 + 10 = 20", true);
    test_parser("10+10=20", true);
    test_parser("5*5= 25", true);
    test_parser("100 / 10 = 10", true);
    test_parser("-25 + -25 = -50", true);
    test_parser("-1 + 2 = 1", true);
    test_parser("1 + 1 = 0", false);
  }
}
