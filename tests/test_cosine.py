import unittest
from collections import UserDict
from types import MappingProxyType

from strsimpy.cosine import Cosine


class TestCosine(unittest.TestCase):

    def test_whitespace_normalization_can_remove_all_shingles(self):
        cos = Cosine(4)
        for short in ("    ", "0  s", "\t\n  ", "a  b"):
            for left, right in ((short, "foo bar"), ("foo bar", short)):
                with self.subTest(left=left, right=right):
                    self.assertEqual(cos.similarity(left, right), 0.0)
                    self.assertEqual(cos.distance(left, right), 1.0)

    def test_empty_profiles(self):
        cos = Cosine(4)
        for left, right in (
            ({}, {"test": 1}),
            ({"test": 1}, {}),
            ({}, {}),
            ({"test": 0}, {"test": 1}),
            ({"test": 1}, {"test": 0}),
        ):
            with self.subTest(left=left, right=right):
                self.assertEqual(cos.similarity_profiles(left, right), 0.0)

    def test_nonempty_profiles_and_equal_strings(self):
        cos = Cosine(4)
        self.assertAlmostEqual(cos.similarity_profiles({"test": 2}, {"test": 3}), 1.0)
        self.assertEqual(cos.similarity_profiles({"abcd": 1}, {"efgh": 1}), 0.0)
        self.assertEqual(cos.similarity("    ", "    "), 1.0)
        self.assertEqual(cos.similarity("", ""), 1.0)
        self.assertEqual(cos.similarity("a", "b"), 0.0)

    def test_cosine(self):
        cos = Cosine(1)
        s = ["", " ", "Shanghai", "ShangHai", "Shang Hai"]
        for i in range(len(s)):
            for j in range(i, len(s)):
                print(
                    "dis between '%s' and '%s': %.4f"
                    % (s[i], s[j], cos.distance(s[i], s[j]))
                )
                print(
                    "sim between '%s' and '%s': %.4f"
                    % (s[i], s[j], cos.similarity(s[i], s[j]))
                )

        self.assertEqual(1.0, cos.similarity("Shanghai", "Shanghai"))
        self.assertEqual(0.0, cos.distance("Shanghai", "Shanghai"))

    def test_direct_and_profile_similarity_agree(self):
        cos = Cosine(2)
        s0 = "My first string"
        s1 = "My other string..."
        self.assertAlmostEqual(
            cos.similarity(s0, s1),
            cos.similarity_profiles(cos.get_profile(s0), cos.get_profile(s1)),
        )

    def test_get_k(self):
        self.assertEqual(2, Cosine(2).get_k())

    def test_similarity_profiles_accept_mapping_wrappers(self):
        cos = Cosine(2)
        left = MappingProxyType({"ab": 2, "bc": 1})
        right = UserDict({"ab": 1, "cd": 1})
        self.assertAlmostEqual(0.6324555320336759, cos.similarity_profiles(left, right))
        self.assertAlmostEqual(
            cos.similarity_profiles({"ab": 2, "bc": 1}, {"ab": 1, "cd": 1}),
            cos.similarity_profiles(left, right),
        )
        self.assertAlmostEqual(
            cos.similarity_profiles(
                MappingProxyType(cos.get_profile("My first string")),
                MappingProxyType(cos.get_profile("My other string...")),
            ),
            cos.similarity("My first string", "My other string..."),
        )


if __name__ == "__main__":
    unittest.main()
