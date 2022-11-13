def parallel_assignment():
    lax_coordinates = (33.9425, -118.408056)
    latitude, longitude = lax_coordinates
    assert latitude == 33.9425
    assert longitude == -118.408056

    # swap two variables
    latitude, longitude = longitude, latitude
    assert longitude == 33.9425
    assert latitude == -118.408056

    # prefix an argument with *
    assert divmod(20, 8) == (2, 4)
    t = (20, 8)
    assert divmod(*t) == (2, 4)


def all_about_stars():
    a, b, *rest = range(5)
    assert a == 0
    assert b == 1
    assert rest == [2, 3, 4]

    # star can appear in any position
    a, *body, c, d = range(5)
    assert a == 0
    assert body == [1, 2]
    assert c == 3
    assert d == 4


def star_function_calls():
    def fun(a, b, c, d, *rest):
        return a, b, c, d, rest

    a, b, c, d, rest = fun(*[1, 2], 3, *range(4, 7))
    assert a == 1
    assert b == 2
    assert c == 3
    assert d == 4
    assert rest == (5, 6)


def nested_unpacking():
    metro_areas = [
        ("Tokyo", "JP", 36.933, (35.689722, 139.691667)),
        ("Delhi", "IN", 21.935, (28.613889, 77.208889)),
        ("Mexico City", "MX", 20.142, (19.43333, -99.133333)),
        ("New York", "US", 20.104, (40.808611, -74.020386)),
        ("Sao Paulo", "BR", 19.649, (-23.547778, -46.635833)),
    ]
    print(f'{"name":<15} | {"latitude":>9} | {"longitude":>9}')
    for name, _, _, (lat, lon) in metro_areas:
        if lon <= 0:
            print(f"{name:15} | {lat:9.4f} | {lon:9.4f}")


def main():
    parallel_assignment()
    all_about_stars()
    star_function_calls()
    nested_unpacking()


if __name__ == "__main__":
    main()
