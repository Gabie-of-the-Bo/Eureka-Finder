use std::collections::HashMap;

use ordered_float::OrderedFloat;
use rand::{rng, seq::IteratorRandom};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Operation {
    Neg, Sqrt,
    Sum, Sub, Mul, Div, Pow
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Constant(OrderedFloat<f64>),
    Operation(Operation)
}

pub struct Expression {
    tokens: Vec<Token>
}

impl Token {
    pub fn arity(&self) -> usize {
        match self {
            Token::Constant(_) => 0,
            Token::Operation(op) => op.arity(),
        }
    }

    pub fn repr(&self) -> String {
        match self {
            Token::Constant(f) => f.to_string(),
            Token::Operation(op) => op.repr().to_string(),
        }
    }
}

impl Operation {
    pub fn arity(&self) -> usize {
        match self {
            Operation::Neg |
            Operation::Sqrt => 1,

            Operation::Sum |
            Operation::Sub |
            Operation::Mul |
            Operation::Div |
            Operation::Pow => 2,
        }
    }
    
    pub fn repr(&self) -> &str {
        match self {
            Operation::Neg => "N",
            Operation::Sqrt => "V",

            Operation::Sum => "+",
            Operation::Sub => "-",
            Operation::Mul => "*",
            Operation::Div => "/",
            Operation::Pow => "^",
        }
    }
}

impl Expression {
    pub fn random(choices: &HashMap<Token, isize>) -> Self {
        let mut rng = rng();

        let mut tokens = vec!();
        let mut stack = 0;
        let mut choices_cpy = choices.clone();

        loop {
            // Check if all constants have been introduced
            let constants = choices_cpy.iter()
                .filter(|i| matches!(i.0, Token::Constant(_)))
                .filter(|i| *i.1 > 0)
                .count();

            if constants == 0 && stack == 1 {
                break;
            }

            // Select a new token
            let (token, number) = choices_cpy.iter()
                .filter(|i| *i.1 > 0 || *i.1 == -1)
                .filter(|i| i.0.arity() <= stack)
                .choose(&mut rng)
                .map(|(t, n)| (t.clone(), n.clone()))
                .unwrap();

            // Special case for negations
            if let Token::Operation(Operation::Neg) = token {
                if let Some(Token::Operation(Operation::Neg)) = tokens.last() {
                    continue;
                }
            }

            // Update available tokens
            choices_cpy.insert(token.clone(), (number - 1).max(-1));

            // Add token to expression
            tokens.push(token.clone());

            if token.arity() > 1 {
                stack -= token.arity() - 1;
            
            } else {
                stack += 1 - token.arity();
            }
        }

        Expression { tokens }
    }

    pub fn calculate(&self) -> f64 {
        let mut stack = vec!();

        macro_rules! binary {
            ($op: tt) => {
                {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();

                    stack.push(a $op b);
                }
            };
        }

        for t in &self.tokens {
            match t {
                Token::Constant(f) => stack.push(f.into_inner()),
                
                Token::Operation(op) => match op {
                    Operation::Neg => {
                        let arg = stack.pop().unwrap();

                        stack.push(-arg);
                    },

                    Operation::Sqrt => {
                        let arg = stack.pop().unwrap();

                        stack.push(arg.sqrt());
                    },

                    Operation::Sum => binary!(+),
                    Operation::Sub => binary!(-),
                    Operation::Mul => binary!(*),
                    Operation::Div => binary!(/),

                    Operation::Pow => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();

                        stack.push(a.powf(b));
                    },
                },
            }
        }
        
        stack.last().copied().unwrap()
    }

    pub fn repr(&self) -> String {
        self.tokens.iter()
            .map(|i| i.repr())
            .collect::<Vec<_>>()
            .join(" ")
    }
}