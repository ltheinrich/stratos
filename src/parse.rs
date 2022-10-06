//! Log parser

use crate::Xy;
use kern::{Fail, Result};

/// Log structure
pub struct Log<'a> {
    // Temp, Height, etc.
    header: Vec<&'a str>,

    // [1, 1, etc.], [2, 2, etc.], etc.
    data: Vec<Vec<&'a str>>,
}

impl<'a> Log<'a> {
    /// Parse log file to Log
    pub fn from(log: &str) -> Result<Log> {
        // split lines and remove comment line
        let mut lines: Vec<&str> = log.lines().collect();
        if lines.len() < 2 {
            return Fail::from("Die Log-Datei ist fehlerhaft");
        }
        lines.remove(0);

        // split header and remove header from line list
        let header: Vec<&str> = lines.first().unwrap_or(&"").split(';').collect();
        lines.remove(0);

        // parse data
        let mut data = Vec::with_capacity(lines.len());
        for line in lines {
            // add list of seperated and trimmed values
            data.push(line.split(';').map(|v| v.trim()).collect::<Vec<&str>>());
        }

        // return Log
        Ok(Log { header, data })
    }

    /// Get header names
    pub fn header(&self) -> &[&'a str] {
        // return header names
        &self.header
    }

    /* unused
    /// Get log values
    pub fn data(&self) -> &[Vec<&'a str>] {
        // return log values
        &self.data
    }
    */

    /// Get values for specific header key
    pub fn at_key(&self, key: &str) -> Result<Vec<f64>> {
        // determine index of key
        let mut index = 0;
        for (i, &header_key) in self.header.iter().enumerate() {
            // check if wanted header name
            if header_key == key {
                // correct header name
                index = i;
                break;
            } else if i + 1 == self.header.len() {
                // header name not found: end of list
                return Fail::from(format!("Werte für {} existieren im Log nicht", key));
            }
        }

        // add values at index to vector
        let mut values = Vec::with_capacity(self.data.len());
        for line in &self.data {
            // convert to f64
            values.push(match line.get(index) {
                Some(&value) => value.parse().unwrap_or(if value == "Y" {
                    // true boolean
                    1.
                } else if value.contains(':') {
                    // convert time hh:mm:ss to minutes
                    let hms: Vec<&str> = value.split(':').collect();
                    hms.get(2).unwrap_or(&"0").parse().unwrap_or(0.) / 60.
                        + hms.get(1).unwrap_or(&"0").parse().unwrap_or(0.)
                        + hms.first().unwrap_or(&"0").parse().unwrap_or(0.) * 60.
                } else {
                    // unknown format
                    0.
                }),
                None => 0.,
            })
        }

        Ok(values)
    }

    /* unused
    /// Get an XY Vec of two value groups
    pub fn get_xy(&self, key1: &str, key2: &str) -> Result<Vec<XY>> {
        // get values for header
        let values1 = self.at_key(key1)?;
        let values2 = self.at_key(key2)?;

        // initialize vector
        let mut tuples = Vec::with_capacity(values1.len());
        for value1 in values1 {
            add each item of first vector
            tuples.push((value1, 0.));
        }

        // iterate through second vector
        for (i, &value2) in values2.iter().enumerate() {
            if let Some(tuple) = tuples.get_mut(i) {
                // replace value in tuples
                tuple.1 = value2;
            }
        }

        // return tuples list
        Ok(tuples)
    }
    */
}

/// Combine into an XY Vec of two value groups
pub fn to_xy(values1: &[f64], values2: &[f64]) -> Result<Vec<Xy>> {
    // initialize vector
    let mut tuples = Vec::with_capacity(values1.len());
    for &value1 in values1 {
        // add each item of first vector
        tuples.push((value1, 0.));
    }

    // iterate through second vector
    for (i, &value2) in values2.iter().enumerate() {
        if let Some(tuple) = tuples.get_mut(i) {
            // replace value in tuples
            tuple.1 = value2;
        }
    }

    // return tuples list
    Ok(tuples)
}
