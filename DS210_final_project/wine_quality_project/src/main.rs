mod data_ingestion;
mod eda;
mod model;
mod comparison;
mod feature_importance;

use data_ingestion::{load_data, normalize_features};
use eda::compute_statistics;
use model::{train_logistic_model, predict};
use comparison::compare_features;
use feature_importance::compute_feature_importance;

fn main() {
    let mut red_wine_data = load_data("src/winequality-red.csv");
    let mut white_wine_data = load_data("src/winequality-white.csv");

    normalize_features(&mut red_wine_data);
    normalize_features(&mut white_wine_data);

    let red_stats = compute_statistics(&red_wine_data);
    let white_stats = compute_statistics(&white_wine_data);

    println!("Red wine statistics: {:?}", red_stats);
    println!("White wine statistics: {:?}", white_stats);

    let red_weights = train_logistic_model(&red_wine_data);
    let white_weights = train_logistic_model(&white_wine_data);

    println!("Red wine feature importance: {:?}", compute_feature_importance(&red_weights));
    println!("White wine feature importance: {:?}", compute_feature_importance(&white_weights));

    let red_predictions = predict(&red_wine_data, &red_weights);
    let white_predictions = predict(&white_wine_data, &white_weights);

    println!("Predictions for red wine: {:?}", red_predictions);
    println!("Predictions for white wine: {:?}", white_predictions);

    let feature_comparison = compare_features(&red_weights, &white_weights);
    println!("Feature comparison: {:?}", feature_comparison);
}