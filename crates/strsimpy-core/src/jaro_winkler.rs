//! Jaro-Winkler similarity and distance.

/// Default prefix scaling threshold.
pub const DEFAULT_THRESHOLD: f64 = 0.7;
const PREFIX_SCALE: f64 = 0.1;

pub(crate) struct MatchDetails {
    pub matches: usize,
    pub transpositions: usize,
    pub prefix: usize,
    pub max_len: usize,
}

/// Jaro-Winkler similarity with the default threshold (0.7).
pub fn similarity(s0: &str, s1: &str) -> f64 {
    similarity_with_threshold(s0, s1, DEFAULT_THRESHOLD)
}

/// Jaro-Winkler similarity with a configurable prefix threshold.
pub fn similarity_with_threshold(s0: &str, s1: &str, threshold: f64) -> f64 {
    if s0 == s1 {
        return 1.0;
    }
    let details = matches(s0, s1);
    if details.matches == 0 {
        return 0.0;
    }

    let m = details.matches as f64;
    let len0 = s0.chars().count() as f64;
    let len1 = s1.chars().count() as f64;
    let jaro_similarity = (m / len0 + m / len1 + (m - details.transpositions as f64) / m) / 3.0;

    if jaro_similarity > threshold {
        jaro_similarity
            + PREFIX_SCALE.min(1.0 / details.max_len as f64)
                * details.prefix as f64
                * (1.0 - jaro_similarity)
    } else {
        jaro_similarity
    }
}

/// `1.0 - Jaro-Winkler similarity`.
pub fn distance(s0: &str, s1: &str) -> f64 {
    1.0 - similarity(s0, s1)
}

/// Jaro-Winkler distance with a configurable prefix threshold.
pub fn distance_with_threshold(s0: &str, s1: &str, threshold: f64) -> f64 {
    1.0 - similarity_with_threshold(s0, s1, threshold)
}

pub(crate) fn matches(s0: &str, s1: &str) -> MatchDetails {
    let chars0: Vec<char> = s0.chars().collect();
    let chars1: Vec<char> = s1.chars().collect();

    let (max_str, min_str): (&[char], &[char]) = if chars0.len() > chars1.len() {
        (&chars0, &chars1)
    } else {
        (&chars1, &chars0)
    };

    let max_len = max_str.len();
    let min_len = min_str.len();
    let range = if max_len >= 2 { max_len / 2 - 1 } else { 0 };

    let mut match_indexes = vec![-1isize; min_len];
    let mut match_flags = vec![false; max_len];
    let mut matched = 0usize;

    for mi in 0..min_len {
        let c1 = min_str[mi];
        let start = mi.saturating_sub(range);
        let end = (mi + range + 1).min(max_len);
        for xi in start..end {
            if !match_flags[xi] && c1 == max_str[xi] {
                match_indexes[mi] = xi as isize;
                match_flags[xi] = true;
                matched += 1;
                break;
            }
        }
    }

    let mut ms0 = Vec::with_capacity(matched);
    for (mi, &index) in match_indexes.iter().enumerate() {
        if index != -1 {
            ms0.push(min_str[mi]);
        }
    }
    let mut ms1 = Vec::with_capacity(matched);
    for (index, &flag) in match_flags.iter().enumerate() {
        if flag {
            ms1.push(max_str[index]);
        }
    }

    let transpositions = ms0
        .iter()
        .zip(ms1.iter())
        .filter(|(left, right)| left != right)
        .count()
        / 2;

    let mut prefix = 0usize;
    for mi in 0..min_len {
        if chars0[mi] == chars1[mi] {
            prefix += 1;
        } else {
            break;
        }
    }

    MatchDetails {
        matches: matched,
        transpositions,
        prefix,
        max_len,
    }
}

/// A [`JaroWinkler`] similarity calculator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JaroWinkler {
    threshold: f64,
}

impl JaroWinkler {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    pub fn similarity(&self, s0: &str, s1: &str) -> f64 {
        similarity_with_threshold(s0, s1, self.threshold)
    }

    pub fn distance(&self, s0: &str, s1: &str) -> f64 {
        distance_with_threshold(s0, s1, self.threshold)
    }
}

impl Default for JaroWinkler {
    fn default() -> Self {
        Self::new(DEFAULT_THRESHOLD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-12,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn readme_examples() {
        assert_close(similarity("My string", "My tsring"), 0.9740740740740741);
        assert_close(similarity("My string", "My ntrisg"), 0.8962962962962963);
    }

    #[test]
    fn distance_is_one_minus_similarity() {
        let cases = [
            ("", ""),
            ("", "a"),
            ("a", "a"),
            ("martha", "marhta"),
            ("dwayne", "duane"),
            ("dixon", "dicksonx"),
            ("上海", "上海市"),
        ];
        for (a, b) in cases {
            assert_close(distance(a, b), 1.0 - similarity(a, b));
        }
    }

    #[test]
    fn identical_and_disjoint_strings() {
        assert_eq!(similarity("", ""), 1.0);
        assert_eq!(similarity("abc", "abc"), 1.0);
        assert_eq!(similarity("abc", "xyz"), 0.0);
    }

    #[test]
    fn threshold_is_configurable() {
        let default = JaroWinkler::default();
        assert_eq!(default.threshold(), 0.7);
        let custom = JaroWinkler::new(0.0);
        assert!(custom.similarity("martha", "marhta") >= default.similarity("martha", "marhta"));
    }

    #[test]
    fn known_jaro_values() {
        assert_close(similarity("martha", "marhta"), 0.9611111111111111);
        assert_close(similarity("dwayne", "duane"), 0.8400000000000001);
        assert_close(similarity("dixon", "dicksonx"), 0.8133333333333332);
    }
}
