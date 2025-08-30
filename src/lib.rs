use js_sys::{Date, Function};
use wasm_bindgen::prelude::*;

use crate::{model::number::Number, search::algorithm::find_expression};

pub mod model {
    pub mod expr;
    pub mod number;
    pub mod parsing;
}

pub mod search {
    pub mod algorithm;
}

#[wasm_bindgen]
pub fn find_function(inputs: &str, objective: f64, threshold: f64, max_seconds: usize, render_callback: Function) {
    let mut min_dist = 1e100;

    let start = Date::now();

    loop {
        let expr = find_expression(inputs, objective);

        let latex = expr.to_infix().to_latex();
        let result = expr.calculate();
        let distance = result.distance(&objective);

        if distance < min_dist {
            min_dist = distance;
    
            render_callback.call3(
                &JsValue::NULL, &latex.into(), &result.to_latex().into(), &distance.into()
            ).expect("Error while executing callback");

            if distance < threshold || (max_seconds > 0 && Date::now() - start > (1000 * max_seconds) as f64) {
                break;
            }
        }
    }
}