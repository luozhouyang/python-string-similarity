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


if __name__ == "__main__":
    unittest.main()
