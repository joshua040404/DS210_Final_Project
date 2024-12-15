#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data() {
        let data = load_data("src/winequality-red.csv");
        assert!(!data.is_empty());
    }

    #[test]
    fn test_compute_statistics() {
        let data = load_data("src/winequality-red.csv");
        let stats = compute_statistics(&data);
        assert!(stats.contains_key("avg_alcohol"));
        assert!(stats.contains_key("avg_quality"));
    }
}