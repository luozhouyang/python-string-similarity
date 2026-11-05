import unittest

from strsimpy.longest_common_subsequence import LongestCommonSubsequence


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
        self.assertEqual(4, a.distance("AGCAT", "GAC"))
        self.assertEqual(2, a.length("AGCAT", "GAC"))

        # length() is a static method in the original API.
        self.assertEqual(2, LongestCommonSubsequence.length("AGCAT", "GAC"))


if __name__ == "__main__":
    unittest.main()
