//! General functions

use crate::analyze::{highest, highest_x, highest_y, lowest_x, lowest_y, set_range, split_up};
use crate::parse::to_xy;
use crate::Xy;
use crate::{Log, Parameters};
use kern::Fail;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::PointStyle;
use plotlib::view::ContinuousView;

/// None instead of ""
pub fn none_empty(opt: Option<&String>) -> Option<&String> {
    if let Some(value) = opt {
        if value.is_empty() {
            return None;
        }
    }
    opt
}

/// Analyse log and return svg image
pub fn draw(log: &str, params: Parameters) -> Result<String, Fail> {
    // parse log
    let log = Log::from(&log)?;
    let x_values = log.at_key(params.x_axis).or_else(Fail::from)?;
    let y_values = log.at_key(params.y_axis).or_else(Fail::from)?;
    let values = to_xy(&x_values, &y_values).or_else(Fail::from)?;

    // get highest and lowest
    let highest_x = highest_x(&values).1;
    let highest_y = highest_y(&values).1;
    let lowest_x = lowest_x(&values).1;
    let lowest_y = lowest_y(&values).1;

    // get highest
    let highest = if let Some(height) = params.height {
        if let Ok(height_values) = log.at_key(height) {
            highest(&height_values).0
        } else {
            values.len()
        }
    } else {
        values.len()
    };

    // split and set range
    let (rise_values, fall_values) = split_up(values, highest);
    let rise_values = set_range(
        rise_values,
        params.x_min,
        params.x_max,
        params.y_min,
        params.y_max,
    );
    let fall_values = set_range(
        fall_values,
        params.x_min,
        params.x_max,
        params.y_min,
        params.y_max,
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

    // add plots to view
    let rise_plot = new_plot(rise_values, params.colour, params.size);
    let fall_plot = new_plot(fall_values, params.colour_fall, params.size);
    view = view.add(rise_plot);
    view = view.add(fall_plot);

    // set x-range
    if let (Some(x_min), Some(x_max)) = (params.x_min, params.x_max) {
        if let (Ok(x_min), Ok(x_max)) = (x_min.parse(), x_max.parse()) {
            view = view.x_range(x_min, x_max);
        }
    } else if let Some(x_min) = params.x_min {
        if let Ok(x_min) = x_min.parse() {
            view = view.x_range(x_min, highest_x);
        }
    } else if let Some(x_max) = params.x_max {
        if let Ok(x_max) = x_max.parse() {
            view = view.x_range(lowest_x, x_max);
        }
    }

    // set y-range
    if let (Some(y_min), Some(y_max)) = (params.y_min, params.y_max) {
        if let (Ok(y_min), Ok(y_max)) = (y_min.parse(), y_max.parse()) {
            view = view.y_range(y_min, y_max);
        }
    } else if let Some(y_min) = params.y_min {
        if let Ok(y_min) = y_min.parse() {
            view = view.y_range(y_min, highest_y);
        }
    } else if let Some(y_max) = params.y_max {
        if let Ok(y_max) = y_max.parse() {
            view = view.y_range(lowest_y, y_max);
        }
    }

    // return output
    Ok(Page::single(&view)
        .to_svg()
        .or_else(Fail::from)?
        .to_string())
}

/// Create plot
fn new_plot(values: Vec<Xy>, colour: Option<&String>, size: Option<&String>) -> Plot {
    Plot::new(values).point_style(
        PointStyle::new()
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
