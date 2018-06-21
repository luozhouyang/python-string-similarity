from .string_distance import StringDistance


class CharacterInsDelInterface:

    def deletion_cost(self, c):
        raise NotImplementedError()

    def insertion_cost(self, c):
        raise NotImplementedError()


class CharacterSubstitutionInterface:

    def cost(self, c0, c1):
        raise NotImplementedError()


class WeightedLevenshtein(StringDistance):

    def __init__(self, character_substitution, character_ins_del=None):
        self.character_ins_del = character_ins_del
        if character_substitution is None:
            raise TypeError("Argument character_substitution is NoneType.")
        self.character_substitution = character_substitution

    def distance(self, s0, s1):
        if s0 is None:
            raise TypeError("Argument s0 is NoneType.")
        if s1 is None:
            raise TypeError("Argument s1 is NoneType.")
        if s0 == s1:
            return 0.0
        if len(s0) == 0:
            return len(s1)
        if len(s1) == 0:
            return len(s0)

        v0, v1 = [0.0] * (len(s1) + 1), [0.0] * (len(s1) + 1)

        v0[0] = 0
        for i in range(1, len(v0)):
            v0[i] = v0[i - 1] + self._insertion_cost(s1[i - 1])

        for i in range(len(s0)):
            s1i = s0[i]
            deletion_cost = self._deletion_cost(s1i)
            v1[0] = v0[0] + deletion_cost

            for j in range(len(s1)):
                s2j = s1[j]
                cost = 0
                if s1i != s2j:
                    cost = self.character_substitution.cost(s1i, s2j)
                insertion_cost = self._insertion_cost(s2j)
                v1[j + 1] = min(v1[j] + insertion_cost, v0[j + 1] + deletion_cost, v0[j] + cost)
            v0, v1 = v1, v0

        return v0[len(s1)]

    def _insertion_cost(self, c):
        if self.character_ins_del is None:
            return 1.0
        return self.character_ins_del.insertion_cost(c)

    def _deletion_cost(self, c):
        if self.character_ins_del is None:
            return 1.0
        return self.character_ins_del.deletion_cost(c)
