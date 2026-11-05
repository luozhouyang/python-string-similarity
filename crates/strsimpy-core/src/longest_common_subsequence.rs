//! Longest Common Subsequence length and distance.

/// Length of the longest common subsequence of `s0` and `s1`.
///
/// Only two rolling rows are kept, so the space usage is O(m).
pub fn length(s0: &str, s1: &str) -> usize {
    let a: Vec<char> = s0.chars().collect();
    let b: Vec<char> = s1.chars().collect();

    let mut previous = vec![0usize; b.len() + 1];
    let mut current = vec![0usize; b.len() + 1];

    for &ch0 in &a {
        for (j, &ch1) in b.iter().enumerate() {
            current[j + 1] = if ch0 == ch1 {
                previous[j] + 1
            } else {
                previous[j + 1].max(current[j])
            };
        }
        std::mem::swap(&mut previous, &mut current);
    }

    previous[b.len()]
}

/// LCS distance: `len(s0) + len(s1) - 2 * length(s0, s1)`.
pub fn distance(s0: &str, s1: &str) -> f64 {
    if s0 == s1 {
        return 0.0;
    }
    let len0 = s0.chars().count();
    let len1 = s1.chars().count();
    (len0 + len1 - 2 * length(s0, s1)) as f64
}

/// A [`LongestCommonSubsequence`] calculator.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LongestCommonSubsequence;

impl LongestCommonSubsequence {
    pub fn new() -> Self {
        Self
    }

    pub fn length(&self, s0: &str, s1: &str) -> usize {
        length(s0, s1)
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
        assert_eq!(length("AGCAT", "GAC"), 2);
        assert_eq!(distance("AGCAT", "GAC"), 4.0);
        assert_eq!(length("AGCAT", "AGCT"), 4);
        assert_eq!(distance("AGCAT", "AGCT"), 1.0);
    }

    #[test]
    fn unicode_examples() {
        assert_eq!(length("上海", "上海市"), 2);
        assert_eq!(distance("上海", "上海市"), 1.0);
        assert_eq!(distance("", "上海市"), 3.0);
    }

    #[test]
    fn empty_and_equal_strings() {
        assert_eq!(length("", ""), 0);
        assert_eq!(length("abc", ""), 0);
        assert_eq!(distance("abc", "abc"), 0.0);
    }

    #[test]
    fn struct_delegates_to_functions() {
        let lcs = LongestCommonSubsequence::new();
        assert_eq!(lcs.length("AGCAT", "GAC"), 2);
        assert_eq!(lcs.distance("AGCAT", "GAC"), 4.0);
    }
}
