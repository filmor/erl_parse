use erl_pp::{self, MacroDef};
use erl_tokenize::{Lexer, LexicalToken};

use {Error, Tokens};

pub trait Preprocessor {
    fn define_macro(&mut self, name: &str, replacement: Vec<LexicalToken>);
    fn undef_macro(&mut self, name: &str);
}
impl<T, E> Preprocessor for erl_pp::Preprocessor<T, E> {
    fn define_macro(&mut self, name: &str, replacement: Vec<LexicalToken>) {
        self.macros_mut().insert(
            name.to_string(),
            MacroDef::Dynamic(replacement),
        );
    }
    fn undef_macro(&mut self, name: &str) {
        self.macros_mut().remove(name);
    }
}
impl<T> Preprocessor for Lexer<T> {
    fn define_macro(&mut self, _name: &str, _replacement: Vec<LexicalToken>) {}
    fn undef_macro(&mut self, _name: &str) {}
}
impl<T, E> Preprocessor for Tokens<T, E>
where
    T: Iterator<Item = Result<LexicalToken, E>> + Preprocessor,
    Error: From<E>,
{
    fn define_macro(&mut self, name: &str, replacement: Vec<LexicalToken>) {
        self.0.define_macro(name, replacement);
    }
    fn undef_macro(&mut self, name: &str) {
        self.0.undef_macro(name);
    }
}
