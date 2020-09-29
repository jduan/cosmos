#!/usr/bin/env python3
# This script takes username and password, and prints the base64 encoding of
# the pair so it can be used in HTTP Base Auth.

import base64
import sys

username = sys.argv[1]
password = sys.argv[2]
pair = "%s:%s" % (username, password)
print("Basic %s" % base64.b64encode(pair.encode()).decode(), end = '')
