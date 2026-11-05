import unittest

from strsimpy.normalized_levenshtein import NormalizedLevenshtein


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

        self.assertEqual(0.0, a.distance(s0, s1))
        self.assertEqual(1.0, a.distance(s0, s2))
        self.assertEqual(1.0, a.distance(s0, s3))
        self.assertAlmostEqual(1.0 / 3.0, a.distance(s2, s3))
        self.assertEqual(1.0, a.similarity(s0, s1))
        self.assertEqual(0.0, a.similarity(s0, s2))
        self.assertAlmostEqual(2.0 / 3.0, a.similarity(s2, s3))

    def test_similarity_complements_distance(self):
        a = NormalizedLevenshtein()
        for s0, s1 in (("", ""), ("", "abc"), ("abc", "abd"), ("上海", "上海市")):
            self.assertAlmostEqual(1.0, a.similarity(s0, s1) + a.distance(s0, s1))


if __name__ == "__main__":
    unittest.main()
