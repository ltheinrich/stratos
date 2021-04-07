//! General types

/// x and y tuple
pub type Xy = (f64, f64);

/// Parameter for log drawing
pub struct Parameters<'a> {
    pub x_axis: &'a str,
    pub y_axis: &'a str,
    pub x_name: Option<&'a String>,
    pub y_name: Option<&'a String>,
    pub x_min: Option<&'a String>,
    pub x_max: Option<&'a String>,
    pub y_min: Option<&'a String>,
    pub y_max: Option<&'a String>,
    pub size: Option<&'a String>,
    pub colour: Option<&'a String>,
    pub colour_fall: Option<&'a String>,
    pub height: Option<&'a String>,
}

// Parameters implementation
impl<'a> Parameters<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn from(
        x_axis: &'a str,
        y_axis: &'a str,
        x_name: Option<&'a String>,
        y_name: Option<&'a String>,
        x_min: Option<&'a String>,
        x_max: Option<&'a String>,
        y_min: Option<&'a String>,
        y_max: Option<&'a String>,
        size: Option<&'a String>,
        colour: Option<&'a String>,
        colour_fall: Option<&'a String>,
        height: Option<&'a String>,
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
