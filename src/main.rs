use std::f64::consts::PI;

use num::Complex;

use crate::search::algorithm::find_expression;

pub mod model {
    pub mod expr;
    pub mod number;
}

pub mod search {
    pub mod algorithm;
}

fn main() {
    let expr = find_expression(Complex::<f64>::from(PI), 1e-6);

    let res = expr.calculate();

    println!("Result:\n\t{}\n\t{}\n\t{}", expr.to_infix().to_string(), expr.to_infix().to_latex(), res);
}
