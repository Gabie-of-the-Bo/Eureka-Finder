use std::cmp::Ordering;

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::model::{expr::Expression, number::Number, parsing::parse_inputs};

pub fn find_expression<T: Number>(inputs: &str, objective: T) -> Expression<T> {
    let choices = parse_inputs(inputs).unwrap();

    let expr = (0..1000).into_iter()
        .par_bridge()
        .map(|_| Expression::<T>::random(&choices))
        .map(|e| {
            let res = e.calculate().distance(&objective);
            (e, res)
        })
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap();

    expr.0
}