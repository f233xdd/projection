use std::{error, fmt};

#[derive(Debug)]
pub struct SyntaxError();

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "SyntaxError")
    }
}
impl error::Error for SyntaxError {}

#[derive(Debug)]
pub struct NamespaceNotSameError();

impl fmt::Display for NamespaceNotSameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "NamespaceNotSameError")
    }
}
impl error::Error for NamespaceNotSameError {}
