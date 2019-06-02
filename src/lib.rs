//! The RuSh library crate.

extern crate chrono;
extern crate pest;
extern crate rand;

/// Include arrays management.
pub mod arrays;
/// Include options management (shopt, set)
pub mod opt;
/// Include prompt management.
pub mod prompt;
/// Include rush core.
pub mod rush;
/// Include variables management.
pub mod variables;

pub use self::rush::RuSh;
