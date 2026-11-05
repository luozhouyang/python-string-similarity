import unittest

from strsimpy.levenshtein import Levenshtein
from strsimpy.weighted_levenshtein import (
    WeightedLevenshtein,
    default_deletion_cost,
    default_insertion_cost,
    default_substitution_cost,
)


def insertion_cost(char):
    return 1.0


def deletion_cost(char):
    return 1.0


def substitution_cost(char_a, char_b):
    if char_a == "t" and char_b == "r":
        return 0.5
    return 1.0


class TestWeightedLevenshtein(unittest.TestCase):

    def test_weighted_levenshtein(self):
        a = WeightedLevenshtein()
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

    def test_defaults_match_plain_levenshtein(self):
        weighted = WeightedLevenshtein()
        plain = Levenshtein()
        for s0, s1 in (
            ("", ""),
            ("", "abc"),
            ("kitten", "sitting"),
            ("上海", "上海市"),
        ):
            self.assertEqual(plain.distance(s0, s1), weighted.distance(s0, s1))

    def test_custom_callables(self):
        weighted = WeightedLevenshtein(
            substitution_cost_fn=substitution_cost,
            insertion_cost_fn=insertion_cost,
            deletion_cost_fn=deletion_cost,
        )
        self.assertEqual(1.0, weighted.distance("String1", "String2"))
        self.assertEqual(0.5, weighted.distance("t", "r"))

    def test_custom_insertion_and_deletion_costs(self):
        weighted = WeightedLevenshtein(
            insertion_cost_fn=lambda char: 2.0,
            deletion_cost_fn=lambda char: 3.0,
        )
        self.assertEqual(4.0, weighted.distance("", "ab"))
        self.assertEqual(6.0, weighted.distance("ab", ""))

    def test_cost_function_attributes(self):
        weighted = WeightedLevenshtein()
        self.assertEqual(1.0, weighted.insertion_cost_fn("a"))
        self.assertEqual(1.0, weighted.deletion_cost_fn("a"))
        self.assertEqual(1.0, weighted.substitution_cost_fn("a", "b"))
        weighted.insertion_cost_fn = default_insertion_cost
        self.assertEqual(1.0, weighted.insertion_cost_fn("a"))
        self.assertEqual(1.0, default_deletion_cost("a"))
        self.assertEqual(1.0, default_substitution_cost("a", "b"))

    def test_none_arguments_raise_type_error(self):
        weighted = WeightedLevenshtein()
        with self.assertRaises(TypeError):
            weighted.distance(None, "abc")

    def test_cost_function_errors_propagate(self):
        def boom(char):
            raise RuntimeError("boom")

        weighted = WeightedLevenshtein(insertion_cost_fn=boom)
        with self.assertRaisesRegex(RuntimeError, "boom"):
            weighted.distance("", "a")


if __name__ == "__main__":
    unittest.main()
