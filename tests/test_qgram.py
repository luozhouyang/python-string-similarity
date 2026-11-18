import unittest
from collections import Counter, UserDict
from types import MappingProxyType

from strsimpy.qgram import QGram


class TestQGram(unittest.TestCase):

    def test_qgram(self):
        a = QGram(1)
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

        self.assertEqual(0, a.distance(s0, s1))
        self.assertEqual(2, a.distance(s0, s2))
        self.assertEqual(3, a.distance(s0, s3))
        self.assertEqual(1, a.distance(s2, s3))
        self.assertEqual(1, a.distance("ShangHai", "Shang Hai"))

    def test_profiles(self):
        qgram = QGram(2)
        self.assertEqual({"AB": 1, "BC": 1, "CD": 1}, qgram.get_profile("ABCD"))
        self.assertEqual(2, qgram.distance("ABCD", "ABCE"))
        self.assertEqual(
            2,
            QGram.distance_profile(
                qgram.get_profile("ABCD"), qgram.get_profile("ABCE")
            ),
        )

    def test_whitespace_normalization(self):
        self.assertEqual({"a ": 1, " b": 1}, QGram(2).get_profile("a  b"))

    def test_distance_profile_accepts_mapping_wrappers(self):
        qgram = QGram(2)
        left = MappingProxyType(qgram.get_profile("ABCD"))
        right = UserDict(qgram.get_profile("ABCE"))
        self.assertEqual(2, QGram.distance_profile(left, right))
        self.assertEqual(2, qgram.distance_profile(left, right))
        self.assertEqual(
            2,
            QGram.distance_profile(
                MappingProxyType({"AB": 1, "BC": 1, "CD": 1}),
                Counter({"AB": 1, "BC": 1, "CE": 1}),
            ),
        )


if __name__ == "__main__":
    unittest.main()
