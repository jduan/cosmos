# https://leetcode.com/problems/validate-ip-address/

valid_hexes = set()
for i in range(0, 10):
    valid_hexes.add(str(i))
valid_hexes.add('a')
valid_hexes.add('b')
valid_hexes.add('c')
valid_hexes.add('d')
valid_hexes.add('e')
valid_hexes.add('f')


class IPv6Validator(object):
    def is_valid(self, part: str) -> bool:
        length = len(part)

        if length < 1 or length > 4:
            return False

        return all([ch.lower() in valid_hexes for ch in part])


    def validate_addr(self, ip: str) -> bool:
        parts = ip.split(":")
        if len(parts) != 8:
            return False

        return all([self.is_valid(part) for part in parts])


def main():
    valid_addrs = [
        "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
        "2001:db8:85a3:0:0:8A2E:0370:7334",
    ]
    invalid_addrs = [
        "2001:0db8:85a3::8A2E:037j:7334",
        "02001:0db8:85a3:0000:0000:8a2e:0370:7334",
    ]

    validator = IPv6Validator()

    print("checking valid addresses")
    for addr in valid_addrs:
        print(validator.validate_addr(addr))

    print("checking invalid addresses")
    for addr in invalid_addrs:
        print(validator.validate_addr(addr))


if __name__ == '__main__':
    main()
