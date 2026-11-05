//! Metric Longest Common Subsequence distance.

use crate::longest_common_subsequence;

/// `1 - LCS(s0, s1) / max(len(s0), len(s1))`.
pub fn distance(s0: &str, s1: &str) -> f64 {
    if s0 == s1 {
        return 0.0;
    }
    let max_len = s0.chars().count().max(s1.chars().count());
    if max_len == 0 {
        return 0.0;
    }
    1.0 - longest_common_subsequence::length(s0, s1) as f64 / max_len as f64
}

/// A [`MetricLCS`] distance calculator.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MetricLCS;

impl MetricLCS {
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

    #[test]
    fn readme_examples() {
        assert!((distance("ABCDEFG", "ABCDEFHJKL") - 0.4).abs() < 1e-12);
        assert!((distance("ABDEF", "ABDIF") - 0.2).abs() < 1e-12);
    }

    #[test]
    fn empty_and_equal_strings() {
        assert_eq!(distance("", ""), 0.0);
        assert_eq!(distance("abc", "abc"), 0.0);
        assert_eq!(distance("", "abc"), 1.0);
    }

    #[test]
    fn unicode() {
        assert!((distance("上海", "上海市") - 1.0 / 3.0).abs() < 1e-12);
    }

    #[test]
    fn struct_delegates_to_function() {
        assert_eq!(MetricLCS::new().distance("", "abc"), 1.0);
    }
}
