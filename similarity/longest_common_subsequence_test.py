import unittest

from .longest_common_subsequence import LongestCommonSubsequence


class TestLongestCommonSubsequence(unittest.TestCase):

    def test_longest_common_subsequence(self):
        a = LongestCommonSubsequence()
        s0 = ""
        s1 = ""
        s2 = "上海"
        s3 = "上海市"
        distance_format = "distance: {:.4}\t between {} and {}"
        print(distance_format.format(str(a.distance(s0, s1)), s0, s1))
        print(distance_format.format(str(a.distance(s0, s2)), s0, s2))
        print(distance_format.format(str(a.distance(s0, s3)), s0, s3))
        print(distance_format.format(str(a.distance(s1, s2)), s1, s2))
        print(distance_format.format(str(a.distance(s1, s3)), s1, s3))
        print(distance_format.format(str(a.distance(s2, s3)), s2, s3))


if __name__ == "__main__":
    unittest.main()
