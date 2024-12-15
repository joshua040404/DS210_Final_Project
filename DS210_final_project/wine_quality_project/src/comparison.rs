pub fn compare_features(red_weights: &[f64], white_weights: &[f64]) -> Vec<(usize, f64, f64)> {
    red_weights.iter()
        .zip(white_weights.iter())
        .enumerate()
        .map(|(i, (red, white))| (i, *red, *white))
        .collect()
}