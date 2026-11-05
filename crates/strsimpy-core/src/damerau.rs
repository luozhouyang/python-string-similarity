//! Unrestricted Damerau-Levenshtein distance.

use std::collections::HashMap;

/// Damerau-Levenshtein distance with adjacent transpositions.
///
/// Unlike [`optimal_string_alignment`](crate::optimal_string_alignment), no
/// substring is restricted to a single edit. The result is a metric.
pub fn distance(s0: &str, s1: &str) -> f64 {
    let a: Vec<char> = s0.chars().collect();
    let b: Vec<char> = s1.chars().collect();

    if a == b {
        return 0.0;
    }

    let n = a.len();
    let m = b.len();
    let inf = n + m;

    let mut da: HashMap<char, usize> = HashMap::with_capacity(n + m);
    for &c in &a {
        da.insert(c, 0);
    }
    for &c in &b {
        da.insert(c, 0);
    }

    let mut h = vec![vec![0usize; m + 2]; n + 2];
    for i in 0..=n {
        h[i + 1][0] = inf;
        h[i + 1][1] = i;
    }
    for j in 0..=m {
        h[0][j + 1] = inf;
        h[1][j + 1] = j;
    }

    for i in 1..=n {
        let mut db = 0usize;
        for j in 1..=m {
            let i1 = *da.get(&b[j - 1]).unwrap_or(&0);
            let j1 = db;

            let cost;
            if a[i - 1] == b[j - 1] {
                cost = 0;
                db = j;
            } else {
                cost = 1;
            }

            h[i + 1][j + 1] = (h[i][j] + cost)
                .min(h[i + 1][j] + 1)
                .min(h[i][j + 1] + 1)
                .min(h[i1][j1] + (i - i1 - 1) + 1 + (j - j1 - 1));
        }
        da.insert(a[i - 1], i);
    }

    h[n + 1][m + 1] as f64
}

/// A [`Damerau`] distance calculator.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Damerau;

impl Damerau {
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
        assert_eq!(distance("ABCDEF", "ABDCEF"), 1.0);
        assert_eq!(distance("ABCDEF", "BACDFE"), 2.0);
        assert_eq!(distance("ABCDEF", "ABCDE"), 1.0);
        assert_eq!(distance("ABCDEF", "BCDEF"), 1.0);
        assert_eq!(distance("ABCDEF", "ABCGDEF"), 1.0);
        assert_eq!(distance("ABCDEF", "POIU"), 6.0);
    }

    #[test]
    fn empty_and_equal_strings() {
        assert_eq!(distance("", ""), 0.0);
        assert_eq!(distance("", "abc"), 3.0);
        assert_eq!(distance("abc", ""), 3.0);
        assert_eq!(distance("abc", "abc"), 0.0);
    }

    #[test]
    fn transposition_is_one_edit() {
        assert_eq!(distance("CA", "AC"), 1.0);
        assert_eq!(distance("上海", "海上海"), 1.0);
    }

    #[test]
    fn struct_delegates_to_function() {
        assert_eq!(Damerau::new().distance("CA", "AC"), 1.0);
    }
}
