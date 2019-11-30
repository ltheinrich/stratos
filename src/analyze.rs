//! Advanced analysis

use crate::XY;

/// Split xy-tuple into seperate vectors (rise, fall)
pub fn split_up<'a>(values: &'a [XY], height: &[f64]) -> (&'a [XY], &'a [XY]) {
    let mut max = (0, 0.);
    height.iter().enumerate().for_each(|(i, &h)| {
        if h >= max.1 {
            max = (i, h);
        }
    });
    values.split_at(max.0)
}
