import unittest

import strsimpy
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
    MetricStringDistance,
    NGram,
    NormalizedLevenshtein,
    NormalizedStringDistance,
    NormalizedStringSimilarity,
    OptimalStringAlignment,
    OverlapCoefficient,
    QGram,
    ShingleBased,
    SorensenDice,
    StringDistance,
    StringSimilarity,
    WeightedLevenshtein,
    shingle_profile,
)


class TestPublicApi(unittest.TestCase):

    def test_root_exports_every_public_class(self):
        for name in (
            "Cosine",
            "Damerau",
            "Jaccard",
            "JaroWinkler",
            "Levenshtein",
            "LongestCommonSubsequence",
            "MetricLCS",
            "NGram",
            "NormalizedLevenshtein",
            "OptimalStringAlignment",
            "OverlapCoefficient",
            "QGram",
            "SIFT4",
            "SIFT4Options",
            "SorensenDice",
            "WeightedLevenshtein",
        ):
            self.assertTrue(hasattr(strsimpy, name), name)
            self.assertIn(name, strsimpy.__all__)

    def test_per_module_imports_still_work(self):
        from strsimpy.cosine import Cosine as ModuleCosine
        from strsimpy.damerau import Damerau as ModuleDamerau
        from strsimpy.jaccard import Jaccard as ModuleJaccard
        from strsimpy.jaro_winkler import JaroWinkler as ModuleJaroWinkler
        from strsimpy.levenshtein import Levenshtein as ModuleLevenshtein
        from strsimpy.longest_common_subsequence import (
            LongestCommonSubsequence as ModuleLCS,
        )
        from strsimpy.metric_lcs import MetricLCS as ModuleMetricLCS
        from strsimpy.ngram import NGram as ModuleNGram
        from strsimpy.normalized_levenshtein import (
            NormalizedLevenshtein as ModuleNormalizedLevenshtein,
        )
        from strsimpy.optimal_string_alignment import (
            OptimalStringAlignment as ModuleOSA,
        )
        from strsimpy.overlap_coefficient import (
            OverlapCoefficient as ModuleOverlapCoefficient,
        )
        from strsimpy.qgram import QGram as ModuleQGram
        from strsimpy.shingle_based import ShingleBased as ModuleShingleBased
        from strsimpy.sift4 import SIFT4 as ModuleSIFT4
        from strsimpy.sift4 import SIFT4Options as ModuleSIFT4Options
        from strsimpy.sorensen_dice import SorensenDice as ModuleSorensenDice
        from strsimpy.string_distance import (
            MetricStringDistance as ModuleMetricStringDistance,
        )
        from strsimpy.string_distance import (
            NormalizedStringDistance as ModuleNormalizedStringDistance,
        )
        from strsimpy.string_distance import StringDistance as ModuleStringDistance
        from strsimpy.string_similarity import (
            NormalizedStringSimilarity as ModuleNormalizedStringSimilarity,
        )
        from strsimpy.string_similarity import (
            StringSimilarity as ModuleStringSimilarity,
        )
        from strsimpy.weighted_levenshtein import (
            WeightedLevenshtein as ModuleWeightedLevenshtein,
        )

        self.assertIs(ModuleCosine, Cosine)
        self.assertIs(ModuleDamerau, Damerau)
        self.assertIs(ModuleJaccard, Jaccard)
        self.assertIs(ModuleJaroWinkler, JaroWinkler)
        self.assertIs(ModuleLevenshtein, Levenshtein)
        self.assertIs(ModuleLCS, LongestCommonSubsequence)
        self.assertIs(ModuleMetricLCS, MetricLCS)
        self.assertIs(ModuleNGram, NGram)
        self.assertIs(ModuleNormalizedLevenshtein, NormalizedLevenshtein)
        self.assertIs(ModuleOSA, OptimalStringAlignment)
        self.assertIs(ModuleOverlapCoefficient, OverlapCoefficient)
        self.assertIs(ModuleQGram, QGram)
        self.assertIs(ModuleShingleBased, ShingleBased)
        self.assertIs(ModuleSIFT4, SIFT4)
        self.assertIs(ModuleSIFT4Options, SIFT4Options)
        self.assertIs(ModuleSorensenDice, SorensenDice)
        self.assertIs(ModuleMetricStringDistance, MetricStringDistance)
        self.assertIs(ModuleNormalizedStringDistance, NormalizedStringDistance)
        self.assertIs(ModuleStringDistance, StringDistance)
        self.assertIs(ModuleNormalizedStringSimilarity, NormalizedStringSimilarity)
        self.assertIs(ModuleStringSimilarity, StringSimilarity)
        self.assertIs(ModuleWeightedLevenshtein, WeightedLevenshtein)

    def test_isinstance_relationships(self):
        self.assertIsInstance(Levenshtein(), StringDistance)
        self.assertIsInstance(Levenshtein(), MetricStringDistance)
        self.assertIsInstance(Damerau(), MetricStringDistance)
        self.assertIsInstance(OptimalStringAlignment(), StringDistance)
        self.assertNotIsInstance(OptimalStringAlignment(), MetricStringDistance)
        self.assertIsInstance(NormalizedLevenshtein(), NormalizedStringDistance)
        self.assertIsInstance(NormalizedLevenshtein(), NormalizedStringSimilarity)
        self.assertIsInstance(JaroWinkler(), NormalizedStringSimilarity)
        self.assertIsInstance(NGram(2), NormalizedStringDistance)
        self.assertIsInstance(LongestCommonSubsequence(), StringDistance)
        self.assertIsInstance(WeightedLevenshtein(), StringDistance)
        self.assertIsInstance(SIFT4Options(), MetricStringDistance)

        jaccard = Jaccard(1)
        self.assertIsInstance(jaccard, ShingleBased)
        self.assertIsInstance(jaccard, MetricStringDistance)
        self.assertIsInstance(jaccard, NormalizedStringDistance)
        self.assertIsInstance(jaccard, NormalizedStringSimilarity)

        for shingle_metric in (
            QGram(1),
            Cosine(1),
            SorensenDice(1),
            OverlapCoefficient(1),
        ):
            self.assertIsInstance(shingle_metric, ShingleBased)

        self.assertNotIsInstance(SIFT4(), StringDistance)

    def test_base_classes_raise_not_implemented(self):
        with self.assertRaises(NotImplementedError):
            StringDistance().distance("a", "b")
        with self.assertRaises(NotImplementedError):
            NormalizedStringDistance().distance("a", "b")
        with self.assertRaises(NotImplementedError):
            MetricStringDistance().distance("a", "b")
        with self.assertRaises(NotImplementedError):
            StringSimilarity().similarity("a", "b")
        with self.assertRaises(NotImplementedError):
            NormalizedStringSimilarity().similarity("a", "b")

    def test_shingle_based_base_class_can_be_used_directly(self):
        base = ShingleBased(2)
        self.assertEqual(2, base.get_k())
        self.assertEqual({"ab": 1, "bc": 1}, base.get_profile("abc"))
        self.assertEqual({"ab": 1, "bc": 1}, shingle_profile("abc", 2))

    def test_version(self):
        self.assertEqual("0.4.1", strsimpy.__version__)

    def test_integral_distance_return_types(self):
        # The original Python implementation returned an int for non-identical
        # strings and 0.0 for identical strings. Keep that behaviour.
        self.assertIsInstance(Levenshtein().distance("a", "b"), int)
        self.assertIsInstance(Levenshtein().distance("a", "a"), float)
        self.assertIsInstance(Damerau().distance("a", "b"), int)
        self.assertIsInstance(OptimalStringAlignment().distance("a", "b"), int)
        self.assertIsInstance(LongestCommonSubsequence().distance("ab", "ac"), int)
        self.assertIsInstance(LongestCommonSubsequence().distance("a", "a"), float)
        self.assertIsInstance(QGram(2).distance("ab", "ac"), int)
        self.assertIsInstance(QGram(2).distance("ab", "ab"), float)
        self.assertIsInstance(SIFT4().distance("hello", "hallo"), int)

    def test_public_attributes(self):
        ngram = NGram(2)
        self.assertEqual(2, ngram.n)
        ngram.n = 3
        self.assertEqual(3, ngram.n)
        with self.assertRaises(ValueError):
            ngram.n = 0

        self.assertEqual(2, Cosine(2).get_k())
        self.assertEqual(2, Cosine(2).k)

        jaro_winkler = JaroWinkler()
        self.assertEqual(0.7, jaro_winkler.get_threshold())
        self.assertEqual(0.7, jaro_winkler.threshold)
        self.assertEqual(3, jaro_winkler.three)
        self.assertEqual(0.1, jaro_winkler.jw_coef)

        self.assertEqual(1, NormalizedLevenshtein().levenshtein.distance("a", "b"))
        self.assertEqual(2, MetricLCS().lcs.length("AGCAT", "GAC"))

        options = SIFT4Options({"maxdistance": 3})
        self.assertEqual(3, options.options["maxdistance"])
        self.assertTrue(callable(options.options["tokenizer"]))

    def test_native_module_path(self):
        self.assertEqual("strsimpy.levenshtein", Levenshtein.__module__)
        self.assertEqual("strsimpy.jaro_winkler", JaroWinkler.__module__)
        self.assertEqual("strsimpy.sift4", SIFT4Options.__module__)
        self.assertEqual("strsimpy.sift4", SIFT4.__module__)
        self.assertEqual(
            "strsimpy.weighted_levenshtein", WeightedLevenshtein.__module__
        )
        self.assertEqual("strsimpy.string_distance", StringDistance.__module__)
        self.assertEqual("strsimpy.string_similarity", StringSimilarity.__module__)
        self.assertEqual("strsimpy.shingle_based", ShingleBased.__module__)


if __name__ == "__main__":
    unittest.main()
