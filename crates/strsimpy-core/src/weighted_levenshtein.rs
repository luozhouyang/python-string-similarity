//! Weighted Levenshtein distance.
//!
//! Costs are supplied through the [`WeightedCosts`] trait so that pure Rust
//! users can use plain closures and Python bindings can call Python callables
//! while propagating their exceptions.

use std::convert::Infallible;

/// Per-operation cost functions used by [`distance_with_costs`].
pub trait WeightedCosts {
    type Error;

    fn insertion_cost(&self, c: char) -> Result<f64, Self::Error>;
    fn deletion_cost(&self, c: char) -> Result<f64, Self::Error>;
    fn substitution_cost(&self, a: char, b: char) -> Result<f64, Self::Error>;
}

/// The default cost scheme: every operation costs 1.0.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DefaultCosts;

impl WeightedCosts for DefaultCosts {
    type Error = Infallible;

    fn insertion_cost(&self, _c: char) -> Result<f64, Self::Error> {
        Ok(1.0)
    }

    fn deletion_cost(&self, _c: char) -> Result<f64, Self::Error> {
        Ok(1.0)
    }

    fn substitution_cost(&self, _a: char, _b: char) -> Result<f64, Self::Error> {
        Ok(1.0)
    }
}

/// Weighted Levenshtein distance with the default unit costs.
pub fn distance(s0: &str, s1: &str) -> f64 {
    distance_with_costs(s0, s1, &DefaultCosts).unwrap_or(f64::INFINITY)
}

/// Weighted Levenshtein distance using the supplied cost functions.
pub fn distance_with_costs<C: WeightedCosts>(
    s0: &str,
    s1: &str,
    costs: &C,
) -> Result<f64, C::Error> {
    let a: Vec<char> = s0.chars().collect();
    let b: Vec<char> = s1.chars().collect();

    if a == b {
        return Ok(0.0);
    }
    if a.is_empty() {
        let mut total = 0.0;
        for &c in &b {
            total += costs.insertion_cost(c)?;
        }
        return Ok(total);
    }
    if b.is_empty() {
        let mut total = 0.0;
        for &c in &a {
            total += costs.deletion_cost(c)?;
        }
        return Ok(total);
    }

    let mut previous = vec![0.0f64; b.len() + 1];
    let mut current = vec![0.0f64; b.len() + 1];

    for (i, &s1_char) in b.iter().enumerate() {
        previous[i + 1] = previous[i] + costs.insertion_cost(s1_char)?;
    }

    for &s0_char in &a {
        let deletion_cost = costs.deletion_cost(s0_char)?;
        current[0] = previous[0] + deletion_cost;

        for (j, &s1_char) in b.iter().enumerate() {
            let substitution_cost = if s0_char == s1_char {
                0.0
            } else {
                costs.substitution_cost(s0_char, s1_char)?
            };
            let insertion_cost = costs.insertion_cost(s1_char)?;
            current[j + 1] = (current[j] + insertion_cost)
                .min(previous[j + 1] + deletion_cost)
                .min(previous[j] + substitution_cost);
        }

        std::mem::swap(&mut previous, &mut current);
    }

    Ok(previous[b.len()])
}

/// A [`WeightedLevenshtein`] calculator with unit costs.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WeightedLevenshtein;

impl WeightedLevenshtein {
    pub fn new() -> Self {
        Self
    }

    pub fn distance(&self, s0: &str, s1: &str) -> f64 {
        distance(s0, s1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct HalfForTR;

    impl WeightedCosts for HalfForTR {
        type Error = Infallible;

        fn insertion_cost(&self, _c: char) -> Result<f64, Self::Error> {
            Ok(1.0)
        }

        fn deletion_cost(&self, _c: char) -> Result<f64, Self::Error> {
            Ok(1.0)
        }

        fn substitution_cost(&self, a: char, b: char) -> Result<f64, Self::Error> {
            if (a == 't' && b == 'r') || (a == 'r' && b == 't') {
                Ok(0.5)
            } else {
                Ok(1.0)
            }
        }
    }

    #[test]
    fn defaults_match_levenshtein() {
        assert_eq!(distance("kitten", "sitting"), 3.0);
        assert_eq!(distance("", "abc"), 3.0);
        assert_eq!(distance("abc", ""), 3.0);
        assert_eq!(distance("abc", "abc"), 0.0);
    }

    #[test]
    fn custom_substitution_costs_are_used() {
        let weighted = distance_with_costs("String1", "String2", &HalfForTR).unwrap();
        assert_eq!(weighted, 1.0);

        let weighted = distance_with_costs("tr", "rt", &HalfForTR).unwrap();
        assert_eq!(weighted, 1.0); // two half-cost substitutions
    }

    #[test]
    fn unicode_costs() {
        let weighted = distance_with_costs("上海", "上海市", &HalfForTR).unwrap();
        assert_eq!(weighted, 1.0);
    }

    #[test]
    fn struct_delegates_to_function() {
        assert_eq!(WeightedLevenshtein::new().distance("abc", "abd"), 1.0);
    }
}
