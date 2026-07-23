// Using catch-all error types like `Box<dyn Error>` isn't recommended for
// library code where callers might want to make decisions based on the error
// content instead of printing it out or propagating it further. Here, we define
// a custom error type to make it possible for callers to decide what to do next
// when our function returns an error.

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum CreationError {
    Negative,
    Zero
}

#[derive(Debug, PartialEq)]
enum ParsePosNonzeroError {
    ParseInt(ParseIntError),
    Creation(CreationError)
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(num: i64) -> Result<Self, CreationError> {
        if num < 0 {
            Err(CreationError::Negative)
        } else if num == 0 {
            Err(CreationError::Zero)
        } else {
            Ok(Self(num as u64))
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        let some_parsed_number = s.parse::<i64>().map_err(ParsePosNonzeroError::ParseInt)?;
        Self::new(some_parsed_number).map_err(ParsePosNonzeroError::Creation)
    }
}

fn main() {

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}
