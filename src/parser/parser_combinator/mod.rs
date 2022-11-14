mod basic_parser;
mod boxed_parser;
mod combinator;
mod traits;

pub(crate) use {basic_parser::single_token, combinator::*, traits::Parser};
