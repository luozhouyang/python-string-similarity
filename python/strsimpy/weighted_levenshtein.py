"""Weighted Levenshtein distance."""

from ._strsimpy import WeightedLevenshtein


def default_insertion_cost(char):
    return 1.0


def default_deletion_cost(char):
    return 1.0


def default_substitution_cost(char_a, char_b):
    return 1.0


__all__ = [
    "WeightedLevenshtein",
    "default_deletion_cost",
    "default_insertion_cost",
    "default_substitution_cost",
]
