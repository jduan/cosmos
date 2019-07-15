import collections
from random import choice

Card = collections.namedtuple('Card', ['rank', 'suit'])


class FrenchDeck:
    ranks = [str(n) for n in range(2, 11)] + list('JQKA')
    suits = 'spades diamonds clubs hearts'.split()

    def __init__(self):
        self._cards = [Card(rank, suit) for suit in self.suits
                       for rank in self.ranks]

    def __len__(self):
        return len(self._cards)

    def __getitem__(self, position):
        return self._cards[position]


suit_values = dict(spades=3, hearts=2, diamonds=1, clubs=0)


def spades_high(card):
    rank_value = FrenchDeck.ranks.index(card.rank)
    return rank_value * len(suit_values) + suit_values[card.suit]


def main():
    beer_card = Card('7', 'diamonds')
    print(beer_card)

    deck = FrenchDeck()
    print('Number of cards', len(deck))
    print('First card', deck[0])
    print('Last card', deck[-1])
    # choice works with a sequence and FrenchDeck is a sequence because it implements
    # both __len__ and __getitem__
    print('Pick a random card', choice(deck))
    print('Pick a random card', choice(deck))

    # slicing works
    print('First 3 cards', deck[:3])
    print('Last 3 cards', deck[-3:])

    # iteration works too because of __getitem__
    for card in deck:
        print(card)

    # in works too
    # If a collection has no __contains__ method, the "in" operator does a sequential scan
    print(Card('Q', 'hearts') in deck)
    print(Card('7', 'beasts') in deck)

    # sort cards
    for card in sorted(deck, key=spades_high):
        print(card)


if __name__ == '__main__':
    main()
