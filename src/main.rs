use std::{collections::HashMap, f64::consts::{E, PI}};

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::model::expr::Expression;

pub mod model {
    pub mod expr;
}

fn main() {
    use crate::model::expr::Token::*;
    use crate::model::expr::Operation::*;

    let mut choices = [
        (Operation(Neg), -1),
        // (Operation(Sqrt), -1),
        (Operation(Sum), -1),
        (Operation(Sub), -1),
        (Operation(Mul), -1),
        (Operation(Div), -1),
        (Operation(Pow), -1),
    ].into_iter().collect::<HashMap<_, _>>();

    for i in 1..10 {
        choices.insert(Constant((i as f64).into()), 1);
    }

    let objective = E;
    let threshold = 1e-4;

    let expr = (0..).into_iter()
        .par_bridge()
        .map(|_| Expression::random(&choices))
        .find_any(|e| (e.calculate() - objective).abs() < threshold)
        .unwrap();

    let res = expr.calculate();

    println!("Result: {} => {} (Diff: {})", expr.repr(), res, (res - objective).abs());
}
