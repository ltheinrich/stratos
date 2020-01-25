//! General types

/// x and y tuple
pub type XY = (f64, f64);

/// Parameter for log drawing
pub struct Parameters<'a> {
    pub x_axis: &'a str,
    pub y_axis: &'a str,
    pub x_name: Option<&'a &'a str>,
    pub y_name: Option<&'a &'a str>,
    pub x_min: Option<&'a &'a str>,
    pub x_max: Option<&'a &'a str>,
    pub y_min: Option<&'a &'a str>,
    pub y_max: Option<&'a &'a str>,
    pub size: Option<&'a &'a str>,
    pub colour: Option<&'a &'a str>,
    pub colour_fall: Option<&'a &'a str>,
    pub height: Option<&'a &'a str>,
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
