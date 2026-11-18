import pickle
import unittest
from multiprocessing import get_context

from strsimpy import (
    SIFT4,
    SIFT4Options,
    Cosine,
    Damerau,
    Jaccard,
    JaroWinkler,
    Levenshtein,
    LongestCommonSubsequence,
    MetricLCS,
    NGram,
    NormalizedLevenshtein,
    OptimalStringAlignment,
    OverlapCoefficient,
    QGram,
    SorensenDice,
    WeightedLevenshtein,
)


def scaled_substitution_cost(char_a, char_b):
    return 2.0 if char_a != char_b else 0.0


class TestPickle(unittest.TestCase):

    def test_stateless_calculators_roundtrip(self):
        calculators = [
            Levenshtein(),
            NormalizedLevenshtein(),
            Damerau(),
            OptimalStringAlignment(),
            JaroWinkler(),
            LongestCommonSubsequence(),
            MetricLCS(),
            NGram(),
            QGram(),
            Cosine(1),
            Jaccard(1),
            SorensenDice(1),
            OverlapCoefficient(1),
            SIFT4(),
        ]
        for calculator in calculators:
            with self.subTest(calculator=type(calculator).__name__):
                restored = pickle.loads(pickle.dumps(calculator))
                self.assertIs(type(restored), type(calculator))
                self.assertEqual(
                    calculator.distance("martha", "marhta"),
                    restored.distance("martha", "marhta"),
                )

    def test_constructor_options_are_retained(self):
        jaro_winkler = pickle.loads(pickle.dumps(JaroWinkler(0.5)))
        self.assertEqual(0.5, jaro_winkler.get_threshold())
        self.assertEqual(0.5, jaro_winkler.threshold)

        ngram = pickle.loads(pickle.dumps(NGram(3)))
        self.assertEqual(3, ngram.n)

        for calculator_class in (
            QGram,
            Cosine,
            Jaccard,
            SorensenDice,
            OverlapCoefficient,
        ):
            with self.subTest(calculator=calculator_class.__name__):
                restored = pickle.loads(pickle.dumps(calculator_class(4)))
                self.assertEqual(4, restored.get_k())

    def test_weighted_levenshtein_keeps_cost_functions(self):
        calculator = WeightedLevenshtein(substitution_cost_fn=scaled_substitution_cost)
        restored = pickle.loads(pickle.dumps(calculator))
        self.assertEqual(2.0, restored.distance("abc", "adc"))
        self.assertEqual(
            calculator.distance("abc", "adc"), restored.distance("abc", "adc")
        )

        default = pickle.loads(pickle.dumps(WeightedLevenshtein()))
        self.assertEqual(1.0, default.distance("abc", "adc"))

    def test_sift4_options_roundtrip(self):
        options = SIFT4Options({"maxdistance": 3, "tokenizer": "ngram"})
        restored = pickle.loads(pickle.dumps(options))
        self.assertEqual(3, restored.maxdistance)
        self.assertEqual(
            SIFT4Options.ngramtokenizer("abcd"), restored.tokenizer("abcd")
        )

    def test_bound_distance_method_can_be_pickled(self):
        distance = pickle.loads(pickle.dumps(Levenshtein().distance))
        self.assertEqual(1, distance("abc", "adc"))

    def test_multiprocessing_pool_can_use_bound_distance(self):
        context = get_context("spawn")
        with context.Pool(1) as pool:
            self.assertEqual(
                [1], pool.starmap(Levenshtein().distance, [("abc", "adc")])
            )


if __name__ == "__main__":
    unittest.main()
