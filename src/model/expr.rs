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

#[derive(Clone)]
pub enum InfixExpression {
    Constant(f64),
    Unary(Operation, Box<InfixExpression>),
    Binary(Operation, Box<InfixExpression>, Box<InfixExpression>)
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
            Operation::Neg => "-",
            Operation::Sqrt => "V",

            Operation::Sum => "+",
            Operation::Sub => "-",
            Operation::Mul => "*",
            Operation::Div => "/",
            Operation::Pow => "^",
        }
    }
    
    pub fn latex(&self) -> &str {
        match self {
            Operation::Neg => "-",
            Operation::Sqrt => "V",

            Operation::Sum => "+",
            Operation::Sub => "-",
            Operation::Mul => "\\cdot",
            Operation::Div => "/",
            Operation::Pow => "^",
        }
    }
}

impl Expression {
    pub fn random(choices: &HashMap<Token, isize>) -> Self {
        let mut rng = rng();

        let mut tokens = vec!();
        let mut stack: isize = 0;
        let mut choices_cpy = choices.clone();
        let mut constants_rem = choices_cpy.iter()
            .filter(|i| matches!(i.0, Token::Constant(_)))
            .filter(|i| *i.1 > 0)
            .count();

        loop {
            // Check if all constants have been introduced
            if constants_rem == 0 && stack == 1 {
                break;
            }

            // Select a new token
            let last_was_neg = matches!(tokens.last(), Some(Token::Operation(Operation::Neg)));

            let (token, number) = choices_cpy.iter()
                .filter(|i| *i.1 > 0 || *i.1 == -1)
                .filter(|i| i.0.arity() as isize <= stack)
                .filter(|(t, _)| !(last_was_neg && matches!(t, Token::Operation(Operation::Neg))))
                .choose(&mut rng)
                .map(|(t, n)| (t.clone(), n.clone()))
                .unwrap();

            // Update available tokens and stack
            stack += 1 - token.arity() as isize;

            if number > 0 {
                choices_cpy.insert(token.clone(), number - 1);
            }

            // Reduce remaining constants
            constants_rem -= matches!(token, Token::Constant(_)) as usize;

            // Add token to expression
            tokens.push(token);
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

    pub fn to_infix(&self) -> InfixExpression {
        let mut stack = vec!();

        for token in &self.tokens {
            match token {
                Token::Constant(f) => stack.push(InfixExpression::Constant(f.into_inner())),

                Token::Operation(op) => match op {
                    Operation::Neg |
                    Operation::Sqrt => {
                        let a = stack.pop().unwrap();
                        stack.push(InfixExpression::Unary(op.clone(), Box::new(a)));
                    },

                    Operation::Sum |
                    Operation::Sub |
                    Operation::Mul |
                    Operation::Div |
                    Operation::Pow => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();

                        stack.push(InfixExpression::Binary(op.clone(), Box::new(a), Box::new(b)));
                    },

                },
            }
        }

        stack.last().cloned().unwrap()
    }

    pub fn repr(&self) -> String {
        self.tokens.iter()
            .map(|i| i.repr())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl InfixExpression {
    pub fn needs_parentheses(&self) -> bool {
        use Operation::*;
        use InfixExpression::*;

        !matches!(self, Constant(_) | Unary(Sqrt, _) | Binary(Div | Pow, _, _))
    }

    pub fn needs_arg_parentheses(&self) -> bool {
        use Operation::*;
        use InfixExpression::*;

        !matches!(self, Unary(Sqrt, _) | Binary(Div | Pow, _, _))
    }

    pub fn to_string(&self) -> String {
        match self {
            InfixExpression::Constant(n) => n.to_string(),
            
            InfixExpression::Unary(op, a) => {
                if let InfixExpression::Constant(n) = **a {
                    format!("{}{}", op.repr(), n)
                
                } else {
                    format!("{}({})", op.repr(), a.to_string())
                }
            },

            InfixExpression::Binary(op, a, b) => {
                let a_str = if let InfixExpression::Constant(n) = **a {
                    format!("{}", n)
                
                } else {
                    format!("({})", a.to_string())
                };

                let b_str = if let InfixExpression::Constant(n) = **b {
                    format!("{}", n)
                
                } else {
                    format!("({})", b.to_string())
                };

                format!("{} {} {}", a_str, op.repr(), b_str)
            },
        }
    }

    pub fn to_latex(&self) -> String {
        use Operation::*;

        match self {
            InfixExpression::Constant(n) => n.to_string(),
            
            InfixExpression::Unary(op, a) => {
                match op {
                    Neg => {
                        if a.needs_parentheses() {
                            format!("-({})", a.to_latex())
                        
                        } else {
                            format!("-{}", a.to_latex())
                        }
                    },

                    Sqrt => format!("\\sqrt{{{}}}", a.to_latex()),
                    
                    _ => unreachable!()
                }
            },

            InfixExpression::Binary(op, a, b) => {
                let a_str = if a.needs_parentheses() && self.needs_arg_parentheses() {
                    format!("({})", a.to_latex())
                
                } else {
                    format!("{}", a.to_latex())
                };

                let b_str = if b.needs_parentheses() && self.needs_arg_parentheses() {
                    format!("({})", b.to_latex())
                
                } else {
                    format!("{}", b.to_latex())
                };

                match op {
                    Div => format!("\\frac{{{}}}{{{}}}", a_str, b_str),
                    Pow => format!("{{{}}}^{{{}}}", a_str, b_str),

                    _ => format!("{} {} {}", a_str, op.latex(), b_str)
                }
            },
        }
    }
}