import unittest

from strsimpy.sorensen_dice import SorensenDice


class TestSorensenDice(unittest.TestCase):

    def test_sorensen_dice(self):
        a = SorensenDice(2)
        s2 = "上海"
        s3 = "上海市"
        distance_format = "distance: {:.4}\t between {} and {}"
        similarity_format = "strsim: {:.4}\t between {} and {}"
        print(distance_format.format(str(a.distance(s2, s3)), s2, s3))
        print(similarity_format.format(str(a.similarity(s2, s3)), s2, s3))

        self.assertAlmostEqual(2.0 / 3.0, a.similarity(s2, s3))
        self.assertAlmostEqual(1.0 / 3.0, a.distance(s2, s3))

    def test_empty_profiles(self):
        a = SorensenDice(3)
        for left, right in (("", "a"), ("a", ""), ("a", "b")):
            with self.subTest(left=left, right=right):
                self.assertEqual(0.0, a.similarity(left, right))
                self.assertEqual(1.0, a.distance(left, right))

    def test_equal_strings(self):
        a = SorensenDice(3)
        self.assertEqual(1.0, a.similarity("", ""))
        self.assertEqual(1.0, a.similarity("Shanghai", "Shanghai"))

    def test_get_profile_and_k(self):
        a = SorensenDice(2)
        self.assertEqual(2, a.get_k())
        self.assertEqual({"AB": 1, "BC": 1}, a.get_profile("ABC"))


if __name__ == "__main__":
    unittest.main()
