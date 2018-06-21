import numpy as np

from .string_distance import StringDistance


class OptimalStringAlignment(StringDistance):

    def distance(self, s0, s1):
        if s0 is None:
            raise TypeError("Argument s0 is NoneType.")
        if s1 is None:
            raise TypeError("Argument s1 is NoneType.")
        if s0 == s1:
            return 0.0

        n, m = len(s0), len(s1)
        if n == 0:
            return 1.0 * n
        if m == 0:
            return 1.0 * m

        d = np.zeros((n + 2, m + 2))
        for i in range(n + 1):
            d[i][0] = i
        for j in range(m + 1):
            d[0][j] = j

        for i in range(1, n + 1):
            for j in range(1, m + 1):
                cost = 1
                if s0[i - 1] == s1[j - 1]:
                    cost = 0
                d[i][j] = min(d[i - 1][j - 1] + cost, d[i][j - 1] + 1, d[i - 1][j] + 1)

                if i > 1 and j > 1 and s0[i - 1] == s1[j - 2] and s0[i - 2] == s1[j - 1]:
                    d[i][j] = min(d[i][j], d[i - 2][j - 2] + cost)

        return d[n][m]
