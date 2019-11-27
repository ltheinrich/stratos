pub fn parse_log(log: &str) -> Log {
    // split lines and remove comment line
    let mut lines: Vec<&str> = log.lines().collect();
    lines.remove(0);

    // split header and remove header from line list
    let header: Vec<&str> = lines[0].split(';').collect();
    lines.remove(0);

    // parse data
    let mut data = Vec::with_capacity(lines.len());
    for line in lines {
        data.push(line.split(';').collect::<Vec<&str>>());
    }

    Log { header, data }
}

/// Log structure
pub struct Log<'a> {
    // Temp, Height, etc.
    header: Vec<&'a str>,

    // [1, 1, etc.], [2, 2, etc.], etc.
    data: Vec<Vec<&'a str>>,
}

impl<'a> Log<'a> {
    /// Get values for specific header key
    pub fn at_key(&self, key: &str) -> Option<Vec<&'a str>> {
        // determine index of key
        let mut index = 0;
        for (i, &header_key) in self.header.iter().enumerate() {
            if header_key == key {
                index = i;
                break;
            } else if i + 1 == self.header.len() {
                return None;
            }
        }

        // add values at index to vector
        let mut values = Vec::with_capacity(self.data.len());
        for line in &self.data {
            values.push(line[index]);
        }

        Some(values)
    }
}

/// Parse values into f64 (Y == 1., invalid == 0.)
pub fn into_f64(values: &[&str]) -> Vec<f64> {
    // initialize vector
    let mut converted = Vec::with_capacity(values.len());

    // iterate through values and parse
    for &value in values {
        converted.push(value.parse().unwrap_or(if value == "Y" { 1. } else { 0. }))
    }

    converted
}

/// Put seperate vectors into a vector of tuples
pub fn into_tuple(values1: Vec<f64>, values2: Vec<f64>) -> Vec<(f64, f64)> {
    // initialize vector and add items of first vector
    let mut tuples = Vec::with_capacity(values1.len());
    for value1 in values1 {
        tuples.push((value1, 0.));
    }

    // iterate through second vector and replace values
    for (index, &value2) in values2.iter().enumerate() {
        if let Some(tuple) = tuples.get_mut(index) {
            tuple.1 = value2;
        }
    }

    tuples
}
