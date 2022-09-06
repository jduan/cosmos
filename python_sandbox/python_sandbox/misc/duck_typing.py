class TheHobbit:
    def __len__(self):
        return 95022


def main():
    the_hobbit = TheHobbit()
    print('Hello, world!')
    print(f"the length of the hobbit: {len(the_hobbit)}")


if __name__ == '__main__':
    main()
