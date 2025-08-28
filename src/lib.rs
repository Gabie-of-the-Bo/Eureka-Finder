pub mod model {
    pub mod expr;
    pub mod number;
    pub mod parsing;
}

pub mod search {
    pub mod algorithm;
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use num::Complex;

    use crate::{model::number::Number, search::algorithm::find_expression};

    pub fn test_approx<T: Number>(inputs: &str, objective: T, thres: f64) {
        let expr = find_expression(inputs, objective, thres);

        assert!(expr.calculate().distance(&objective) < thres)
    }

    #[test]
    fn f32_approx() {
        test_approx(
            "+,-,/,*,^,neg,sqrt,1-9",
            PI as f32,
            1e-4
        );
    }

    #[test]
    fn f64_approx() {
        test_approx(
            "+,-,/,*,^,neg,sqrt,1-9",
            PI,
            1e-4
        );
    }

    #[test]
    fn complex_f32_approx() {
        test_approx(
            "+,-,/,*,^,neg,sqrt,1-9",
            Complex::<f32>::from(PI as f32),
            1e-4
        );
    }

    #[test]
    fn complex_f64_approx() {
        test_approx(
            "+,-,/,*,^,neg,sqrt,1-9",
            Complex::<f64>::from(PI),
            1e-4
        );
    }
}