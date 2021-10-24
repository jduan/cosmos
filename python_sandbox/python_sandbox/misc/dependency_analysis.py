# This script takes in a file that's part of a "dot" file output which contains edge info, eg:
#   "com.bennet:proto-auth:jar:compile" -> "com.bennet:proto-error:jar:compile"
#   "com.bennet:proto-auth:jar:compile" -> "com.bennet:proto-common:jar:compile"
#   "com.bennet:proto-issuer-transaction:jar:compile" -> "com.bennet:proto-error:jar:compile"
#   "com.bennet:proto-issuer-transaction:jar:compile" -> "com.bennet:proto-common:jar:compile"
#   "com.bennet:proto-tokenization:jar:compile" -> "com.bennet:proto-error:jar:compile"
#   "com.bennet:proto-vault:jar:compile" -> "com.bennet:proto-error:jar:compile"
#   "com.bennet:proto-vault:jar:compile" -> "com.bennet:proto-common:jar:compile"
#   "com.bennet:proto-identity:jar:compile" -> "com.bennet:proto-error:jar:compile"
#   "com.bennet:proto-identity:jar:compile" -> "com.bennet:proto-common:jar:compile"
#   "com.bennet:proto-issuance:jar:compile" -> "com.bennet:proto-common:jar:compile"
#
# It will analyze the graph info and find nodes that are mostly depended on and other things.

import re
import sys

LINE_REGEX = r'"(.*?)" -> "(.*?)"'

graph = {}


class Node(object):
    def __init__(self, name):
        self.name = name
        self.dependencies = []
        self.dependents = []

    def depends_on(self, other):
        self.dependencies.append(other)
        other.dependents.append(self)

    def incoming_degree(self):
        return len(self.dependents)

    def outgoing_degree(self):
        return len(self.dependencies)


class Graph(object):
    def __init__(self):
        self.nodes = {}

    def get_node(self, name):
        if name not in self.nodes:
            node = Node(name)
            self.nodes[name] = node
        return self.nodes[name]

    def size(self):
        return len(self.nodes)


graph = Graph()
for line in sys.stdin:
    match = re.search(LINE_REGEX, line)
    from_node = graph.get_node(match.group(1))
    to_node = graph.get_node(match.group(2))
    from_node.depends_on(to_node)

print("name,outgoing_degree,incoming_degree")
for node in graph.nodes.values():
    print("%s,%s,%s" % (node.name, node.outgoing_degree(), node.incoming_degree()))