//! Cosine similarity and distance over shingle profiles.

use std::collections::HashMap;

use crate::shingle;

/// Cosine similarity between the shingle profiles of two strings.
pub fn similarity(s0: &str, s1: &str, k: usize) -> f64 {
    if s0 == s1 {
        return 1.0;
    }
    if s0.chars().count() < k || s1.chars().count() < k {
        return 0.0;
    }
    let profile0: HashMap<String, f64> = shingle::profile(s0, k)
        .into_iter()
        .map(|(shingle, count)| (shingle, count as f64))
        .collect();
    let profile1: HashMap<String, f64> = shingle::profile(s1, k)
        .into_iter()
        .map(|(shingle, count)| (shingle, count as f64))
        .collect();
    similarity_profiles(&profile0, &profile1)
}

/// `1.0 - similarity(s0, s1, k)`.
pub fn distance(s0: &str, s1: &str, k: usize) -> f64 {
    1.0 - similarity(s0, s1, k)
}

/// Cosine similarity between two precomputed profiles.
pub fn similarity_profiles(
    profile0: &HashMap<String, f64>,
    profile1: &HashMap<String, f64>,
) -> f64 {
    let norm0 = norm(profile0);
    let norm1 = norm(profile1);
    if norm0 == 0.0 || norm1 == 0.0 {
        return 0.0;
    }
    dot_product(profile0, profile1) / (norm0 * norm1)
}

fn dot_product(profile0: &HashMap<String, f64>, profile1: &HashMap<String, f64>) -> f64 {
    let (small, large) = if profile0.len() < profile1.len() {
        (profile0, profile1)
    } else {
        (profile1, profile0)
    };
    let mut aggregate = 0.0;
    for (shingle, &value) in small {
        if value == 0.0 {
            continue;
        }
        if let Some(&other) = large.get(shingle) {
            aggregate += value * other;
        }
    }
    aggregate
}

fn norm(profile: &HashMap<String, f64>) -> f64 {
    profile
        .values()
        .map(|value| value * value)
        .sum::<f64>()
        .sqrt()
}

/// A [`Cosine`] similarity calculator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cosine {
    k: usize,
}

impl Cosine {
    pub fn new(k: usize) -> Self {
        Self { k }
    }

    pub fn k(&self) -> usize {
        self.k
    }

    pub fn profile(&self, string: &str) -> HashMap<String, usize> {
        shingle::profile(string, self.k)
    }

    pub fn similarity(&self, s0: &str, s1: &str) -> f64 {
        similarity(s0, s1, self.k)
    }

    pub fn distance(&self, s0: &str, s1: &str) -> f64 {
        distance(s0, s1, self.k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings_are_similar() {
        assert_eq!(similarity("Shanghai", "Shanghai", 1), 1.0);
        assert_eq!(similarity("ShangHai", "ShangHai", 1), 1.0);
        assert!((similarity("Shang  Hai", "Shang Hai", 4) - 1.0).abs() < 1e-12);
        assert_eq!(similarity("", "", 3), 1.0);
    }

    #[test]
    fn short_strings_have_zero_similarity() {
        assert_eq!(similarity("a", "b", 2), 0.0);
        assert_eq!(distance("a", "b", 2), 1.0);
    }

    #[test]
    fn whitespace_normalization_can_empty_a_profile() {
        assert_eq!(similarity("    ", "foo bar", 4), 0.0);
        assert_eq!(distance("    ", "foo bar", 4), 1.0);
    }

    #[test]
    fn profile_similarity_handles_zero_norms() {
        let mut empty = HashMap::new();
        assert_eq!(similarity_profiles(&empty, &empty), 0.0);

        empty.insert("test".to_string(), 0.0);
        let mut other = HashMap::new();
        other.insert("test".to_string(), 1.0);
        assert_eq!(similarity_profiles(&empty, &other), 0.0);
        assert_eq!(similarity_profiles(&other, &empty), 0.0);
    }

    #[test]
    fn profile_similarity_values() {
        let mut p0 = HashMap::new();
        p0.insert("test".to_string(), 2.0);
        let mut p1 = HashMap::new();
        p1.insert("test".to_string(), 3.0);
        assert!((similarity_profiles(&p0, &p1) - 1.0).abs() < 1e-12);

        let mut p2 = HashMap::new();
        p2.insert("abcd".to_string(), 1.0);
        let mut p3 = HashMap::new();
        p3.insert("efgh".to_string(), 1.0);
        assert_eq!(similarity_profiles(&p2, &p3), 0.0);
    }
}
