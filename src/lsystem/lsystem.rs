use super::LSystemError;
use super::geometry::{Action, Segment, lsystem_string_to_segments};
use super::operator::Operator;
use super::polski_stos::PolskiStos;



pub mod loader;
pub mod samples;



#[derive(Debug, Clone, Copy)]
enum RuleToken {
    Symbol(usize),
    Parameter(usize),
    Constant(usize),
    Operator(Operator),
}

#[derive(Debug, Clone, Copy)]
pub enum StringToken {
    Symbol(usize),
    Value(f32),
}



pub struct LSystem {
    axiom: Vec<StringToken>,
    rules: Vec<Vec<RuleToken>>,
    consts: Vec<f32>,
    actions: Vec<Option<Action>>, // TODO: `Option` is good idea ?
}

impl LSystem {
    fn expand_once(&self, string: &[StringToken]) -> Result<Vec<StringToken>, LSystemError> {
        let mut new_string = Vec::new();

        let mut parameters = Vec::new();

        // let mut string_index = 0;
        let mut string_iter = string.iter().copied();

        let mut next_string_symbol = string_iter_take_symbol(&mut string_iter)?;

        while let Some(string_symbol) = next_string_symbol {
            // check if there is rule for the symbol in our grammar
            let rule = match self.rules.get(string_symbol) {
                Some(rule) => rule,
                None => {
                    // no rule, just copy
                    new_string.push(StringToken::Symbol(string_symbol));
                    next_string_symbol = string_iter_march_to_next_symbol(&mut string_iter, |value| { new_string.push(StringToken::Value(value)); });
                    continue;
                },
            };

            // collect params
            parameters.clear();

            next_string_symbol = string_iter_march_to_next_symbol(&mut string_iter, |value| { parameters.push(value); });

            let mut rule_iter = rule.iter();
            
            // first token of the rule must be symbol
            match rule_iter.next() {
                Some(&RuleToken::Symbol(symbol)) => {
                    new_string.push(StringToken::Symbol(symbol));
                },
                Some(_) | None => { return Err(LSystemError::RuleSymbolExpectedAtBeginning) },
            }

            // process the rule
            let mut polish_stack = PolskiStos::new();

            for rule_token in rule_iter {
                match *rule_token {
                    RuleToken::Symbol(symbol) => {
                        let stack_iter = polish_stack.dokoncz().map_err(|_| LSystemError::RuleInvalidExpression)?;
                        new_string.extend(stack_iter.map(StringToken::Value));
                        polish_stack.pusty();

                        new_string.push(StringToken::Symbol(symbol));
                    },
                    RuleToken::Operator(operator) => {
                        polish_stack.dodaj_operator(operator);
                    },
                    RuleToken::Constant(constant) => {
                        let constant_value = *self.consts.get(constant).ok_or(LSystemError::ConstantNotFound { constant })?;
                        polish_stack.dodaj_wartosc(constant_value)?;
                    },
                    RuleToken::Parameter(paramter) => {
                        let parameter_value = *parameters.get(paramter).ok_or(LSystemError::StringNotEnoughParameters { paramter })?;
                        polish_stack.dodaj_wartosc(parameter_value)?;
                    },
                }
            }

            let stack_iter = polish_stack.dokoncz().map_err(|_| LSystemError::RuleInvalidExpression)?;
            new_string.extend(stack_iter.map(|value| StringToken::Value(value)));
        }

        Ok(new_string)
    }

    pub fn expand(&self, iter_count: usize) -> Result<Vec<StringToken>, LSystemError> {
        let mut string = self.axiom.clone();

        for _ in 0..iter_count {
            string = self.expand_once(&string)?;
        }

        Ok(string)
    }

    pub fn expand_to_geometry(&self, iter_count: usize) -> Result<Vec<Segment>, LSystemError> {
        let string = self.expand(iter_count)?;
        lsystem_string_to_segments(&string, &self.actions)
    }
}


pub(in super) fn string_iter_march_to_next_symbol(string_iter: &mut impl Iterator<Item = StringToken>, mut f: impl FnMut(f32)) -> Option<usize> {
    loop {
        match string_iter.next() {
            Some(StringToken::Value(value)) => { f(value); },
            Some(StringToken::Symbol(symbol)) => { return Some(symbol); },
            None => { return None; },
        }
    };
}

pub(in super) fn string_iter_take_symbol(string_iter: &mut impl Iterator<Item = StringToken>) -> Result<Option<usize>, LSystemError> {
    match string_iter.next() {
        Some(StringToken::Value(_)) => Err(LSystemError::StringSymbolExpected),
        Some(StringToken::Symbol(symbol)) => Ok(Some(symbol)),
        None => Ok(None),
    }
}