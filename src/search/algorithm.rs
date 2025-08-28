use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::model::{expr::Expression, number::Number, parsing::parse_inputs};

pub fn find_expression<T: Number>(inputs: &str, objective: T, threshold: f64) -> Expression<T> {
    let choices = parse_inputs(inputs).unwrap();

    let expr = (0..).into_iter()
        .par_bridge()
        .map(|_| Expression::<T>::random(&choices))
        .find_any(|e| e.calculate().distance(&objective) < threshold)
        .unwrap();

    expr
}