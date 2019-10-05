import gensim
from nltk.tokenize import word_tokenize

class GensimSimilarity:
    def __init__(self):
        self.raw_documents = ["I'm taking the show on the road.",
            "My socks are a force multiplier.",
            "I am the barber who cuts everyone's hair who doesn't cut their own.",
            "Legend has it that the mind is a mad monkey.",
            "I make my own fun."]

    def getSimilarity(gen):
        gen_docs = [[w.lower() for w in word_tokenize(text)] 
                for text in gen.raw_documents]
        print(gen_docs)
        dictionary = gensim.corpora.Dictionary(gen_docs)
        print("Number of words in dictionary:",len(dictionary))

        for i in range(len(dictionary)):
            print(i, dictionary[i])

        corpus = [dictionary.doc2bow(gen_doc) for gen_doc in gen_docs]
        print(corpus)

        tf_idf = gensim.models.TfidfModel(corpus)
        print(tf_idf)
        s = 0
        for i in corpus:
            s += len(i)
        print(s)

        sims = gensim.similarities.Similarity('workdir/',tf_idf[corpus],num_features=len(dictionary))

        query_doc = [w.lower() for w in word_tokenize("Socks are a force for good.")]
        print(query_doc)
        query_doc_bow = dictionary.doc2bow(query_doc)
        print(query_doc_bow)
        query_doc_tf_idf = tf_idf[query_doc_bow]
        print(f'Result: {sims[query_doc_tf_idf]}')

similarity = GensimSimilarity()
similarity.getSimilarity()
