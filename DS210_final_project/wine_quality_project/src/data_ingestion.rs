use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn load_data(file_path: &str) -> Vec<HashMap<String, String>> {
    let file = File::open(file_path).expect("Failed to open the file");
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    let mut headers = Vec::new();
    
    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
        let values: Vec<&str> = line.split(';').collect();
        if index == 0 {
            headers = values.iter().map(|s| s.trim_matches('"').to_string()).collect();
        } else {
            let mut row = HashMap::new();

            for (header, value) in headers.iter().zip(values.iter()) {
                row.insert(header.clone(), value.trim_matches('"').to_string());
            }

            data.push(row);
        }
    }

    data
}

pub fn normalize_features(data: &mut [HashMap<String, String>]) {
    let mut feature_min = HashMap::new();
    let mut feature_max = HashMap::new();

    for row in data.iter() {
        for (key, value) in row.iter() {
            if let Ok(value) = value.parse::<f64>() {
                let min_entry = feature_min.entry(key.clone()).or_insert(value);
                let max_entry = feature_max.entry(key.clone()).or_insert(value);

                if value < *min_entry {
                    *min_entry = value;
                }
                if value > *max_entry {
                    *max_entry = value;
                }
            }
        }
    }

    for row in data.iter_mut() {
        for (key, value) in row.iter_mut() {
            if let Ok(val) = value.parse::<f64>() {
                if let (Some(&min), Some(&max)) = (feature_min.get(key), feature_max.get(key)) {
                    if max > min {
                        let normalized = (val - min) / (max - min);
                        *value = format!("{:.6}", normalized);
                    } else {
                        *value = "0.0".to_string();
                    }
                }
            }
        }
    }
    
}
