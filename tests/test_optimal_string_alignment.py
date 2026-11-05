import unittest

from strsimpy.optimal_string_alignment import OptimalStringAlignment


class TestOptimalStringAlignment(unittest.TestCase):

    def test_optimal_string_alignment(self):
        a = OptimalStringAlignment()
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

    def test_readme_example(self):
        self.assertEqual(3.0, OptimalStringAlignment().distance("CA", "ABC"))

    def test_empty_strings(self):
        a = OptimalStringAlignment()
        self.assertEqual(3.0, a.distance("", "abc"))
        self.assertEqual(3.0, a.distance("abc", ""))

    def test_adjacent_transposition(self):
        self.assertEqual(1.0, OptimalStringAlignment().distance("CA", "AC"))


if __name__ == "__main__":
    unittest.main()
