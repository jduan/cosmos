# https://leetcode.com/problems/validate-ip-address/

class IPv4Validator(object):
    def is_valid(self, part: str) -> bool:
        # invalid if part isn't a number
        try:
            parti = int(part)
        except ValueError:
            return False

        if parti < 0 or parti > 255:
            return False

        # "01" is invalid
        if part.startswith("0") and parti != 0:
            return False

        # "00" is invalid but "0" is valid
        if parti == 0 and len(part) > 1:
            return False

        return True


    def validate_addr(self, ip: str) -> bool:
        parts = ip.split(".")
        if len(parts) != 4:
            return False

        return all([self.is_valid(part) for part in parts])


def main():
    valid_addrs = [
        "172.16.254.1",
        "192.168.1.1",
        "192.168.1.0",
    ]
    invalid_addrs = [
        "192.168.01.1",
        "192.168.1.00",
        "192.168@1.1",
        "192.168.@1.1",
        "256.256.256.256",
        "-192.168.1.0",
        "192..1.0",
    ]

    validator = IPv4Validator()

    print("checking valid addresses")
    for addr in valid_addrs:
        print(validator.validate_addr(addr))

    print("checking invalid addresses")
    for addr in invalid_addrs:
        print(validator.validate_addr(addr))


if __name__ == '__main__':
    main()
