import unittest

from strsimpy.jaccard import Jaccard


class TestJaccard(unittest.TestCase):

    def test_jaccard(self):
        jaccard = Jaccard(1)
        s = ["", " ", "Shanghai", "ShangHai", "Shang Hai"]
        for i in range(len(s)):
            for j in range(i, len(s)):
                print(
                    "dis between '%s' and '%s': %.4f"
                    % (s[i], s[j], jaccard.distance(s[i], s[j]))
                )
                print(
                    "sim between '%s' and '%s': %.4f"
                    % (s[i], s[j], jaccard.similarity(s[i], s[j]))
                )

        self.assertEqual(1.0, jaccard.similarity("", ""))
        self.assertEqual(0.75, jaccard.similarity("Shanghai", "Shang Hai"))

    def test_known_values(self):
        self.assertAlmostEqual(1.0 / 3.0, Jaccard(2).similarity("car", "bar"))
        self.assertAlmostEqual(2.0 / 3.0, Jaccard(2).distance("car", "bar"))
        self.assertAlmostEqual(0.25, Jaccard(3).similarity("eat", "eating"))

    def test_short_strings_have_zero_similarity(self):
        jaccard = Jaccard(3)
        self.assertEqual(0.0, jaccard.similarity("a", "b"))
        self.assertEqual(1.0, jaccard.distance("a", "b"))

    def test_get_profile_and_k(self):
        jaccard = Jaccard(2)
        self.assertEqual(2, jaccard.get_k())
        self.assertEqual({"AB": 1, "BC": 1}, jaccard.get_profile("ABC"))


if __name__ == "__main__":
    unittest.main()
