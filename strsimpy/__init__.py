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


from .cosine import Cosine
from .damerau import Damerau
from .jaccard import Jaccard
from .levenshtein import Levenshtein
from .longest_common_subsequence import LongestCommonSubsequence
from .metric_lcs import MetricLCS
from .ngram import NGram
from .normalized_levenshtein import NormalizedLevenshtein
from .optimal_string_alignment import OptimalStringAlignment
from .qgram import QGram
from .shingle_based import ShingleBased
from .sorensen_dice import SorensenDice
from .string_distance import StringDistance
from .string_similarity import StringSimilarity
from .weighted_levenshtein import WeightedLevenshtein

__name__ = 'strsimpy'
__version__ = '0.1.6'
