//! Advanced analysis

use crate::XY;

/// Split xy-tuple into seperate vectors (rise, fall)
pub fn split_up(values: Vec<XY>, highest: usize) -> (Vec<XY>, Vec<XY>) {
    // check if empty vector
    if values.is_empty() {
        return (vec![], vec![]);
    }

    // split into rise and fall
    let mut fall = Vec::new();
    let rise: Vec<XY> = values
        .into_iter()
        .enumerate()
        .filter(|(i, v)| {
            if i > &highest {
                fall.push(*v);
                false
            } else {
                true
            }
        })
        .map(|(_, v)| v)
        .collect();

    // return vectors
    (rise, fall)
}

/// Get highest value in list
/// Returns index and value
pub fn highest(values: &[f64]) -> (usize, f64) {
    let mut max = (0, std::f64::MIN);
    values.iter().enumerate().for_each(|(i, &v)| {
        if v > max.1 {
            max = (i, v);
        }
    });
    max
}

/* unused
/// Get lowest value in list
/// Returns index and value
pub fn lowest(values: &[f64]) -> (usize, f64) {
    let mut min = (0, std::f64::MAX);
    values.iter().enumerate().for_each(|(i, &v)| {
        if v < min.1 {
            min = (i, v);
        }
    });
    min
}
*/

/// Get highest x-value in XY-list
/// Returns index and value
pub fn highest_x(values: &[XY]) -> (usize, f64) {
    let mut max = (0, std::f64::MIN);
    values.iter().enumerate().for_each(|(i, &v)| {
        if v.0 > max.1 {
            max = (i, v.0);
        }
    });
    max
}

/// Get lowest x-value in XY-list
/// Returns index and value
pub fn lowest_x(values: &[XY]) -> (usize, f64) {
    let mut min = (0, std::f64::MAX);
    values.iter().enumerate().for_each(|(i, &v)| {
        if v.0 < min.1 {
            min = (i, v.0);
        }
    });
    min
}

/// Get highest y-value in XY-list
/// Returns index and value
pub fn highest_y(values: &[XY]) -> (usize, f64) {
    let mut max = (0, std::f64::MIN);
    values.iter().enumerate().for_each(|(i, &v)| {
        if v.1 > max.1 {
            max = (i, v.1);
        }
    });
    max
}

/// Get lowest y-value in XY-list
/// Returns index and value
pub fn lowest_y(values: &[XY]) -> (usize, f64) {
    let mut min = (0, std::f64::MAX);
    values.iter().enumerate().for_each(|(i, &v)| {
        if v.1 < min.1 {
            min = (i, v.1);
        }
    });
    min
}

/// Set range for XY values
pub fn set_range(
    mut values: Vec<XY>,
    x_min: Option<&&str>,
    x_max: Option<&&str>,
    y_min: Option<&&str>,
    y_max: Option<&&str>,
) -> Vec<XY> {
    // set x-min
    if let Some(x_min) = x_min {
        if let Ok(x_min) = x_min.parse() {
            values = remove_lower_x(values, x_min);
        }
    }

    // set x-max
    if let Some(x_max) = x_max {
        if let Ok(x_max) = x_max.parse() {
            values = remove_higher_x(values, x_max);
        }
    }

    // set y-min
    if let Some(y_min) = y_min {
        if let Ok(y_min) = y_min.parse() {
            values = remove_lower_y(values, y_min);
        }
    }

    // set y-max
    if let Some(y_max) = y_max {
        if let Ok(y_max) = y_max.parse() {
            values = remove_higher_y(values, y_max);
        }
    }

    // return values
    values
}

/// Remove higher values (x) from XY
/// Returns XY-list
pub fn remove_higher_x(values: Vec<XY>, highest: f64) -> Vec<XY> {
    values
        .into_iter()
        .filter(|v| {
            if v.0 <= highest {
                return true;
            }
            false
        })
        .collect()
}

/// Remove lower values (x) from XY
/// Returns XY-list
pub fn remove_lower_x(values: Vec<XY>, lowest: f64) -> Vec<XY> {
    values
        .into_iter()
        .filter(|v| {
            if v.0 >= lowest {
                return true;
            }
            false
        })
        .collect()
}

/// Remove higher values (y) from XY
/// Returns XY-list
pub fn remove_higher_y(values: Vec<XY>, highest: f64) -> Vec<XY> {
    values
        .into_iter()
        .filter(|v| {
            if v.1 <= highest {
                return true;
            }
            false
        })
        .collect()
}

/// Remove lower values (y) from XY
/// Returns XY-list
pub fn remove_lower_y(values: Vec<XY>, lowest: f64) -> Vec<XY> {
    values
        .into_iter()
        .filter(|v| {
            if v.1 >= lowest {
                return true;
            }
            false
        })
        .collect()
}
