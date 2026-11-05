import unittest

from strsimpy.levenshtein import Levenshtein


class TestLevenshtein(unittest.TestCase):

    def test_levenshtein(self):
        a = Levenshtein()
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

    def test_known_values(self):
        a = Levenshtein()
        self.assertEqual(3, a.distance("kitten", "sitting"))
        self.assertEqual(2, a.distance("flaw", "lawn"))
        self.assertEqual(3, a.distance("Saturday", "Sunday"))
        self.assertEqual(1, a.distance("😀", "😁"))

    def test_none_arguments_raise_type_error(self):
        a = Levenshtein()
        with self.assertRaises(TypeError):
            a.distance(None, "abc")
        with self.assertRaises(TypeError):
            a.distance("abc", None)


if __name__ == "__main__":
    unittest.main()
