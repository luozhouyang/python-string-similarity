//! Optimal String Alignment (restricted Damerau-Levenshtein) distance.

/// Optimal String Alignment distance.
///
/// This is the restricted edit distance where no substring may be edited more
/// than once. Empty strings are handled correctly: the distance from an empty
/// string to a string of length `n` is `n`.
pub fn distance(s0: &str, s1: &str) -> f64 {
    let a: Vec<char> = s0.chars().collect();
    let b: Vec<char> = s1.chars().collect();

    if a == b {
        return 0.0;
    }

    let n = a.len();
    let m = b.len();
    if n == 0 {
        return m as f64;
    }
    if m == 0 {
        return n as f64;
    }

    let mut d = vec![vec![0usize; m + 1]; n + 1];
    for (i, row) in d.iter_mut().enumerate().take(n + 1) {
        row[0] = i;
    }
    for (j, cell) in d[0].iter_mut().enumerate() {
        *cell = j;
    }

    for i in 1..=n {
        for j in 1..=m {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            d[i][j] = (d[i - 1][j - 1] + cost)
                .min(d[i][j - 1] + 1)
                .min(d[i - 1][j] + 1);

            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                d[i][j] = d[i][j].min(d[i - 2][j - 2] + cost);
            }
        }
    }

    d[n][m] as f64
}

/// An [`OptimalStringAlignment`] calculator.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OptimalStringAlignment;

impl OptimalStringAlignment {
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
    fn readme_example() {
        assert_eq!(distance("CA", "ABC"), 3.0);
    }

    #[test]
    fn empty_strings_have_their_length_as_distance() {
        assert_eq!(distance("", ""), 0.0);
        assert_eq!(distance("", "abc"), 3.0);
        assert_eq!(distance("abc", ""), 3.0);
    }

    #[test]
    fn adjacent_transposition() {
        assert_eq!(distance("CA", "AC"), 1.0);
        assert_eq!(distance("abcd", "abdc"), 1.0);
    }

    #[test]
    fn struct_delegates_to_function() {
        assert_eq!(OptimalStringAlignment::new().distance("CA", "ABC"), 3.0);
    }
}
