use std::collections::HashMap;

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::model::{expr::Expression, number::Number};

pub fn find_expression<T: Number>(objective: T, threshold: f64) -> Expression<T> {
    use crate::model::expr::Token::*;
    use crate::model::expr::Operation::*;

    let mut choices = [
        (Operation(Neg), -1),
        (Operation(Sqrt), -1),
        (Operation(Sum), -1),
        (Operation(Sub), -1),
        (Operation(Mul), -1),
        (Operation(Div), -1),
        (Operation(Pow), -1),
    ].into_iter().collect::<HashMap<_, _>>();

    for i in 0..10 {
        choices.insert(Constant(T::from_f32(i as f32)), 1);
    }

    let expr = (0..).into_iter()
        .par_bridge()
        .map(|_| Expression::<T>::random(&choices))
        .find_any(|e| e.calculate().distance(&objective) < threshold)
        .unwrap();

    expr
}