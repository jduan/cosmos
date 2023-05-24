import yaml

email_message = """\
message:
  date: 2022-01-16 12:46:17Z
  from: john.doe@domain.com
  to:
    - bobby@domain.com
    - molly@domain.com
  cc:
    - jane.doe@domain.com
  subject: Friendly reminder
  content: |
    Dear XYZ,

    Lorem ipsum dolor sit amet...
  attachments:
    image1.gif: !!binary
        R0lGODdhCAAIAPAAAAIGAfr4+SwAA
        AAACAAIAAACDIyPeWCsClxDMsZ3CgA7
"""


def get_subject():
    msg = yaml.safe_load(email_message)
    return msg['message']['subject']


def dict_tricks():
    # you can merge two dicts
    cities_us = {'New York City': 'US', 'Los Angeles': 'US'}
    cities_uk = {'Londo': 'UK', 'Birmingham': 'UK'}
    print(f"cities_us and cities_uk merged: {cities_us | cities_uk}")
    # This is an in-place union
    cities_us |= cities_uk
    print(f"cities_us: {cities_us}")

    # before python 3.9, you can merge two dicts using dictionary unpacking
    cities_us = {'New York City': 'US', 'Los Angeles': 'US'}
    cities_uk = {'Londo': 'UK', 'Birmingham': 'UK'}
    cities = {**cities_us, **cities_uk}
    print(f"cities: {cities}")

    # use dict comprehension to create another dict
    cities = ['London', 'New York', 'Tokyo', 'Cambridge', 'Oxford']
    countries = ['UK', 'US', 'Japan', 'UK', 'UK']
    uk_cities = {city: country for city, country in zip(cities, countries) if country == "UK"}
    print(f"uk_cities: {uk_cities}")

    # reverse keys and values of a dict (method 1)
    cities = {'London': 'UK', 'Tokyo': 'Japan', 'New York': 'US'}
    reversed_cities = {v: k for k, v in cities.items()}
    print(f"reversed_cities: {reversed_cities}")

    # reverse keys and values of a dict (method 2)
    reversed_cities = dict(zip(cities.values(), cities.keys()))
    print(f"reversed_cities: {reversed_cities}")

    # convert a list of pairs into a dict
    cities = [('London', 'UK'), ('New York', 'US'), ('Tokyo', 'Japan')]
    d_cities = dict(cities)
    print(f"d_cities: {d_cities}")

    # sort a dict
    cities = {'London': '2', 'Tokyo': '3', 'New York': '1'}
    print(sorted(cities.items(), key=lambda d: d[1]))

    # use default dict
    from collections import defaultdict
    city = defaultdict(str)
    city['UK'] = 'London'
    print(city['Italy'])

    # using the Counter class
    # Counter is a subclass of dict that’s specially designed for counting hashable objects in Python. It’s a dictionary
    # that stores objects as keys and counts as values. To count with Counter, you typically provide a sequence or
    # iterable of hashable objects as an argument to the class’s constructor.
    from collections import Counter
    author = "Jingjing Duan"
    chars = Counter(author)
    print(f"chars: {chars}")
