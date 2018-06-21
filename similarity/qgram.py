from .shingle_based import ShingleBased
from .string_distance import StringDistance


class QGram(ShingleBased, StringDistance):

    def __init__(self, k=3):
        super().__init__(k)

    def distance(self, s0, s1):
        if s0 is None:
            raise TypeError("Argument s0 is NoneType.")
        if s1 is None:
            raise TypeError("Argument s1 is NoneType.")
        if s0 == s1:
            return 0.0

        profile0 = self.get_profile(s0)
        profile1 = self.get_profile(s1)
        return self.distance_profile(profile0, profile1)

    @staticmethod
    def distance_profile(profile0, profile1):
        union = set()
        for k in profile0.keys():
            union.add(k)
        for k in profile1.keys():
            union.add(k)
        agg = 0
        for k in union:
            v0, v1 = 0, 0
            if profile0.get(k) is not None:
                v0 = int(profile0.get(k))
            if profile1.get(k) is not None:
                v1 = int(profile1.get(k))
            agg += abs(v0 - v1)
        return agg
