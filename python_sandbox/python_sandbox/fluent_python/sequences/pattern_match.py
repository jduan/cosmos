def handle_command(message):
    match message:
        # this matches any sequence with 3 items: the first one must be "BEEPER", the 2nd and 3rd can be anything
        # and they will be bound to variables "frequency" and "times"
        case ["BEEPER", frequency, times]:
            print(f"Going to call beep, frequency: {frequency}, times: {times})")
        case ["NECK", angle]:
            print(f"Going to rotate neck: {angle}")
        # This will never match since the one above is the same
        case ["NECK", times]:
            print(f"Going to rotate times: {times}")
        case ["LED", ident, intensity]:
            print(f"Going to call leds: ident: {ident}, intensity: {intensity}")
        case ["LED", ident, red, green, blue]:
            print(f"Going to call leds: ident: {ident}, red: {red}, green: {green}, blue: {blue}")
        case _:
            print("Invalid command")


def destruct_nested_tuples():
    metro_areas = [
        ("Tokyo", "JP", 36.933, (35.689722, 139.691667)),
        ("Delhi", "IN", 21.935, (28.613889, 77.208889)),
        ("Mexico City", "MX", 20.142, (19.43333, -99.133333)),
        ("New York", "US", 20.104, (40.808611, -74.020386)),
        ("Sao Paulo", "BR", 19.649, (-23.547778, -46.635833)),
    ]
    print(f'{"name":<15} | {"latitude":>9} | {"longitude":>9}')
    for record in metro_areas:
        match record:
            case [name, _, _, (lat, lon)] if lon <= 0:
                print(f"{name:15} | {lat:9.4f} | {lon:9.4f}")


def main():
    handle_command(["BEEPER", 99, 3])
    handle_command(["NECK", 90])
    handle_command(["LED", 99, 3])
    handle_command(["LED", 99, 255, 255, 255])
    handle_command(["WTH", 99, 3])
    destruct_nested_tuples()


if __name__ == "__main__":
    main()
