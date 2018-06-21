from .shingle_based import ShingleBased
from .string_distance import NormalizedStringDistance
from .string_similarity import NormalizedStringSimilarity


class SorensenDice(ShingleBased, NormalizedStringDistance, NormalizedStringSimilarity):

    def __init__(self, k=3):
        super().__init__(k)

    def distance(self, s0, s1):
        return 1.0 - self.similarity(s0, s1)

    def similarity(self, s0, s1):
        if s0 is None:
            raise TypeError("Argument s0 is NoneType.")
        if s1 is None:
            raise TypeError("Argument s1 is NoneType.")
        if s0 == s1:
            return 1.0
        union = set()
        profile0, profile1 = self.get_profile(s0), self.get_profile(s1)
        for k in profile0.keys():
            union.add(k)
        for k in profile1.keys():
            union.add(k)
        inter = 0
        for k in union:
            if k in profile0.keys() and k in profile1.keys():
                inter += 1
        return 2.0 * inter / (len(profile0) + len(profile1))
