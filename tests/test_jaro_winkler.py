import unittest

from strsimpy.jaro_winkler import JaroWinkler


class TestJaroWinkler(unittest.TestCase):

    def test_jarowinkler(self):
        a = JaroWinkler()
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

        self.assertEqual(1.0, a.similarity(s0, s1))
        self.assertEqual(0.0, a.similarity(s0, s2))
        self.assertAlmostEqual(0.9111111111111111, a.similarity(s2, s3))

    def test_readme_values(self):
        a = JaroWinkler()
        self.assertAlmostEqual(
            0.9740740740740741, a.similarity("My string", "My tsring")
        )
        self.assertAlmostEqual(
            0.8962962962962963, a.similarity("My string", "My ntrisg")
        )

    def test_distance_is_one_minus_similarity(self):
        a = JaroWinkler()
        for s0, s1 in (("", ""), ("", "a"), ("martha", "marhta"), ("上海", "上海市")):
            self.assertAlmostEqual(1.0 - a.similarity(s0, s1), a.distance(s0, s1))

    def test_threshold(self):
        self.assertEqual(0.7, JaroWinkler().get_threshold())
        self.assertEqual(0.5, JaroWinkler(0.5).get_threshold())


if __name__ == "__main__":
    unittest.main()
