use super::statement::Statement;

#[derive(Debug)]
pub struct Program<'s>(pub Vec<Statement<'s>>);
