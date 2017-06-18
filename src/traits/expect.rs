use num::BigUint;
use erl_tokenize::tokens::{AtomToken, CharToken, FloatToken, IntegerToken, KeywordToken,
                           StringToken, SymbolToken, VariableToken};
use erl_tokenize::values::{Symbol, Keyword};

use {Result, ErrorKind};

pub trait Expect: Sized {
    type Value: ?Sized;
    fn expect(&self, expected: &Self::Value) -> Result<()>;
}
impl Expect for AtomToken {
    type Value = str;
    fn expect(&self, expected: &Self::Value) -> Result<()> {
        track_assert_eq!(self.value(), expected, ErrorKind::InvalidInput);
        Ok(())
    }
}
impl Expect for CharToken {
    type Value = char;
    fn expect(&self, expected: &Self::Value) -> Result<()> {
        track_assert_eq!(self.value(), *expected, ErrorKind::InvalidInput);
        Ok(())
    }
}
impl Expect for FloatToken {
    type Value = f64;
    fn expect(&self, expected: &Self::Value) -> Result<()> {
        track_assert_eq!(self.value(), *expected, ErrorKind::InvalidInput);
        Ok(())
    }
}
impl Expect for IntegerToken {
    type Value = BigUint;
    fn expect(&self, expected: &Self::Value) -> Result<()> {
        track_assert_eq!(self.value(), expected, ErrorKind::InvalidInput);
        Ok(())
    }
}
impl Expect for KeywordToken {
    type Value = Keyword;
    fn expect(&self, expected: &Self::Value) -> Result<()> {
        track_assert_eq!(self.value(), *expected, ErrorKind::InvalidInput);
        Ok(())
    }
}
impl Expect for StringToken {
    type Value = str;
    fn expect(&self, expected: &Self::Value) -> Result<()> {
        track_assert_eq!(self.value(), expected, ErrorKind::InvalidInput);
        Ok(())
    }
}
impl Expect for SymbolToken {
    type Value = Symbol;
    fn expect(&self, expected: &Self::Value) -> Result<()> {
        track_assert_eq!(self.value(), *expected, ErrorKind::InvalidInput);
        Ok(())
    }
}
impl Expect for VariableToken {
    type Value = str;
    fn expect(&self, expected: &Self::Value) -> Result<()> {
        track_assert_eq!(self.value(), expected, ErrorKind::InvalidInput);
        Ok(())
    }
}
