//! General functions

use crate::analyze::{highest_x, highest_y, lowest_x, lowest_y, set_range, split_up};
use crate::parse::to_xy;
use crate::XY;
use crate::{Log, Parameters};
use plotlib::page::Page;
use plotlib::scatter::{Scatter, Style};
use plotlib::style::Point;
use plotlib::view::ContinuousView;
use std::error;

/// None instead of ""
pub fn none_empty<'a, 'b>(opt: Option<&'a &'b str>) -> Option<&'a &'b str> {
    if let Some(value) = opt {
        if value == &"" {
            return None;
        }
    }
    opt
}

/// Analyse log and return svg image
pub fn draw<'a>(log: &'a str, params: Parameters) -> Result<String, Box<dyn error::Error>> {
    // parse log
    let log = Log::from(&log)?;
    let x_values = log.at_key(params.x_axis)?;
    let y_values = log.at_key(params.y_axis)?;
    let values = to_xy(&x_values, &y_values)?;
    let mut height_values = match params.height {
        Some(height) => Some(log.at_key(height)?),
        None => None,
    };

    let values = set_range(
        values,
        params.x_min,
        params.x_max,
        params.y_min,
        params.y_max,
        &mut height_values,
    );

    // create view
    let mut view = ContinuousView::new()
        .x_label(if let Some(x_name) = params.x_name {
            (*x_name).to_string()
        } else {
            params.x_axis.to_string()
        })
        .y_label(if let Some(y_name) = params.y_name {
            (*y_name).to_string()
        } else {
            params.y_axis.to_string()
        });

    // create scatters
    let mut scatters = (Scatter::from_slice(&[]), Scatter::from_slice(&[]));
    if let Some(height_values) = height_values {
        // split and create scatters
        let (rise_values, fall_values) = split_up(&values, &height_values);
        let rise_scatter = new_scatter(&rise_values, params.colour, params.size);
        let fall_scatter = new_scatter(&fall_values, params.colour_fall, params.size);

        // add scatters
        scatters.0 = rise_scatter;
        scatters.1 = fall_scatter;
        view = view.add(&scatters.0);
        view = view.add(&scatters.1);
    } else {
        // create and add scatter to vector
        let scatter = new_scatter(&values, params.colour, params.size);
        scatters.0 = scatter;
        view = view.add(&scatters.0);
    }

    // set x-range
    if let (Some(x_min), Some(x_max)) = (params.x_min, params.x_max) {
        if let (Ok(x_min), Ok(x_max)) = (x_min.parse(), x_max.parse()) {
            view = view.x_range(x_min, x_max);
        }
    } else if let Some(x_min) = params.x_min {
        if let Ok(x_min) = x_min.parse() {
            view = view.x_range(x_min, highest_x(&values).1);
        }
    } else if let Some(x_max) = params.x_max {
        if let Ok(x_max) = x_max.parse() {
            view = view.x_range(lowest_x(&values).1, x_max);
        }
    }

    // set y-range
    if let (Some(y_min), Some(y_max)) = (params.y_min, params.y_max) {
        if let (Ok(y_min), Ok(y_max)) = (y_min.parse(), y_max.parse()) {
            view = view.y_range(y_min, y_max);
        }
    } else if let Some(y_min) = params.y_min {
        if let Ok(y_min) = y_min.parse() {
            view = view.y_range(y_min, highest_y(&values).1);
        }
    } else if let Some(y_max) = params.y_max {
        if let Ok(y_max) = y_max.parse() {
            view = view.y_range(lowest_y(&values).1, y_max);
        }
    }

    // return output
    Ok(Page::single(&view).to_svg()?.to_string())
}

/// Create scatter
fn new_scatter(values: &[XY], colour: Option<&&str>, size: Option<&&str>) -> Scatter {
    Scatter::from_slice(values).style(
        &Style::new()
            .colour(if let Some(colour) = colour {
                (*colour).to_string()
            } else {
                String::from("#DD3355")
            })
            .size(if let Some(size) = size {
                size.parse().unwrap_or(1u8)
            } else {
                1u8
            }),
    )
}
