pub fn compute_feature_importance(weights: &[f64]) -> Vec<(usize, f64)> {
    let mut importance: Vec<(usize, f64)> = weights.iter()
        .enumerate()
        .map(|(i, &weight)| (i, weight.abs()))
        .collect();

    importance.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    importance
}