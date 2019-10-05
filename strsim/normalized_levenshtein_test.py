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

from .normalized_levenshtein import NormalizedLevenshtein


class TestNormalizedLevenshtein(unittest.TestCase):

    def test_normalized_levenshtein(self):
        a = NormalizedLevenshtein()
        s0 = ""
        s1 = ""
        s2 = "上海"
        s3 = "上海市"
        distance_format = "distance: {:.4}\t between {} and {}"
        similarity_format = "strsim: {:.4}\t between {} and {}"
        print(distance_format.format(str(a.distance(s0, s1)), s0, s1))
        print(distance_format.format(str(a.distance(s0, s2)), s0, s2))
        print(distance_format.format(str(a.distance(s0, s3)), s0, s3))
        print(distance_format.format(str(a.distance(s1, s2)), s1, s2))
        print(distance_format.format(str(a.distance(s1, s3)), s1, s3))
        print(distance_format.format(str(a.distance(s2, s3)), s2, s3))

        print(similarity_format.format(str(a.similarity(s0, s1)), s0, s1))
        print(similarity_format.format(str(a.similarity(s0, s2)), s0, s2))
        print(similarity_format.format(str(a.similarity(s0, s3)), s0, s3))
        print(similarity_format.format(str(a.similarity(s1, s2)), s1, s2))
        print(similarity_format.format(str(a.similarity(s1, s3)), s1, s3))
        print(similarity_format.format(str(a.similarity(s2, s3)), s2, s3))


if __name__ == "__main__":
    unittest.main()
