import unittest

from strsimpy.metric_lcs import MetricLCS


class TestMetricLCS(unittest.TestCase):

    def test_metric_lcs(self):
        a = MetricLCS()
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
        self.assertEqual(1.0, a.distance(s0, s2))
        self.assertEqual(1.0, a.distance(s0, s3))
        self.assertAlmostEqual(1.0 / 3.0, a.distance(s2, s3))

    def test_readme_values(self):
        a = MetricLCS()
        self.assertAlmostEqual(0.4, a.distance("ABCDEFG", "ABCDEFHJKL"))
        self.assertAlmostEqual(0.2, a.distance("ABDEF", "ABDIF"))


if __name__ == "__main__":
    unittest.main()
