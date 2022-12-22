# Simple equation parser and equation validator.

I made this to learn [nom](https://docs.rs/nom/latest/nom/), the Rust parser combinators library inspired by Haskell's
[parsec](https://hackage.haskell.org/package/parsec).

This library introduces an Equation type with an Expression and expected result. The
Equation and expressions are parsed from a string slice using nom parser combinators. This
allows for a lot of flexibility in the input format. For example from the unit tests.

```rust
  fn test_parser(input: &str, correct: bool) {
    let equation = input.parse::<Equation>().unwrap();
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
```

Every one of these equations can be correctly parsed and validated.

This example expands on the [Basic Calculator](https://github.com/Geal/nom#parsers-written-with-nom) example.