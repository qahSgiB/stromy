use rand::distr::Distribution;

use super::LSystemError;



/// Enum of all operators used in mathematical expressions used in l-systems for computation of paramters.
#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Neg, // unary minus
    Add,
    Sub,
    Mul,
    Div,
    Rand, // generates random f32 between two supplied parameters
}

impl Operator {
    /// If the operator is unary returns result of operation wrapped in `Some`.
    /// Otherwise returns `None`.
    pub fn try_apply_1(self, a: f32) -> Option<f32> {
        match self {
            Operator::Neg => Some(-a),
            _ => None,
        }
    }

    /// If the operator is binary returns result of operation wrapped in `Some`.
    /// Otherwise returns `None`
    pub fn try_apply_2(self, a: f32, b: f32) -> Result<Option<f32>, LSystemError> {
        Ok(match self {
            Operator::Add => Some(a + b),
            Operator::Sub => Some(a - b),
            Operator::Mul => Some(a * b),
            Operator::Div => Some(a / b),
            Operator::Rand => {
                // TODO: move this up maybe
                let mut rng = rand::rng();
                let dist = rand::distr::Uniform::new(a, b).map_err(|_| LSystemError::OperatorRandInvalidRange)?;

                Some(dist.sample(&mut rng))
            }
            _ => None,
        })
    }
}