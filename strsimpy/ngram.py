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

from .string_distance import NormalizedStringDistance


class NGram(NormalizedStringDistance):

    def __init__(self, n=2):
        self.n = n

    def distance(self, s0, s1):
        if s0 is None:
            raise TypeError("Argument s0 is NoneType.")
        if s1 is None:
            raise TypeError("Argument s1 is NoneType.")
        if s0 == s1:
            return 0.0

        special = '\n'
        sl = len(s0)
        tl = len(s1)

        if sl == 0 or tl == 0:
            return 1.0

        cost = 0
        if sl < self.n or tl < self.n:
            for i in range(min(sl, tl)):
                if s0[i] == s1[i]:
                    cost += 1
            return 1.0 * cost / max(sl, tl)

        sa = [''] * (sl + self.n - 1)

        for i in range(len(sa)):
            if i < self.n - 1:
                sa[i] = special
            else:
                sa[i] = s0[i - self.n + 1]

        p = [0.0] * (sl + 1)
        d = [0.0] * (sl + 1)
        t_j = [''] * self.n
        for i in range(sl + 1):
            p[i] = 1.0 * i

        for j in range(1, tl + 1):
            if j < self.n:
                for ti in range(self.n - j):
                    t_j[ti] = special
                for ti in range(self.n - j, self.n):
                    t_j[ti] = s1[ti - (self.n - j)]
            else:
                t_j = s1[j - self.n:j]

            d[0] = 1.0 * j
            for i in range(sl + 1):
                cost = 0
                tn = self.n
                for ni in range(self.n):
                    if sa[i - 1 + ni] != t_j[ni]:
                        cost += 1
                    elif sa[i - 1 + ni] == special:
                        tn -= 1
                ec = cost / tn
                d[i] = min(d[i - 1] + 1, p[i] + 1, p[i - 1] + ec)
            p, d = d, p

        return p[sl] / max(tl, sl)
