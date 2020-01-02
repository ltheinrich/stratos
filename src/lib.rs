//! Stratos Rust library

extern crate kern;
extern crate plotlib;

pub mod analyze;
pub mod http;
pub mod parse;

mod meta;

pub use crate::meta::{init_version, version};

use plotlib::{
    page::Page,
    scatter::{Scatter, Style},
    style::Point,
    view::ContinuousView,
};
use std::error;

/// x and y
pub type XY = (f64, f64);

/// Parameter for log drawing
pub struct Parameters<'a> {
    x_axis: &'a str,
    y_axis: &'a str,
    x_name: Option<&'a &'a str>,
    y_name: Option<&'a &'a str>,
    x_min: Option<&'a &'a str>,
    x_max: Option<&'a &'a str>,
    y_min: Option<&'a &'a str>,
    y_max: Option<&'a &'a str>,
    size: Option<&'a &'a str>,
    colour: Option<&'a &'a str>,
    colour_fall: Option<&'a &'a str>,
    height: Option<&'a &'a str>,
}

// Parameters implementation
impl<'a> Parameters<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn from(
        x_axis: &'a str,
        y_axis: &'a str,
        x_name: Option<&'a &'a str>,
        y_name: Option<&'a &'a str>,
        x_min: Option<&'a &'a str>,
        x_max: Option<&'a &'a str>,
        y_min: Option<&'a &'a str>,
        y_max: Option<&'a &'a str>,
        size: Option<&'a &'a str>,
        colour: Option<&'a &'a str>,
        colour_fall: Option<&'a &'a str>,
        height: Option<&'a &'a str>,
    ) -> Self {
        Self {
            x_axis,
            y_axis,
            x_name,
            y_name,
            x_min,
            x_max,
            y_min,
            y_max,
            size,
            colour,
            colour_fall,
            height,
        }
    }
}

/// Analyse log and return svg image
pub fn draw<'a>(log: &'a str, params: Parameters) -> Result<String, Box<dyn error::Error>> {
    // parse log
    let log = parse::parse_log(&log);
    let x_values = log.at_key(params.x_axis)?;
    let x_values = parse::into_f64(&x_values);
    let y_values = log.at_key(params.y_axis)?;
    let y_values = parse::into_f64(&y_values);

    // process values
    let values = parse::into_xy(x_values, y_values);

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
    if let Some(height) = params.height {
        // get height values
        let height_values = log.at_key(height)?;
        let height_values = parse::into_f64(&height_values);

        // split and create scatters
        let (rise_values, fall_values) = analyze::split_up(&values, &height_values);
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

    // add x/y range
    if let (Some(x_min), Some(x_max)) = (params.x_min, params.x_max) {
        if let (Ok(x_min), Ok(x_max)) = (x_min.parse(), x_max.parse()) {
            view = view.x_range(x_min, x_max);
        }
    }
    if let (Some(y_min), Some(y_max)) = (params.y_min, params.y_max) {
        if let (Ok(y_min), Ok(y_max)) = (y_min.parse(), y_max.parse()) {
            view = view.y_range(y_min, y_max);
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
