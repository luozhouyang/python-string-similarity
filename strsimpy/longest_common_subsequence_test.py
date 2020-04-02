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

import unittest

from .longest_common_subsequence import LongestCommonSubsequence


class LongestCommonSubsequenceTest(unittest.TestCase):

    def test_longest_common_subsequence(self):
        a = LongestCommonSubsequence()
        s0 = ""
        s1 = ""
        s2 = "上海"
        s3 = "上海市"

        self.assertEqual(0, a.distance(s0, s1))
        self.assertEqual(2, a.distance(s0, s2))
        self.assertEqual(3, a.distance(s0, s3))
        self.assertEqual(1, a.distance(s2, s3))
        self.assertEqual(2, a.length(s2, s3))
        self.assertEqual(4, a.distance('AGCAT', 'GAC'))
        self.assertEqual(2, a.length('AGCAT', 'GAC'))


if __name__ == "__main__":
    unittest.main()
