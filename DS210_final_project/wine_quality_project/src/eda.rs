use std::collections::HashMap;

pub fn compute_statistics(data: &[HashMap<String, String>]) -> HashMap<String, f64> {
    let mut stats = HashMap::new();
    let mut alcohol_sum = 0.0;
    let mut quality_sum = 0.0;
    let count = data.len() as f64;

    for row in data {
        if let (Some(alcohol), Some(quality)) = (row.get("alcohol"), row.get("quality")) {
            if let (Ok(alcohol_val), Ok(quality_val)) = (alcohol.parse::<f64>(), quality.parse::<f64>()) {
                alcohol_sum += alcohol_val;
                quality_sum += quality_val;
            }
        }
    }

    stats.insert("avg_alcohol".to_string(), alcohol_sum / count);
    stats.insert("avg_quality".to_string(), quality_sum / count);
    stats
}