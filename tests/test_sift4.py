import unittest

from strsimpy.sift4 import SIFT4, SIFT4Options


class SIFT4Test(unittest.TestCase):

    def test_named_token_matcher(self):
        strings = ("hello world", "hallo world")
        named = {"tokenizer": "wordsplit", "tokenmatcher": "sift4tokenmatcher"}
        callable_option = {
            "tokenizer": "wordsplit",
            "tokenmatcher": SIFT4Options.sift4tokenmatcher,
        }
        self.assertEqual(SIFT4().distance(*strings, options=callable_option), 0)
        self.assertEqual(SIFT4().distance(*strings, options=named), 0)
        self.assertEqual(SIFT4().distance("cat dog", "cat pig", options=named), 1)

    def test_unknown_named_token_matcher(self):
        with self.assertRaisesRegex(ValueError, "sift4tokenmatcher"):
            SIFT4Options({"tokenmatcher": "unknown"})

    def testSIFT4(self):
        s = SIFT4()

        results = [
            ("This is the first string", "And this is another string", 5, 11.0),
            (
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                "Amet Lorm ispum dolor sit amet, consetetur adixxxpiscing elit.",
                10,
                12.0,
            ),
        ]

        for a, b, offset, res in results:
            self.assertEqual(res, s.distance(a, b, maxoffset=offset))

    def test_empty_strings(self):
        s = SIFT4()
        self.assertEqual(0, s.distance("", ""))
        self.assertEqual(3, s.distance("", "abc"))
        self.assertEqual(3, s.distance("abc", ""))

    def test_options_attributes(self):
        options = SIFT4Options()
        self.assertEqual(0, options.maxdistance)
        self.assertTrue(callable(options.tokenizer))
        self.assertTrue(callable(options.tokenmatcher))
        self.assertTrue(callable(options.matchingevaluator))
        self.assertTrue(callable(options.locallengthevaluator))
        self.assertTrue(callable(options.transpositioncostevaluator))
        self.assertTrue(callable(options.transpositionsevaluator))

        options = SIFT4Options({"maxdistance": 10})
        self.assertEqual(10, options.maxdistance)

    def test_options_validation(self):
        with self.assertRaises(ValueError):
            SIFT4Options({"unknown": 1})
        with self.assertRaises(ValueError):
            SIFT4Options({"maxdistance": 1.5})
        with self.assertRaises(ValueError):
            SIFT4Options("not a dict")

    def test_custom_option_callables(self):
        calls = []

        def matching_evaluator(token1, token2):
            calls.append((token1, token2))
            return 1

        options = {"matchingevaluator": matching_evaluator}
        self.assertEqual(0, SIFT4().distance("abc", "abc", options=options))
        self.assertTrue(calls)

        options = {
            "tokenizer": "characterfrequency",
            "locallengthevaluator": "rewardlengthevaluator",
            "transpositioncostevaluator": "longertranspositionsaremorecostly",
            "transpositionsevaluator": lambda lcss, trans: lcss - trans,
        }
        self.assertIsInstance(SIFT4().distance("abc", "abd", options=options), int)

    def test_maxdistance_option(self):
        options = {"maxdistance": 2}
        self.assertLessEqual(SIFT4().distance("abcdef", "uvwxyz", options=options), 2)

    def test_options_setters(self):
        options = SIFT4Options()
        options.maxdistance = 5
        self.assertEqual(5, options.maxdistance)
        options.tokenizer = list
        self.assertIs(options.tokenizer, list)

    def test_static_helpers(self):
        self.assertEqual(["he", "el"], SIFT4Options.ngramtokenizer("hello", 2))
        self.assertEqual(
            ["hello", "world"],
            SIFT4Options.wordsplittokenizer("  hello\tworld\n"),
        )
        expected_counts = [0] * 26
        expected_counts[0] = 2  # a, A
        expected_counts[1] = 1  # b
        expected_counts[2] = 1  # c
        self.assertEqual(
            expected_counts,
            SIFT4Options.characterfrequencytokenizer("abcA"),
        )
        self.assertAlmostEqual(5.0 / 3.0, SIFT4Options.rewardlengthevaluator(2.0))
        self.assertAlmostEqual(8.0, SIFT4Options.rewardlengthevaluator2(4.0))
        self.assertAlmostEqual(
            2.0, SIFT4Options.longertranspositionsaremorecostly(0, 9)
        )
        self.assertAlmostEqual(
            1.0, SIFT4Options.sift4matchingevaluator("hello", "hello")
        )
        self.assertTrue(SIFT4Options.sift4tokenmatcher("hello", "hallo"))
        self.assertFalse(SIFT4Options.sift4tokenmatcher("cat", "dog"))


if __name__ == "__main__":
    unittest.main()
