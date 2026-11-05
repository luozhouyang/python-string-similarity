"""Abstract base classes kept for API compatibility.

The algorithms themselves live in the Rust extension module
``strsimpy._strsimpy``.  The base classes are implemented in Python and the
native classes are registered as virtual subclasses, so ``isinstance`` checks
written against the original pure-Python API keep working.
"""

from abc import ABCMeta

from . import _strsimpy
from ._strsimpy import (
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
    SIFT4Options,
    SorensenDice,
    WeightedLevenshtein,
)


class StringDistance(metaclass=ABCMeta):
    def distance(self, s0, s1):
        raise NotImplementedError()


class NormalizedStringDistance(StringDistance):
    def distance(self, s0, s1):
        raise NotImplementedError()


class MetricStringDistance(StringDistance):
    def distance(self, s0, s1):
        raise NotImplementedError()


class StringSimilarity(metaclass=ABCMeta):
    def similarity(self, s0, s1):
        raise NotImplementedError()


class NormalizedStringSimilarity(StringSimilarity):
    def similarity(self, s0, s1):
        raise NotImplementedError()


class ShingleBased(metaclass=ABCMeta):
    def __init__(self, k=3):
        self.k = k

    def get_k(self):
        return self.k

    def get_profile(self, string):
        return _strsimpy.shingle_profile(string, self.k)


# Present the same module paths as the original pure-Python classes.
StringDistance.__module__ = "strsimpy.string_distance"
NormalizedStringDistance.__module__ = "strsimpy.string_distance"
MetricStringDistance.__module__ = "strsimpy.string_distance"
StringSimilarity.__module__ = "strsimpy.string_similarity"
NormalizedStringSimilarity.__module__ = "strsimpy.string_similarity"
ShingleBased.__module__ = "strsimpy.shingle_based"


# Keep isinstance() checks working exactly as they did with the class hierarchy
# of the pure-Python implementation.
StringDistance.register(OptimalStringAlignment)
StringDistance.register(WeightedLevenshtein)
StringDistance.register(QGram)
StringDistance.register(LongestCommonSubsequence)
StringDistance.register(SIFT4Options)

MetricStringDistance.register(Levenshtein)
MetricStringDistance.register(Damerau)
MetricStringDistance.register(Jaccard)
MetricStringDistance.register(MetricLCS)
MetricStringDistance.register(SIFT4Options)

NormalizedStringDistance.register(NormalizedLevenshtein)
NormalizedStringDistance.register(JaroWinkler)
NormalizedStringDistance.register(MetricLCS)
NormalizedStringDistance.register(NGram)
NormalizedStringDistance.register(Jaccard)
NormalizedStringDistance.register(Cosine)
NormalizedStringDistance.register(SorensenDice)
NormalizedStringDistance.register(OverlapCoefficient)

StringSimilarity.register(NormalizedLevenshtein)
StringSimilarity.register(JaroWinkler)
StringSimilarity.register(Jaccard)
StringSimilarity.register(Cosine)
StringSimilarity.register(SorensenDice)
StringSimilarity.register(OverlapCoefficient)

NormalizedStringSimilarity.register(NormalizedLevenshtein)
NormalizedStringSimilarity.register(JaroWinkler)
NormalizedStringSimilarity.register(Jaccard)
NormalizedStringSimilarity.register(Cosine)
NormalizedStringSimilarity.register(SorensenDice)
NormalizedStringSimilarity.register(OverlapCoefficient)

ShingleBased.register(QGram)
ShingleBased.register(Cosine)
ShingleBased.register(Jaccard)
ShingleBased.register(SorensenDice)
ShingleBased.register(OverlapCoefficient)

__all__ = [
    "MetricStringDistance",
    "NormalizedStringDistance",
    "NormalizedStringSimilarity",
    "ShingleBased",
    "StringDistance",
    "StringSimilarity",
]
