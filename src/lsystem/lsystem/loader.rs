use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::Peekable;
use std::path::Path;

use super::super::geometry::Action;
use super::super::operator::Operator;
use super::{LSystem, StringToken, RuleToken};



#[derive(Debug, Clone, Copy)]
pub enum LSystemLoaderError {
    ExpectedStringTokenType,
    ExpectedRuleTokenType,
    ExpectedSpace,
    ExpectedNewline,
    UnexpectedEOF,
    ExpectedDigitOrSpace,
    ExpectedDigitOrDot,
    ExpectedOperator,
    ExpectedAction,
}


#[derive(Debug, Clone, Copy)]
enum StringTokenType {
    Symbol,
    Value,
}

#[derive(Debug, Clone, Copy)]
enum RuleTokenType {
    Symbol,
    Parameter,
    Constant,
    Operator,
}

fn peek_newline(src: &mut Peekable<impl Iterator<Item = u8>>) -> bool {
    src.peek().copied() == Some(b'\n')
}

fn parse_space(src: &mut impl Iterator<Item = u8>) -> Result<(), LSystemLoaderError> {
    match src.next() {
        Some(b' ') => Ok(()),
        _ => Err(LSystemLoaderError::ExpectedSpace),
    }
}

fn parse_newline(src: &mut impl Iterator<Item = u8>) -> Result<(), LSystemLoaderError> {
    match src.next() {
        Some(b'\n') => Ok(()),
        _ => Err(LSystemLoaderError::ExpectedNewline),
    }
}

fn parse_string_token_type(src: &mut impl Iterator<Item = u8>) -> Result<Option<StringTokenType>, LSystemLoaderError> {
    let string_token_type = match src.next() {
        Some(b'S') => StringTokenType::Symbol,
        Some(b'V') => StringTokenType::Value,
        Some(b'\n') => { return Ok(None); },
        _ => { return Err(LSystemLoaderError::ExpectedStringTokenType); },
    };

    parse_space(src)?;

    Ok(Some(string_token_type))
}

fn parse_rule_token_type(src: &mut impl Iterator<Item = u8>) -> Result<Option<RuleTokenType>, LSystemLoaderError> {
    let string_token_type = match src.next() {
        Some(b'S') => RuleTokenType::Symbol,
        Some(b'P') => RuleTokenType::Parameter,
        Some(b'C') => RuleTokenType::Constant,
        Some(b'O') => RuleTokenType::Operator,
        Some(b'\n') => { return Ok(None); },
        _ => { return Err(LSystemLoaderError::ExpectedRuleTokenType); },
    };

    parse_space(src)?;

    Ok(Some(string_token_type))
}

fn parse_usize(src: &mut impl Iterator<Item = u8>) -> Result<usize, LSystemLoaderError> {
    let mut value = 0;

    loop {
        match src.next() {
            Some(b' ') => {
                break;
            }
            Some(byte) if byte.is_ascii_digit() => {
                value = value * 10 + ((byte - b'0') as usize);
            }
            _ => { return Err(LSystemLoaderError::ExpectedDigitOrSpace); },
        }
    }

    Ok(value)
}

fn parse_f32(src: &mut impl Iterator<Item = u8>) -> Result<f32, LSystemLoaderError> {
    let mut value = 0.0;

    loop {
        match src.next() {
            Some(b'.') => {
                break;
            }
            Some(byte) if byte.is_ascii_digit() => {
                value = value * 10.0 + ((byte - b'0') as f32);
            }
            _ => { return Err(LSystemLoaderError::ExpectedDigitOrDot); },
        }
    }

    let mut mult = 0.1;

    loop {
        match src.next() {
            Some(b' ') => {
                break;
            }
            Some(byte) if byte.is_ascii_digit() => {
                value += ((byte - b'0') as f32) * mult;
                mult /= 10.0;
            }
            _ => { return Err(LSystemLoaderError::ExpectedDigitOrSpace); },
        }
    }

    Ok(value)
}

fn parse_operator(src: &mut impl Iterator<Item = u8>) -> Result<Operator, LSystemLoaderError> {
    let operator = match src.next() {
        Some(b'!') => Operator::Neg,
        Some(b'+') => Operator::Add,
        Some(b'-') => Operator::Sub,
        Some(b'*') => Operator::Mul,
        Some(b'/') => Operator::Div,
        Some(b'#') => Operator::Rand,
        _ => { return Err(LSystemLoaderError::ExpectedOperator); },
    };

    parse_space(src)?;

    Ok(operator)
}

fn parse_action(src: &mut impl Iterator<Item = u8>) -> Result<Option<Action>, LSystemLoaderError> {
    let action = match src.next() {
        Some(b'[') => Some(Action::Push),
        Some(b']') => Some(Action::Pop),
        Some(b'F') => Some(Action::Forward),
        Some(b'P') => Some(Action::Pitch),
        Some(b'Y') => Some(Action::Yaw),
        Some(b'R') => Some(Action::Roll),
        Some(b'X') => None,
        _ => { return Err(LSystemLoaderError::ExpectedAction); },
    };

    parse_space(src)?;

    Ok(action)
}


pub fn load(src: impl Iterator<Item = u8>) -> Result<LSystem, LSystemLoaderError> {
    let mut src = src.filter(|&byte| byte != 13).peekable();

    /* axiom */
    let mut axiom = Vec::new();

    while let Some(string_token_type) = parse_string_token_type(&mut src)? {
        match string_token_type {
            StringTokenType::Symbol => {
                let symbol = parse_usize(&mut src)?;
                axiom.push(StringToken::Symbol(symbol));
            },
            StringTokenType::Value => {
                let value = parse_f32(&mut src)?;
                axiom.push(StringToken::Value(value));
            },
        }
    };

    parse_newline(&mut src)?;

    /* rules */
    let mut rules = Vec::new();

    while !peek_newline(&mut src) {
        let mut rule = Vec::new();

        while let Some(rule_token_type) = parse_rule_token_type(&mut src)? {
            match rule_token_type {
                RuleTokenType::Symbol => {
                    let symbol = parse_usize(&mut src)?;
                    rule.push(RuleToken::Symbol(symbol));
                },
                RuleTokenType::Parameter => {
                    let parameter = parse_usize(&mut src)?;
                    rule.push(RuleToken::Parameter(parameter));
                },
                RuleTokenType::Constant => {
                    let constant = parse_usize(&mut src)?;
                    rule.push(RuleToken::Constant(constant));
                },
                RuleTokenType::Operator => {
                    let operator = parse_operator(&mut src)?;
                    rule.push(RuleToken::Operator(operator));
                },
            }
        }
        
        rules.push(rule);
    }

    parse_newline(&mut src)?;

    /* consts */
    let mut consts = Vec::new();

    while !peek_newline(&mut src) {
        let constant = parse_f32(&mut src)?;
        consts.push(constant);
    }

    parse_newline(&mut src)?;
    parse_newline(&mut src)?;

    /* actions */
    let mut actions = Vec::new();

    while !peek_newline(&mut src) {
        let action = parse_action(&mut src)?;
        actions.push(action);
    }

    parse_newline(&mut src)?;

    /* return */
    Ok(LSystem { axiom, rules, consts, actions })
}

pub fn load_from_file(path: impl AsRef<Path>) -> Result<LSystem, LSystemLoaderError> {
    let f = File::open(path).unwrap(); // TODO: error
    let f_buf_reader = BufReader::new(f);
    let f_iter = f_buf_reader.bytes().map(|byte| byte.unwrap());

    load(f_iter)
}