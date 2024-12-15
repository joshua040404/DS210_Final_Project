use std::collections::HashMap;

pub fn train_logistic_model(data: &[HashMap<String, String>]) -> Vec<f64> {
    let features: Vec<Vec<f64>> = data.iter()
        .map(|row| row.values().filter_map(|v| v.parse::<f64>().ok()).collect())
        .collect();
    let target: Vec<f64> = data.iter()
        .filter_map(|row| row.get("quality").and_then(|v| v.parse::<f64>().ok()))
        .collect();
    let mut weights = vec![0.0; features[0].len()];
    let learning_rate = 0.01;

    for _ in 0..1000 {
        let mut gradient = vec![0.0; weights.len()];
        
        for (i, feature) in features.iter().enumerate() {
            let prediction: f64 = weights.iter().zip(feature).map(|(w, x)| w * x).sum();
            let error = prediction - target[i];

            for (j, &x) in feature.iter().enumerate() {
                gradient[j] += error * x;
            }
        }

        for i in 0..weights.len() {
            weights[i] -= learning_rate * gradient[i];
        }
    }

    weights
}

pub fn predict(data: &[HashMap<String, String>], weights: &[f64]) -> Vec<f64> {
    data.iter()
        .map(|row| {
            row.values()
                .filter_map(|v| v.parse::<f64>().ok())
                .zip(weights.iter())
                .map(|(x, w)| x * w)
                .sum()
        })
        .collect()
}