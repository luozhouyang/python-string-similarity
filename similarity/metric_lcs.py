from .string_distance import MetricStringDistance, NormalizedStringDistance
from .longest_common_subsequence import LongestCommonSubsequence


class MetricLCS(MetricStringDistance, NormalizedStringDistance):

    def __init__(self):
        self.lcs = LongestCommonSubsequence()

    def distance(self, s0, s1):
        if s0 is None:
            raise TypeError("Argument s0 is NoneType.")
        if s1 is None:
            raise TypeError("Argument s1 is NoneType.")
        if s0 == s1:
            return 0.0
        max_len = int(max(len(s0), len(s1)))
        if max_len == 0:
            return 0.0
        return 1.0 - (1.0 * self.lcs.length(s0, s1)) / max_len
