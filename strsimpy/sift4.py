# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
from .string_distance import MetricStringDistance


class SIFT4Options(MetricStringDistance):
    def __init__(self, options=None):
        self.options = {
            'maxdistance': 0,
            'tokenizer': lambda x: [i for i in x],
            'tokenmatcher': lambda t1, t2: t1 == t2,
            'matchingevaluator': lambda t1, t2: 1,
            'locallengthevaluator': lambda x: x,
            'transpositioncostevaluator': lambda c1, c2: 1,
            'transpositionsevaluator': lambda lcss, trans: lcss - trans
        }
        otheroptions = {
            'tokenizer': {
                'ngram': self.ngramtokenizer,
                'wordsplit': self.wordsplittokenizer,
                'characterfrequency': self.characterfrequencytokenizer
            },
            'tokematcher': {'sift4tokenmatcher': self.sift4tokenmatcher},
            'matchingevaluator': {'sift4matchingevaluator': self.sift4matchingevaluator},
            'locallengthevaluator': {
                'rewardlengthevaluator': self.rewardlengthevaluator,
                'rewardlengthevaluator2': self.rewardlengthevaluator2
            },
            'transpositioncostevaluator': {'longertranspositionsaremorecostly':self.longertranspositionsaremorecostly},
            'transpositionsevaluator': {}
        }
        if isinstance(options, dict):
            for k, v in options.items():
                if k in self.options.keys():
                    if k == 'maxdistance':
                        if isinstance(v, int):
                            self.options[k] = v
                        else:
                            raise ValueError("Option maxdistance should be int")
                    else:
                        if callable(v):
                            self.options[k] = v
                        else:
                            if v in otheroptions[k].keys():
                                self.options[k] = otheroptions[k][v]
                            else:
                                msg = "Option {} should be callable or one of [{}]".format(k, ', '.join(otheroptions[k].keys()))
                                raise ValueError(msg)
                else:
                    raise ValueError("Option {} not recognized.".format(k))
        elif options is not None:
            raise ValueError("options should be a dictionary")
        self.maxdistance = self.options['maxdistance']
        self.tokenizer = self.options['tokenizer']
        self.tokenmatcher = self.options['tokenmatcher']
        self.matchingevaluator = self.options['matchingevaluator']
        self.locallengthevaluator = self.options['locallengthevaluator']
        self.transpositioncostevaluator = self.options['transpositioncostevaluator']
        self.transpositionsevaluator = self.options['transpositionsevaluator']

    # tokenizers:
    @staticmethod
    def ngramtokenizer(s, n):
        result = []
        if not s:
            return result
        for i in range(len(s) - n - 1):
            result.append(s[i:(i + n)])
        return result

    @staticmethod
    def wordsplittokenizer(s):
        if not s:
            return []
        return s.split()

    @staticmethod
    def characterfrequencytokenizer(s):
        letters = [i for i in 'abcdefghijklmnopqrstuvwxyz']
        return [s.lower().count(x) for x in letters]

    # tokenMatchers:
    @staticmethod
    def sift4tokenmatcher(t1, t2):
        similarity = 1 - SIFT4().distance(t1, t2, 5) / max(len(t1), len(t2))
        return similarity > 0.7

    # matchingEvaluators:
    @staticmethod
    def sift4matchingevaluator(t1, t2):
        similarity = 1 - SIFT4().distance(t1, t2, 5) / max(len(t1), len(t2))
        return similarity

    # localLengthEvaluators:
    @staticmethod
    def rewardlengthevaluator(l):
        if l < 1:
            return l
        return l - 1 / (l + 1)

    @staticmethod
    def rewardlengthevaluator2(l):
        return pow(l, 1.5)

    # transpositionCostEvaluators:
    @staticmethod
    def longertranspositionsaremorecostly(c1, c2):
        return abs(c2 - c1) / 9 + 1


class SIFT4:
    # As described in https://siderite.dev/blog/super-fast-and-accurate-string-distance.html/
    def distance(self, s1, s2, maxoffset=5, options=None):
        options = SIFT4Options(options)
        t1, t2 = options.tokenizer(s1), options.tokenizer(s2)
        l1, l2 = len(t1), len(t2)
        if l1 == 0:
            return l2
        if l2 == 0:
            return l1

        c1, c2, lcss, local_cs, trans, offset_arr = 0, 0, 0, 0, 0, []
        while (c1 < l1) and (c2 < l2):
            if options.tokenmatcher(t1[c1], t2[c2]):
                local_cs += options.matchingevaluator(t1[c1], t2[c2])
                isTrans = False
                i = 0
                while i < len(offset_arr):
                    ofs = offset_arr[i]
                    if (c1 <= ofs['c1']) or (c2 <= ofs['c2']):
                        isTrans = abs(c2 - c1) >= abs(ofs['c2'] - ofs['c1'])
                        if isTrans:
                            trans += options.transpositioncostevaluator(c1, c2)
                        else:
                            if not ofs['trans']:
                                ofs['trans'] = True
                                trans += options.transpositioncostevaluator(ofs['c1'], ofs['c2'])
                        break
                    else:
                        if (c1 > ofs['c2']) and (c2 > ofs['c1']):
                            offset_arr.pop(i)
                        else:
                            i += 1
                offset_arr.append({'c1': c1, 'c2': c2, 'trans': isTrans})
            else:
                lcss += options.locallengthevaluator(local_cs)
                local_cs = 0
                if c1 != c2:
                    c1 = c2 = min(c1, c2)
                for i in range(maxoffset):
                    if (c1 + i < l1) or (c2 + i < l2):
                        if (c1 + i < l1) and options.tokenmatcher(t1[c1 + i], t2[c2]):
                            c1 += i - 1
                            c2 -= 1
                            break
                    if (c2 + i < l2) and options.tokenmatcher(t1[c1], t2[c2 + i]):
                        c1 -= 1
                        c2 += i - 1
                        break
            c1 += 1
            c2 += 1
            if options.maxdistance:
                temporarydistance = options.locallengthevaluator(max(c1, c2)) - options.transpositionsevaluator(lcss, trans)
                if temporarydistance >= options.maxdistance:
                    return round(temporarydistance)
            if (c1 >= l1) or (c2 >= l2):
                lcss += options.locallengthevaluator(local_cs)
                local_cs = 0
                c1 = c2 = min(c1, c2)
        lcss += options.locallengthevaluator(local_cs)
        return round(options.locallengthevaluator(max(l1, l2)) - options.transpositionsevaluator(lcss, trans))


