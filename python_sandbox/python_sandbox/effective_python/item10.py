"""
In Python, there are two types that represent sequences of character data: bytes
and str. Instances of bytes contain raw, unsigned 8-bit values (often displayed
in ASCII encoding).

Instances of str contain Unicode code points that represent textual characters from human languages.

Things to remember:

    bytes contains sequences of 8-bit values, and str contains sequences of Unicode code points.

    Use helper functions to ensure that the inputs you operate on are the type
    of character sequence you expect (8-bit values, UTF-8-encoded strings,
    Unicode code points, etc).

    bytes and str instances can’t be used together with operators (like >, ==, +, and %).

    If you want to read or write binary data to/from a file, always open the
    file using a binary mode (like "rb" or "wb").

    If you want to read or write Unicode data to/from a file, be careful about
    your system’s default text encoding. Explicitly pass the encoding parameter
    to open to avoid surprises.
"""


def to_str(bytes_or_str):
    if isinstance(bytes_or_str, bytes):
        value = bytes_or_str.decode("utf-8")
    else:
        value = bytes_or_str
    return value


def to_bytes(bytes_or_str):
    if isinstance(bytes_or_str, str):
        value = bytes_or_str.encode("utf-8")
    else:
        value = bytes_or_str
    return value


# if you use "w", it won't work because the file is opened in write text mode
# when a file is in text mode, you can only write str, not bytes
with open("data.bin", "wb") as f:
    f.write(b"\xf1\xf2\xf3\xf4\xf5")

# if you use "r", it won't work because the file is opened in read text mode
# when a file is in text mode, you can only read str, not bytes
with open("data.bin", "rb") as f:
    data = f.read()
    assert data == b"\xf1\xf2\xf3\xf4\xf5"

# read the file encoded in cp1252 format (a legacy Windows encoding)
with open("data.bin", "r", encoding="cp1252") as f:
    data = f.read()
    assert data == "ñòóôõ"
