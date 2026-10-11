import unittest

from .sift4 import SIFT4, SIFT4Options


class SIFT4Test(unittest.TestCase):

    def test_named_token_matcher(self):
        strings = ('hello world', 'hallo world')
        named = {'tokenizer': 'wordsplit', 'tokenmatcher': 'sift4tokenmatcher'}
        callable_option = {
            'tokenizer': 'wordsplit',
            'tokenmatcher': SIFT4Options.sift4tokenmatcher,
        }
        self.assertEqual(SIFT4().distance(*strings, options=callable_option), 0)
        self.assertEqual(SIFT4().distance(*strings, options=named), 0)
        self.assertEqual(SIFT4().distance(
            'cat dog', 'cat pig', options=named), 1)

    def test_unknown_named_token_matcher(self):
        with self.assertRaisesRegex(ValueError, 'sift4tokenmatcher'):
            SIFT4Options({'tokenmatcher': 'unknown'})

    def testSIFT4(self):
        s = SIFT4()
        
        results = [
            ('This is the first string', 'And this is another string', 5, 11.0),
            ('Lorem ipsum dolor sit amet, consectetur adipiscing elit.', 'Amet Lorm ispum dolor sit amet, consetetur adixxxpiscing elit.', 10, 12.0)
        ]

        for a, b, offset, res in results:
            self.assertEqual(res, s.distance(a, b, maxoffset=offset))


if __name__ == "__main__":
    unittest.main()
