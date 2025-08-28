use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::model::{expr::Token, number::Number};

lazy_static! {
    static ref RANGE: Regex = Regex::new(r"^\d+-\d+$").unwrap();
}


pub fn parse_inputs<T: Number>(string: &str) -> Result<HashMap<Token<T>, isize>, String> {
    let mut res = HashMap::new();

    for mut part in string.split(",") {
        part = part.trim();

        if part == "+" {
            res.entry(Token::Operation(super::expr::Operation::Sum)).or_insert(-1);

        } else if part == "-" {
            res.entry(Token::Operation(super::expr::Operation::Sub)).or_insert(-1);

        } else if part == "*" {
            res.entry(Token::Operation(super::expr::Operation::Mul)).or_insert(-1);

        } else if part == "/" {
            res.entry(Token::Operation(super::expr::Operation::Div)).or_insert(-1);

        } else if part == "^" {
            res.entry(Token::Operation(super::expr::Operation::Pow)).or_insert(-1);

        } else if part == "neg" {
            res.entry(Token::Operation(super::expr::Operation::Neg)).or_insert(-1);

        } else if part == "sqrt" {
            res.entry(Token::Operation(super::expr::Operation::Sqrt)).or_insert(-1);

        } else if part.parse::<f32>().is_ok() {
            let t = Token::Constant(T::from_f32(part.parse::<f32>().unwrap()));
            *res.entry(t).or_insert(0) += 1;
        
        } else if RANGE.is_match(part) {
            let sides = part.split("-").collect::<Vec<_>>();
            let from = sides[0].parse::<i32>().unwrap();
            let to = sides[1].parse::<i32>().unwrap();

            for i in from..=to {
                let t = Token::Constant(T::from_f32(i as f32));
                *res.entry(t).or_insert(0) += 1;
            }
        
        } else {
            return Err(format!("Invalid format: '{}'", part))
        }
    }

    Ok(res)
}