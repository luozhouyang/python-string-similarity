import unittest

from strsimpy.overlap_coefficient import OverlapCoefficient


class TestOverlapCoefficient(unittest.TestCase):

    def test_overlap_coefficient_onestringissubsetofother_return0(self):
        sim = OverlapCoefficient(3)
        s1, s2 = "eat", "eating"
        actual = sim.distance(s1, s2)
        print("distance: {:.4}\t between '{}' and '{}'".format(str(actual), s1, s2))
        self.assertEqual(0, actual)

    def test_overlap_coefficient_onestringissubset_return1(self):
        sim = OverlapCoefficient(3)
        s1, s2 = "eat", "eating"
        actual = sim.similarity(s1, s2)
        print("strsim: {:.4}\t between '{}' and '{}'".format(str(actual), s1, s2))
        self.assertEqual(1, actual)

    def test_overlap_coefficient_onestringissubsetofother_return1(self):
        sim = OverlapCoefficient(3)
        s1, s2 = "eat", "eating"
        actual = sim.similarity(s1, s2)
        print("strsim: {:.4}\t between '{}' and '{}'".format(str(actual), s1, s2))
        self.assertEqual(1, actual)

    def test_overlap_coefficient_halfsimilar_return1(self):
        sim = OverlapCoefficient(2)
        s1, s2 = "car", "bar"
        self.assertEqual(1 / 2, sim.similarity(s1, s2))
        self.assertEqual(1 / 2, sim.distance(s1, s2))

    def test_empty_profile_after_whitespace_normalization(self):
        sim = OverlapCoefficient(3)
        for short in ("   ", "\t\n "):
            for left, right in ((short, "eating"), ("eating", short)):
                with self.subTest(left=left, right=right):
                    self.assertEqual(0.0, sim.similarity(left, right))
                    self.assertEqual(1.0, sim.distance(left, right))
        self.assertEqual(1.0, sim.similarity("   ", "   "))


if __name__ == "__main__":
    unittest.main()
