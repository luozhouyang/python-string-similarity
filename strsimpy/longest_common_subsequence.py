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

from .string_distance import StringDistance


class LongestCommonSubsequence(StringDistance):
    def distance(self, s0, s1):
        if s0 is None:
            raise TypeError("Argument s0 is NoneType.")
        if s1 is None:
            raise TypeError("Argument s1 is NoneType.")
        if s0 == s1:
            return 0.0
        return len(s0) + len(s1) - 2 * self.length(s0, s1)

    @staticmethod
    def length(s0, s1):
        if s0 is None:
            raise TypeError("Argument s0 is NoneType.")
        if s1 is None:
            raise TypeError("Argument s1 is NoneType.")
        s0_len, s1_len = len(s0), len(s1)
        # only the previous row is ever read, so keep two rolling rows
        # instead of the full matrix
        prev = [0] * (s1_len + 1)
        curr = [0] * (s1_len + 1)
        for i in range(1, s0_len + 1):
            ch0 = s0[i - 1]
            for j in range(1, s1_len + 1):
                if ch0 == s1[j - 1]:
                    curr[j] = prev[j - 1] + 1
                else:
                    up = prev[j]
                    left = curr[j - 1]
                    curr[j] = up if up >= left else left
            prev, curr = curr, prev
        return prev[s1_len]
