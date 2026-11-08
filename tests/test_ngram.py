import unittest

from strsimpy.ngram import NGram


class TestNGram(unittest.TestCase):

    def test_ngram(self):
        a = NGram(2)
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

    def test_known_values(self):
        self.assertAlmostEqual(0.5833333333333334, NGram(2).distance("ABCD", "ABTUIO"))
        self.assertAlmostEqual(
            0.4583333333333333, NGram(2).distance("ABABABAB", "ABCABCABCABC")
        )
        self.assertAlmostEqual(0.5, NGram(3).distance("ab", "ac"))
        self.assertAlmostEqual(1.0 / 6.0, NGram(2).distance("ShangHai", "Shang Hai"))

    def test_upstream_ordering(self):
        ngram = NGram(2)
        self.assertLess(
            ngram.distance("ABABABAB", "ABCABCABCABC"),
            ngram.distance("ABABABAB", "POIULKJH"),
        )

    def test_newlines_are_input_characters(self):
        self.assertEqual(0.5, NGram(1).distance("\na", "\nb"))
        self.assertEqual(0.25, NGram(2).distance("\na", "\nb"))

    def test_newline_relabeling_preserves_distance(self):
        pairs = [
            ("\na", "\nb"),
            ("a\nb", "a\nc"),
            ("a\n", "b\n"),
            ("\n\n", "\na"),
            ("a\n\nb", "\na\nb"),
            ("\na\0", "\nb\0"),
            ("海\na", "海\nb"),
            ("", "\n"),
        ]
        for n in range(1, 5):
            ngram = NGram(n)
            for a, b in pairs:
                with self.subTest(n=n, a=a, b=b):
                    # Relabeling an input character cannot change edit costs.
                    expected = ngram.distance(
                        a.replace("\n", "x"), b.replace("\n", "x")
                    )
                    actual = ngram.distance(a, b)
                    self.assertAlmostEqual(expected, actual)
                    self.assertGreaterEqual(actual, 0.0)
                    self.assertLessEqual(actual, 1.0)

    def test_invalid_n(self):
        with self.assertRaises(ValueError):
            NGram(0)


if __name__ == "__main__":
    unittest.main()
