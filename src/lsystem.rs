pub mod geometry;
pub mod lsystem;
mod operator;
mod polski_stos;



#[derive(Debug, Clone, Copy)]
pub enum LSystemError {
    OperatorRandInvalidRange,
    StringSymbolExpected,
    RuleSymbolExpectedAtBeginning,
    RuleInvalidExpression,
    ConstantNotFound { constant: usize },
    StringNotEnoughParameters { paramter: usize },
    TooMuchPop,
    ActionParameterExpected,
}