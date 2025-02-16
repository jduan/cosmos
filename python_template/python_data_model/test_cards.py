# This import is ok because it imports from cards.py which is in the same directory.
from cards import Card, FrenchDeck, spades_high
from random import choice
import sys


def test_cards():
    print(f"sys.path: {sys.path}")
    beer_card = Card("7", "diamonds")
    assert beer_card.rank == "7"
    assert beer_card.suit == "diamonds"


def test_deck():
    deck = FrenchDeck()
    assert len(deck) == 52
    assert deck[0] == Card("2", "spades")
    assert deck[-1] == Card("A", "hearts")
    # choice works with a sequence and FrenchDeck is a sequence because it implements
    # both __len__ and __getitem__
    print(f"Pick a random card: {choice(deck)}")
    print(f"Pick a random card: {choice(deck)}")

    # slicing works
    assert deck[:3] == [Card("2", "spades"), Card("3", "spades"), Card("4", "spades")]
    assert deck[-3:] == [Card("Q", "hearts"), Card("K", "hearts"), Card("A", "hearts")]

    # iteration works too because of __getitem__
    print("iterating cards")
    for card in deck:
        print(card)

    # in works too
    # If a collection has no __contains__ method, the "in" operator does a sequential scan
    assert Card("Q", "hearts") in deck
    assert Card("7", "beasts") not in deck

    # sort cards
    print("sorted cards")
    for card in sorted(deck, key=spades_high):
        print(card)

    # you can iterate the cards in reverse order
    print("reversed cards")
    for card in reversed(deck):
        print(card)
