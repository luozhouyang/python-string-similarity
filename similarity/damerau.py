from .string_distance import MetricStringDistance
import numpy as np


class Damerau(MetricStringDistance):

    def distance(self, s0, s1):
        if s0 is None:
            raise TypeError("Argument s0 is NoneType.")
        if s1 is None:
            raise TypeError("Argument s1 is NoneType.")
        if s0 == s1:
            return 0.0
        inf = int(len(s0) + len(s1))
        da = dict()
        for i in range(len(s0)):
            da[s0[i]] = str(0)
        for i in range(len(s1)):
            da[s1[i]] = str(0)
        h = np.zeros((len(s0) + 2, len(s1) + 2))
        for i in range(len(s0) + 1):
            h[i + 1][0] = inf
            h[i + 1][1] = i
        for j in range(len(s1) + 1):
            h[0][j + 1] = inf
            h[1][j + 1] = j
        for i in range(1, len(s0) + 1):
            db = 0
            for j in range(1, len(s1) + 1):
                i1 = int(da[s1[j - 1]])
                j1 = db

                cost = 1
                if s0[i - 1] == s1[j - 1]:
                    cost = 0
                    db = j
                h[i + 1][j + 1] = min(h[i][j] + cost,
                                      h[i + 1][j] + 1,
                                      h[i][j + 1] + 1,
                                      h[i1][j1] + (i - i1 - 1) + 1 + (j - j1 - 1))
            da[s0[i - 1]] = str(i)

        return h[len(s0) + 1][len(s1) + 1]
