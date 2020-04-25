import unittest


class Node:
    def __init__(self, key, value, prev=None, next=None):
        # We need the key so we can go back to the dictionary to remove it when this key is evicted.
        self.key = key
        self.value = value
        self.prev = prev
        self.next = next

    def __str__(self):
        return "Node(key: %s, value: %s)" % (self.key, self.value)

    def __eq__(self, other):
        return self.key == other.key


class LRUCache:
    """
    Note that every time the linked list's head or tail is changed, you need to set:
    head.prev = None
    tail.next = None

    Otherwise, you may end up with having loops!
    """
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.dict = {}
        self.head = None
        self.tail = None
        self.size = 0

    # Find node, move it to the end, and return its value
    def get(self, key: int) -> int:
        if key not in self.dict:
            return -1
        else:
            node = self.dict[key]
            prev_node = node.prev
            next_node = node.next
            # node is head
            if prev_node is None:
                if node != self.tail:
                    self.head = next_node
                    next_node.prev = None
                    self.tail.next = node
                    node.prev = self.tail
                    node.next = None
                    self.tail = node
            else:
                if node != self.tail:
                    prev_node.next = next_node
                    next_node.prev = prev_node
                    self.tail.next = node
                    node.prev = self.tail
                    node.next = None
                    self.tail = node
            return node.value

    # If key already exists, find its node, move it to the end, and update its value
    # If key doesn't exist yet, add a new node to the end, and increment size by 1. Evict oldest
    # item if needed.
    def put(self, key: int, value: int) -> None:
        if self.get(key) is -1:
            # append a new node to the end
            node = Node(key, value)
            if self.tail is None:
                self.head = node
                self.tail = node
            else:
                self.tail.next = node
                node.prev = self.tail
                self.tail = node
            self.dict[key] = node

            self.size += 1
            if self.size > self.capacity:
                self.size -= 1
                del self.dict[self.head.key]
                self.head = self.head.next
                self.head.prev = None
        else:
            # after calling self.get(key), it already got moved to the end of the list
            self.tail.value = value
            self.dict[key] = self.tail

    def print_debug_info(self, msg):
        print(msg)
        node = self.head
        while node is not None:
            print(node)
            node = node.next


class TestLRUCache(unittest.TestCase):
    def test1(self):
        cache = LRUCache(2)
        cache.print_debug_info("just initialized")
        cache.put(1, 1)
        cache.print_debug_info("put 1")
        cache.put(2, 2)
        cache.print_debug_info("put 2")
        self.assertEqual(1, cache.get(1))
        cache.print_debug_info("get 1")
        cache.put(3, 3)
        cache.print_debug_info("put 3")
        self.assertEqual(-1, cache.get(2))

    def test2(self):
        cache = LRUCache(1)
        cache.print_debug_info("just initialized")
        cache.put(2, 1)
        cache.print_debug_info("put 2")
        self.assertEqual(1, cache.get(2))
        cache.print_debug_info("get 2")
        cache.put(3, 2)
        cache.print_debug_info("put 3")
        self.assertEqual(-1, cache.get(2))
        cache.print_debug_info("get 2")
        self.assertEqual(2, cache.get(3))
        cache.print_debug_info("get 3")


if __name__ == '__main__':
    unittest.main()
