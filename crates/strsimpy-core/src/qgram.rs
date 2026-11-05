//! Q-Gram distance (Ukkonen).

use std::collections::{HashMap, HashSet};

use crate::shingle;

/// Q-Gram distance: the L1 norm of the difference between the shingle
/// profiles of the two strings.
pub fn distance(s0: &str, s1: &str, k: usize) -> usize {
    if s0 == s1 {
        return 0;
    }
    let profile0 = shingle::profile(s0, k);
    let profile1 = shingle::profile(s1, k);
    distance_profile(&profile0, &profile1)
}

/// Computes the Q-Gram distance between two precomputed profiles.
pub fn distance_profile(
    profile0: &HashMap<String, usize>,
    profile1: &HashMap<String, usize>,
) -> usize {
    let union: HashSet<&String> = profile0.keys().chain(profile1.keys()).collect();
    union
        .into_iter()
        .map(|shingle| {
            let count0 = profile0.get(shingle).copied().unwrap_or(0) as i64;
            let count1 = profile1.get(shingle).copied().unwrap_or(0) as i64;
            (count0 - count1).unsigned_abs() as usize
        })
        .sum()
}

/// A [`QGram`] distance calculator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QGram {
    k: usize,
}

impl QGram {
    pub fn new(k: usize) -> Self {
        Self { k }
    }

    pub fn k(&self) -> usize {
        self.k
    }

    pub fn profile(&self, string: &str) -> HashMap<String, usize> {
        shingle::profile(string, self.k)
    }

    pub fn distance(&self, s0: &str, s1: &str) -> usize {
        distance(s0, s1, self.k)
    }
}

impl Default for QGram {
    fn default() -> Self {
        Self::new(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_example() {
        assert_eq!(distance("ABCD", "ABCE", 2), 2);
    }

    #[test]
    fn empty_and_equal_strings() {
        assert_eq!(distance("", "", 2), 0);
        assert_eq!(distance("abc", "abc", 2), 0);
        assert_eq!(distance("", "ab", 2), 1);
    }

    #[test]
    fn profile_based_distance_matches_direct_distance() {
        let qgram = QGram::new(2);
        let p0 = qgram.profile("night");
        let p1 = qgram.profile("nacht");
        assert_eq!(qgram.distance("night", "nacht"), distance_profile(&p0, &p1));
    }

    #[test]
    fn unicode() {
        assert_eq!(distance("上海", "上海市", 1), 1);
    }
}
