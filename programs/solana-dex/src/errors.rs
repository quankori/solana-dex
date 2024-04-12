use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("User insufficient funds")]
    InsufficientFunds,
    #[msg("Calculation overflow.")]
    Overflow,
    #[msg("Division error, possibly division by zero.")]
    DivisionError,
    #[msg("Calculation underflow.")]
    Underflow,
}
