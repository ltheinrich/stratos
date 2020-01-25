//! Advanced analysis

use crate::XY;

/// Split xy-tuple into seperate vectors (rise, fall)
pub fn split_up<'a>(values: &'a [XY], height: &[f64]) -> (&'a [XY], &'a [XY]) {
    if values.is_empty() {
        return (&[], &[]);
    }
    values.split_at(highest(height).0)
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
    height: &mut Option<Vec<f64>>,
) -> Vec<XY> {
    let mut removed = Vec::new();

    // set x-min
    if let Some(x_min) = x_min {
        if let Ok(x_min) = x_min.parse() {
            let (temp_values, mut temp_removed) = remove_lower_x(values, x_min);
            values = temp_values;
            removed.append(&mut temp_removed);
        }
    }

    // set x-max
    if let Some(x_max) = x_max {
        if let Ok(x_max) = x_max.parse() {
            let (temp_values, mut temp_removed) = remove_higher_x(values, x_max);
            values = temp_values;
            removed.append(&mut temp_removed);
        }
    }

    // set y-min
    if let Some(y_min) = y_min {
        if let Ok(y_min) = y_min.parse() {
            let (temp_values, mut temp_removed) = remove_lower_y(values, y_min);
            values = temp_values;
            removed.append(&mut temp_removed);
        }
    }

    // set y-max
    if let Some(y_max) = y_max {
        if let Ok(y_max) = y_max.parse() {
            let (temp_values, mut temp_removed) = remove_higher_y(values, y_max);
            values = temp_values;
            removed.append(&mut temp_removed);
        }
    }

    // remove values from height
    if let Some(temp_height) = height {
        removed.sort();
        removed.dedup();
        removed.iter().enumerate().for_each(|(i, ri)| {
            temp_height.remove(ri - i);
        });
    }

    values
}

/// Remove higher values (x) from XY
/// Returns XY-list and removed indexes
pub fn remove_higher_x(mut values: Vec<XY>, highest: f64) -> (Vec<XY>, Vec<usize>) {
    let mut removed = Vec::new();
    values = values
        .into_iter()
        .enumerate()
        .filter(|(i, v)| {
            if v.0 <= highest {
                removed.push(*i);
                return true;
            }
            false
        })
        .map(|(_, v)| v)
        .collect();
    (values, removed)
}

/// Remove lower values (x) from XY
/// Returns XY-list and removed indexes
pub fn remove_lower_x(mut values: Vec<XY>, lowest: f64) -> (Vec<XY>, Vec<usize>) {
    let mut removed = Vec::new();
    values = values
        .into_iter()
        .enumerate()
        .filter(|(i, v)| {
            if v.0 >= lowest {
                removed.push(*i);
                return true;
            }
            false
        })
        .map(|(_, v)| v)
        .collect();
    (values, removed)
}

/// Remove higher values (y) from XY
/// Returns XY-list and removed indexes
pub fn remove_higher_y(mut values: Vec<XY>, highest: f64) -> (Vec<XY>, Vec<usize>) {
    let mut removed = Vec::new();
    values = values
        .into_iter()
        .enumerate()
        .filter(|(i, v)| {
            if v.1 <= highest {
                removed.push(*i);
                return true;
            }
            false
        })
        .map(|(_, v)| v)
        .collect();
    (values, removed)
}

/// Remove lower values (y) from XY
/// Returns XY-list and removed indexes
pub fn remove_lower_y(mut values: Vec<XY>, lowest: f64) -> (Vec<XY>, Vec<usize>) {
    let mut removed = Vec::new();
    values = values
        .into_iter()
        .enumerate()
        .filter(|(i, v)| {
            if v.1 >= lowest {
                removed.push(*i);
                return true;
            }
            false
        })
        .map(|(_, v)| v)
        .collect();
    (values, removed)
}
