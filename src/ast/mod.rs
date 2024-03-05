mod expr;
mod statement;
mod program;

pub use program::Program;
pub use statement::*;
pub use expr::*;

pub type Ident<'a> = &'a str;
