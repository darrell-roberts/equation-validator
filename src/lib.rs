use parser::parse_equation;
use std::str::FromStr;

mod parser;

/// A recursive Expression.
#[derive(Debug)]
enum Expr {
  Num(f32),
  Add(Box<Expr>, Box<Expr>),
  Subtract(Box<Expr>, Box<Expr>),
  Multiply(Box<Expr>, Box<Expr>),
  Divide(Box<Expr>, Box<Expr>),
  Exponent(Box<Expr>, Box<Expr>),
}

impl std::fmt::Display for Expr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use crate::Expr::*;
    match self {
      Num(num) => write!(f, "{num}"),
      Add(expr1, expr2) => write!(f, "{expr1} + {expr2}"),
      Subtract(expr1, expr2) => write!(f, "{expr1} - {expr2}"),
      Multiply(expr1, expr2) => write!(f, "{expr1} * {expr2}"),
      Divide(expr1, expr2) => write!(f, "{expr1} / {expr2}"),
      Exponent(expr1, expr2) => write!(f, "{expr1}^{expr2}"),
    }
  }
}

/// An equation with LHS equation and RHS result.
#[derive(Debug)]
pub struct Equation {
  expected: f32,
  expression: Expr,
}

impl std::fmt::Display for Equation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{} = {} {}",
      self.expression,
      self.expected,
      if self.is_correct() { "✔" } else { "❌" }
    )
  }
}

/// Evaluate the expression for this equation.
fn evaluate(expr: &Expr) -> f32 {
  use crate::Expr::*;
  match expr {
    Num(num) => *num,
    Add(expr1, expr2) => evaluate(expr1) + evaluate(expr2),
    Subtract(expr1, expr2) => evaluate(expr1) - evaluate(expr2),
    Multiply(expr1, expr2) => evaluate(expr1) * evaluate(expr2),
    Divide(expr1, expr2) => evaluate(expr1) / evaluate(expr2),
    Exponent(expr1, expr2) => evaluate(expr1).powf(evaluate(expr2)),
  }
}

impl Equation {
  /// Evaluate the expression for this equation.
  pub fn eval(&self) -> f32 {
    evaluate(&self.expression)
  }

  /// Evaluate the expression for this equation and check if it matches RHS expectation.
  pub fn is_correct(&self) -> bool {
    self.eval() == self.expected
  }
}

/// Parsing error for equation text.
#[derive(Debug)]
pub struct InvalidEquation(String);

impl FromStr for Equation {
  type Err = InvalidEquation;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match parse_equation(s) {
      Ok((_, p)) => Ok(p),
      Err(e) => Err(InvalidEquation(e.to_string())),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::*;

  fn test_parser(input: &str, correct: bool) {
    let equation = input.parse::<Equation>().unwrap();
    println!("{equation}");
    assert_eq!(equation.is_correct(), correct);
  }

  #[test]
  fn test_parse() {
    test_parser("5 + -2 = 3", true);
    test_parser("10 + 10 = 20", true);
    test_parser("10+10=20", true);
    test_parser("10+10-10=10", true);
    test_parser("5*5= 25", true);
    test_parser("100 / 10 = 10", true);
    test_parser("5 * 5 = 25", true);
    test_parser("-25 + -25 = -50", true);
    test_parser("-1 + 2 = 1", true);
    test_parser("1 + 1 = 0", false);
    test_parser("(1 + 1) * 5 = 10", true);
    test_parser("1 + 1 * 5 = 6", true);
    test_parser("5^2 * 2 = 50", true);
    test_parser("5 * 2 + 3 = 13", true);
    test_parser("1.5 + 2.5 = 4.0", true);
    test_parser("   1.5 +  2.5 = 4.0", true);
    test_parser("   1.5 +  2.5 * 5 = 14  ", true);
  }
}
