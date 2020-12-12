import sys


def get_dict(dict_file):
    with open(dict_file, "r") as fd:
        return fd.readlines()


def get_anagrams(dictionary, word):
    return [w.strip() for w in dictionary if sort_word(word) == sort_word(w.strip())]


def sort_word(word):
    return ''.join(sorted(list(word)))


def main():
    dict_file = "/usr/share/dict/words"
    dictionary = get_dict(dict_file)
    if len(sys.argv) == 2:
        word1 = sys.argv[1]
        anagrams1 = get_anagrams(dictionary, word1)
        for anagram in anagrams1:
            print(anagram)
    elif len(sys.argv) == 3:
        word1 = sys.argv[1]
        word2 = sys.argv[2]
        anagrams1 = get_anagrams(dictionary, word1)
        anagrams2 = get_anagrams(dictionary, word2)
        for (w1, w2) in zip(anagrams1, anagrams2):
            print("%s %s" % (w1, w2))


if __name__ == '__main__':
    main()
