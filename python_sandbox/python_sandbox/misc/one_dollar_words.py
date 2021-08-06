def main():
    # build a list of words from the dictionary
    words = []
    with open("/usr/share/dict/words") as fd:
        for word in fd:
            value = 0
            for c in word.rstrip():
                value += ord(c.lower()) - ord('a') + 1
            if value == 100:
                words.append(word.rstrip())

    words.sort(key=len)
    for word in words:
        print(word)


if __name__ == '__main__':
    main()
