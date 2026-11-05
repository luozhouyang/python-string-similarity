//! SIFT4 string distance.
//!
//! SIFT4 is a general-purpose string distance inspired by Jaro-Winkler and
//! Longest Common Subsequence. The algorithm is generic over a [`Sift4Options`]
//! implementation so the Python bindings can plug in tokenizers and evaluators
//! supplied at runtime.

use std::convert::Infallible;

/// Options and callbacks used by the SIFT4 algorithm.
pub trait Sift4Options {
    type Token;
    type Error;

    fn max_distance(&self) -> f64;
    fn tokenize(&self, string: &str) -> Result<Vec<Self::Token>, Self::Error>;
    fn token_match(&self, token1: &Self::Token, token2: &Self::Token) -> Result<bool, Self::Error>;
    fn matching_evaluator(
        &self,
        token1: &Self::Token,
        token2: &Self::Token,
    ) -> Result<f64, Self::Error>;
    fn local_length_evaluator(&self, length: f64) -> Result<f64, Self::Error>;
    fn transposition_cost_evaluator(&self, c1: f64, c2: f64) -> Result<f64, Self::Error>;
    fn transpositions_evaluator(&self, lcss: f64, transpositions: f64) -> Result<f64, Self::Error>;
}

/// The default SIFT4 options: character tokens, equality matching, unit
/// transposition costs and no maximum distance cutoff.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DefaultOptions;

impl Sift4Options for DefaultOptions {
    type Token = char;
    type Error = Infallible;

    fn max_distance(&self) -> f64 {
        0.0
    }

    fn tokenize(&self, string: &str) -> Result<Vec<char>, Infallible> {
        Ok(string.chars().collect())
    }

    fn token_match(&self, token1: &char, token2: &char) -> Result<bool, Infallible> {
        Ok(token1 == token2)
    }

    fn matching_evaluator(&self, _t1: &char, _t2: &char) -> Result<f64, Infallible> {
        Ok(1.0)
    }

    fn local_length_evaluator(&self, length: f64) -> Result<f64, Infallible> {
        Ok(length)
    }

    fn transposition_cost_evaluator(&self, _c1: f64, _c2: f64) -> Result<f64, Infallible> {
        Ok(1.0)
    }

    fn transpositions_evaluator(&self, lcss: f64, transpositions: f64) -> Result<f64, Infallible> {
        Ok(lcss - transpositions)
    }
}

/// SIFT4 distance with the default options.
pub fn distance(s1: &str, s2: &str, max_offset: i64) -> f64 {
    distance_with_options(s1, s2, max_offset, &DefaultOptions).unwrap_or(f64::NAN)
}

#[derive(Debug, Clone, Copy)]
struct Offset {
    c1: isize,
    c2: isize,
    transposed: bool,
}

/// SIFT4 distance using the supplied options.
pub fn distance_with_options<O: Sift4Options>(
    s1: &str,
    s2: &str,
    max_offset: i64,
    options: &O,
) -> Result<f64, O::Error> {
    let tokens1 = options.tokenize(s1)?;
    let tokens2 = options.tokenize(s2)?;
    let l1 = tokens1.len() as isize;
    let l2 = tokens2.len() as isize;

    if l1 == 0 {
        return Ok(l2 as f64);
    }
    if l2 == 0 {
        return Ok(l1 as f64);
    }

    let mut c1: isize = 0;
    let mut c2: isize = 0;
    let mut lcss = 0.0f64;
    let mut local_cs = 0.0f64;
    let mut transpositions = 0.0f64;
    let mut offsets: Vec<Offset> = Vec::new();

    while c1 < l1 && c2 < l2 {
        if options.token_match(&tokens1[c1 as usize], &tokens2[c2 as usize])? {
            local_cs += options.matching_evaluator(&tokens1[c1 as usize], &tokens2[c2 as usize])?;

            let mut is_transposition = false;
            let mut i = 0usize;
            while i < offsets.len() {
                let offset = offsets[i];
                if c1 <= offset.c1 || c2 <= offset.c2 {
                    is_transposition = (c2 - c1).abs() >= (offset.c2 - offset.c1).abs();
                    if is_transposition {
                        transpositions +=
                            options.transposition_cost_evaluator(c1 as f64, c2 as f64)?;
                    } else if !offset.transposed {
                        offsets[i].transposed = true;
                        transpositions += options
                            .transposition_cost_evaluator(offset.c1 as f64, offset.c2 as f64)?;
                    }
                    break;
                } else if c1 > offset.c2 && c2 > offset.c1 {
                    offsets.remove(i);
                } else {
                    i += 1;
                }
            }

            offsets.push(Offset {
                c1,
                c2,
                transposed: is_transposition,
            });
        } else {
            lcss += options.local_length_evaluator(local_cs)?;
            local_cs = 0.0;

            if c1 != c2 {
                let minimum = c1.min(c2);
                c1 = minimum;
                c2 = minimum;
            }

            for i in 0..max_offset {
                let offset = i as isize;
                if c1 + offset >= l1 && c2 + offset >= l2 {
                    continue;
                }
                if c1 + offset < l1
                    && options
                        .token_match(&tokens1[(c1 + offset) as usize], &tokens2[c2 as usize])?
                {
                    c1 += offset - 1;
                    c2 -= 1;
                    break;
                }
                if c2 + offset < l2
                    && options
                        .token_match(&tokens1[c1 as usize], &tokens2[(c2 + offset) as usize])?
                {
                    c1 -= 1;
                    c2 += offset - 1;
                    break;
                }
            }
        }

        c1 += 1;
        c2 += 1;

        let max_distance = options.max_distance();
        if max_distance != 0.0 {
            let temporary_distance = options.local_length_evaluator(c1.max(c2) as f64)?
                - options.transpositions_evaluator(lcss, transpositions)?;
            if temporary_distance >= max_distance {
                return Ok(round_ties_even(temporary_distance));
            }
        }

        if c1 >= l1 || c2 >= l2 {
            lcss += options.local_length_evaluator(local_cs)?;
            local_cs = 0.0;
            let minimum = c1.min(c2);
            c1 = minimum;
            c2 = minimum;
        }
    }

    lcss += options.local_length_evaluator(local_cs)?;
    let result = options.local_length_evaluator(l1.max(l2) as f64)?
        - options.transpositions_evaluator(lcss, transpositions)?;
    Ok(round_ties_even(result))
}

/// Python's `round()` uses banker's rounding (ties to even).
pub(crate) fn round_ties_even(value: f64) -> f64 {
    if value.is_finite() {
        value.round_ties_even()
    } else {
        value
    }
}

/// A [`SIFT4`] distance calculator with the default options.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SIFT4;

impl SIFT4 {
    pub fn new() -> Self {
        Self
    }

    pub fn distance(&self, s1: &str, s2: &str, max_offset: i64) -> f64 {
        distance(s1, s2, max_offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_examples() {
        assert_eq!(
            distance("This is the first string", "And this is another string", 5),
            11.0
        );
        assert_eq!(
            distance(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                "Amet Lorm ispum dolor sit amet, consetetur adixxxpiscing elit.",
                10
            ),
            12.0
        );
    }

    #[test]
    fn empty_strings() {
        assert_eq!(distance("", "", 5), 0.0);
        assert_eq!(distance("", "abc", 5), 3.0);
        assert_eq!(distance("abc", "", 5), 3.0);
    }

    #[test]
    fn identical_strings() {
        assert_eq!(distance("hello", "hello", 5), 0.0);
    }

    #[test]
    fn max_distance_cutoff_is_respected() {
        let unrestricted = distance("abcdef", "uvwxyz", 5);
        let restricted = distance_with_options("abcdef", "uvwxyz", 2, &CutoffOptions).unwrap();
        assert!(restricted <= unrestricted);
    }

    #[derive(Debug)]
    struct CutoffOptions;

    impl Sift4Options for CutoffOptions {
        type Token = char;
        type Error = Infallible;

        fn max_distance(&self) -> f64 {
            2.0
        }

        fn tokenize(&self, string: &str) -> Result<Vec<char>, Infallible> {
            Ok(string.chars().collect())
        }

        fn token_match(&self, t1: &char, t2: &char) -> Result<bool, Infallible> {
            Ok(t1 == t2)
        }

        fn matching_evaluator(&self, _t1: &char, _t2: &char) -> Result<f64, Infallible> {
            Ok(1.0)
        }

        fn local_length_evaluator(&self, length: f64) -> Result<f64, Infallible> {
            Ok(length)
        }

        fn transposition_cost_evaluator(&self, _c1: f64, _c2: f64) -> Result<f64, Infallible> {
            Ok(1.0)
        }

        fn transpositions_evaluator(
            &self,
            lcss: f64,
            transpositions: f64,
        ) -> Result<f64, Infallible> {
            Ok(lcss - transpositions)
        }
    }

    #[test]
    fn banker_rounding_matches_python() {
        assert_eq!(round_ties_even(0.5), 0.0);
        assert_eq!(round_ties_even(1.5), 2.0);
        assert_eq!(round_ties_even(2.5), 2.0);
        assert_eq!(round_ties_even(3.5), 4.0);
    }
}
