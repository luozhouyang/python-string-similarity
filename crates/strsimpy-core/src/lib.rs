//! Pure Rust implementations of the string similarity and distance measures
//! exposed by the `strsimpy` Python package.
//!
//! All algorithms operate on Unicode scalar values (code points), matching the
//! semantics of Python's `str`.

pub mod cosine;
pub mod damerau;
pub mod jaccard;
pub mod jaro_winkler;
pub mod levenshtein;
pub mod longest_common_subsequence;
pub mod metric_lcs;
pub mod ngram;
pub mod normalized_levenshtein;
pub mod optimal_string_alignment;
pub mod overlap_coefficient;
pub mod qgram;
pub mod shingle;
pub mod sift4;
pub mod sorensen_dice;
pub mod weighted_levenshtein;
