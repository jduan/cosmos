from collections import defaultdict


class FrequencyList(list):
    def __init__(self, members):
        super().__init__(members)

    def frequency(self):
        counts = defaultdict(int)
        for item in self:
            counts[item] += 1
        return counts


class BinaryNode:
    def __init__(self, value, left=None, right=None):
        self.value = value
        self.left = left
        self.right = right


class Count:
    def __init__(self):
        self.value = 0

    def increment(self):
        self.value += 1


class IndexableNode(BinaryNode):
    def _search(self, count, index):
        if self.left:
            self.left._search(count, index)
        if count.value == index:
            return self.value
        count.increment()
        # visit self
        if self.right:
            count.increment()
            self.right._search(count, index)

    def __getitem__(self, index):
        found = self._search(Count(), index)
        if not found:
            raise IndexError('Index out of range')
        return found


def main():
    foo = FrequencyList(['a', 'b', 'a', 'c', 'b', 'a', 'd'])
    assert len(foo) == 7
    foo.pop()
    assert foo.frequency() == {'a': 3, 'b': 2, 'c': 1}

    tree = IndexableNode(
        10,
        left=IndexableNode(
            5,
            left=IndexableNode(2),
            right=IndexableNode(
                6, right=IndexableNode(7))),
        right=IndexableNode(
            15, left=IndexableNode(11)))
    print('Index 0: ', tree[0])


if __name__ == '__main__':
    main()
