# Copyright (c) 2018 luozhouyang
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

from enum import IntEnum
from .cosine import Cosine
from .damerau import Damerau
from .jaccard import Jaccard
from .jarowinkler import JaroWinkler
from .levenshtein import Levenshtein
from .longest_common_subsequence import LongestCommonSubsequence
from .metric_lcs import MetricLCS
from .ngram import NGram
from .normalized_levenshtein import NormalizedLevenshtein
from .optimal_string_alignment import OptimalStringAlignment
from .qgram import QGram
from .sorensen_dice import SorensenDice
from .weighted_levenshtein import WeightedLevenshtein


class Algorithm(IntEnum):
    COSINE = 1
    DAMERAU = 2
    JACCARD = 3
    JARO_WINKLE = 4
    LEVENSHTEIN = 5
    LCS = 6
    METRIC_LCS = 7
    N_GRAM = 8
    NORMALIZED_LEVENSHTEIN = 9
    OPTIMAL_STRING_ALIGNMENT = 10
    Q_GRAM = 11
    SORENSEN_DICE = 12
    WEIGHTED_LEVENSHTEIN = 13


class Factory:
    @staticmethod
    def get_algorithm(algorithm: Algorithm, k=3):
        if algorithm == Algorithm.COSINE:
            return Cosine(k)
        elif algorithm == Algorithm.DAMERAU:
            return Damerau()
        elif algorithm == Algorithm.JACCARD:
            return Jaccard(k)
        elif algorithm == Algorithm.JARO_WINKLE:
            return JaroWinkler()
        elif algorithm == Algorithm.LEVENSHTEIN:
            return Levenshtein()
        elif algorithm == Algorithm.LCS:
            return LongestCommonSubsequence()
        elif algorithm == Algorithm.METRIC_LCS:
            return MetricLCS()
        elif algorithm == Algorithm.N_GRAM:
            return NGram()
        elif algorithm == Algorithm.NORMALIZED_LEVENSHTEIN:
            return NormalizedLevenshtein()
        elif algorithm == Algorithm.OPTIMAL_STRING_ALIGNMENT:
            return OptimalStringAlignment()
        elif algorithm == Algorithm.Q_GRAM:
            return QGram()
        elif algorithm == Algorithm.SORENSEN_DICE:
            return SorensenDice(k)
        elif algorithm == Algorithm.WEIGHTED_LEVENSHTEIN:
            raise TypeError("This method does not support create weighted_levenshtein algorithm.")
        else:
            return Cosine(k)

    @staticmethod
    def get_weighted_levenshtein(char_sub, char_change):
        return WeightedLevenshtein(char_sub, char_change)


if __name__ == "__main__":
    a = Factory().get_algorithm(Algorithm.LEVENSHTEIN)
    distance_format = "distance: {:.4} between {} and {}"
    s0 = "你好"
    s1 = "你好啊"
    print(distance_format.format(str(a.distance(s0, s1)), s0, s1))
