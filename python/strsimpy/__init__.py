"""Rust-backed string similarity and distance measures.

The public API is unchanged: every class can be imported from the package
root, and the historical per-module imports (for example
``from strsimpy.levenshtein import Levenshtein``) continue to work.
"""

from importlib.metadata import PackageNotFoundError, version as _package_version

from ._base import (
    MetricStringDistance,
    NormalizedStringDistance,
    NormalizedStringSimilarity,
    ShingleBased,
    StringDistance,
    StringSimilarity,
)
from ._strsimpy import (
    SIFT4,
    SIFT4Options,
    Cosine,
    Damerau,
    Jaccard,
    JaroWinkler,
    Levenshtein,
    LongestCommonSubsequence,
    MetricLCS,
    NGram,
    NormalizedLevenshtein,
    OptimalStringAlignment,
    OverlapCoefficient,
    QGram,
    SorensenDice,
    WeightedLevenshtein,
    shingle_profile,
)

try:
    __version__ = _package_version("strsimpy")
except PackageNotFoundError:  # pragma: no cover - only when not installed
    __version__ = "0.4.0"

__all__ = [
    "SIFT4",
    "Cosine",
    "Damerau",
    "Jaccard",
    "JaroWinkler",
    "Levenshtein",
    "LongestCommonSubsequence",
    "MetricLCS",
    "MetricStringDistance",
    "NGram",
    "NormalizedLevenshtein",
    "NormalizedStringDistance",
    "NormalizedStringSimilarity",
    "OptimalStringAlignment",
    "OverlapCoefficient",
    "QGram",
    "SIFT4Options",
    "ShingleBased",
    "SorensenDice",
    "StringDistance",
    "StringSimilarity",
    "WeightedLevenshtein",
    "shingle_profile",
]
