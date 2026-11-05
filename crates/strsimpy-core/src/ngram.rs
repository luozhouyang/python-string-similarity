//! Normalized N-Gram distance (Kondrak, 2005).

/// Normalized N-Gram distance between two strings.
///
/// The implementation follows Kondrak's algorithm as used by
/// `java-string-similarity`: the first string is prefixed with `n - 1`
/// newline padding characters and the edit distance between n-gram profiles
/// is normalized by the length of the longest input string.
pub fn distance(s0: &str, s1: &str, n: usize) -> f64 {
    assert!(n > 0, "n must be greater than zero");

    let a: Vec<char> = s0.chars().collect();
    let b: Vec<char> = s1.chars().collect();

    if a == b {
        return 0.0;
    }

    let sl = a.len();
    let tl = b.len();
    if sl == 0 || tl == 0 {
        return 1.0;
    }

    if sl < n || tl < n {
        let mut matched = 0usize;
        for i in 0..sl.min(tl) {
            if a[i] == b[i] {
                matched += 1;
            }
        }
        return 1.0 - matched as f64 / sl.max(tl) as f64;
    }

    const SPECIAL: char = '\n';

    let mut sa = vec![SPECIAL; sl + n - 1];
    sa[n - 1..].copy_from_slice(&a);

    let mut previous: Vec<f64> = (0..=sl).map(|i| i as f64).collect();
    let mut current = vec![0.0f64; sl + 1];
    let mut t_j = vec![SPECIAL; n];

    for j in 1..=tl {
        if j < n {
            let padding = n - j;
            t_j[..padding].fill(SPECIAL);
            t_j[padding..n].copy_from_slice(&b[..n - padding]);
        } else {
            t_j.copy_from_slice(&b[j - n..j]);
        }

        current[0] = j as f64;
        for i in 1..=sl {
            let mut cost = 0usize;
            let mut denominator = n;
            for ni in 0..n {
                let sc = sa[i - 1 + ni];
                if sc != t_j[ni] {
                    cost += 1;
                } else if sc == SPECIAL {
                    denominator -= 1;
                }
            }
            let edit_cost = cost as f64 / denominator as f64;
            current[i] = (current[i - 1] + 1.0)
                .min(previous[i] + 1.0)
                .min(previous[i - 1] + edit_cost);
        }

        std::mem::swap(&mut previous, &mut current);
    }

    previous[sl] / sl.max(tl) as f64
}

/// An [`NGram`] distance calculator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NGram {
    n: usize,
}

impl NGram {
    /// Creates an N-Gram calculator. `n` must be greater than zero.
    pub fn new(n: usize) -> Self {
        assert!(n > 0, "n must be greater than zero");
        Self { n }
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn distance(&self, s0: &str, s1: &str) -> f64 {
        distance(s0, s1, self.n)
    }
}

impl Default for NGram {
    fn default() -> Self {
        Self::new(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_values() {
        assert!((distance("ABCD", "ABTUIO", 2) - 0.5833333333333334).abs() < 1e-12);
        assert!((distance("ABABABAB", "ABCABCABCABC", 2) - 0.4583333333333333).abs() < 1e-12);
        assert!((distance("ABABABAB", "POIULKJH", 2) - 1.0).abs() < 1e-12);
        assert!((distance("上海", "上海市", 2) - 1.0 / 3.0).abs() < 1e-12);
        assert!((distance("ShangHai", "Shang Hai", 2) - 1.0 / 6.0).abs() < 1e-12);
    }

    #[test]
    fn equal_and_empty_strings() {
        assert_eq!(distance("AB", "AB", 2), 0.0);
        assert_eq!(distance("", "", 2), 0.0);
        assert_eq!(distance("", "a", 2), 1.0);
        assert_eq!(distance("a", "", 4), 1.0);
    }

    #[test]
    fn short_string_branch() {
        assert!((distance("a", "b", 3) - 1.0).abs() < 1e-12);
        assert!((distance("ab", "ac", 3) - 0.5).abs() < 1e-12);
    }

    #[test]
    fn ordering_matches_upstream_java_test() {
        let ngram = NGram::default();
        assert!(
            ngram.distance("ABABABAB", "ABCABCABCABC") < ngram.distance("ABABABAB", "POIULKJH")
        );
    }
}
