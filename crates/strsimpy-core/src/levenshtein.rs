//! Levenshtein edit distance (Wagner-Fischer algorithm).

/// Levenshtein distance between two strings.
///
/// The result is the minimum number of single-character insertions, deletions
/// and substitutions needed to transform `s0` into `s1`. The implementation
/// only keeps two rows, so its space usage is O(m).
pub fn distance(s0: &str, s1: &str) -> f64 {
    let a: Vec<char> = s0.chars().collect();
    let b: Vec<char> = s1.chars().collect();

    if a == b {
        return 0.0;
    }
    if a.is_empty() {
        return b.len() as f64;
    }
    if b.is_empty() {
        return a.len() as f64;
    }

    let mut previous: Vec<usize> = (0..=b.len()).collect();
    let mut current = vec![0usize; b.len() + 1];

    for (i, &ch0) in a.iter().enumerate() {
        current[0] = i + 1;
        for (j, &ch1) in b.iter().enumerate() {
            let substitution = previous[j] + usize::from(ch0 != ch1);
            let insertion = current[j] + 1;
            let deletion = previous[j + 1] + 1;
            current[j + 1] = substitution.min(insertion).min(deletion);
        }
        std::mem::swap(&mut previous, &mut current);
    }

    previous[b.len()] as f64
}

/// A [`Levenshtein`] distance calculator.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Levenshtein;

impl Levenshtein {
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
    fn empty_and_equal_strings() {
        assert_eq!(distance("", ""), 0.0);
        assert_eq!(distance("", "abc"), 3.0);
        assert_eq!(distance("abc", ""), 3.0);
        assert_eq!(distance("abc", "abc"), 0.0);
    }

    #[test]
    fn classic_examples() {
        assert_eq!(distance("kitten", "sitting"), 3.0);
        assert_eq!(distance("flaw", "lawn"), 2.0);
        assert_eq!(distance("Saturday", "Sunday"), 3.0);
        // LCS distance between AGCAT and GAC is 4; Levenshtein is 3.
        assert_eq!(distance("AGCAT", "GAC"), 3.0);
    }

    #[test]
    fn unicode_is_counted_in_code_points() {
        assert_eq!(distance("上海", "上海市"), 1.0);
        assert_eq!(distance("", "上海市"), 3.0);
        assert_eq!(distance("😀", "😁"), 1.0);
    }

    #[test]
    fn struct_delegates_to_function() {
        assert_eq!(Levenshtein::new().distance("abc", "abd"), 1.0);
    }
}
