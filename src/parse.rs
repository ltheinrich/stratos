pub fn parse_log(log: &str) -> Log {
    let mut lines: Vec<&str> = log.lines().collect();
    lines.remove(0);
    let header: Vec<&str> = lines[0].split(';').collect();
    lines.remove(0);
    let mut data = Vec::with_capacity(lines.len());
    for line in lines {
        data.push(line.split(';').collect::<Vec<&str>>());
    }
    Log { header, data }
}

pub struct Log<'a> {
    // Temp, Height, etc.
    header: Vec<&'a str>,

    // [1, 1, etc.], [2, 2, etc.], etc.
    data: Vec<Vec<&'a str>>,
}

impl<'a> Log<'a> {
    pub fn get_data(&self) -> &Vec<Vec<&'a str>> {
        &self.data
    }

    pub fn at_key(&self, key: &str) -> Option<Vec<&'a str>> {
        let mut index = 0;
        for (i, &header_key) in self.header.iter().enumerate() {
            if header_key == key {
                index = i;
                break;
            } else if i + 1 == self.header.len() {
                return None;
            }
        }
        let mut values = Vec::with_capacity(self.data.len());
        for line in &self.data {
            values.push(line[index]);
        }
        Some(values)
    }
}

pub fn into_f64(values: &Vec<&str>) -> Vec<f64> {
    let mut converted = Vec::new();
    for &value in values {
        converted.push(value.parse().unwrap_or(if value == "Y" { 1. } else { 0. }))
    }
    converted
}

pub fn into_tuple(values1: Vec<f64>, values2: Vec<f64>) -> Vec<(f64, f64)> {
    let mut tuples = Vec::new();
    for value1 in values1 {
        tuples.push((value1, 0.));
    }
    for (index, &value2) in values2.iter().enumerate() {
        tuples[index].1 = value2;
    }
    tuples
}
