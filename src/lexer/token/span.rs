//! The span of a token.

/// The span of a token
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    /// The start of a token in text.
    pub from: usize,
    /// The end of a token in text.
    pub to: usize,
}
