import unittest

from strsimpy.damerau import Damerau


class TestDamerau(unittest.TestCase):

    def test_damerau(self):
        a = Damerau()
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

        self.assertEqual(0.0, a.distance(s0, s1))
        self.assertEqual(2.0, a.distance(s0, s2))
        self.assertEqual(3.0, a.distance(s0, s3))
        self.assertEqual(1.0, a.distance(s2, s3))

    def test_readme_values(self):
        damerau = Damerau()
        self.assertEqual(1.0, damerau.distance("ABCDEF", "ABDCEF"))
        self.assertEqual(2.0, damerau.distance("ABCDEF", "BACDFE"))
        self.assertEqual(1.0, damerau.distance("ABCDEF", "ABCDE"))
        self.assertEqual(1.0, damerau.distance("ABCDEF", "BCDEF"))
        self.assertEqual(1.0, damerau.distance("ABCDEF", "ABCGDEF"))
        self.assertEqual(6.0, damerau.distance("ABCDEF", "POIU"))

    def test_adjacent_transposition(self):
        self.assertEqual(1.0, Damerau().distance("CA", "AC"))


if __name__ == "__main__":
    unittest.main()
