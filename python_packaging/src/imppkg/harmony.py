import sys

from termcolor import cprint

from imppkg.harmonic_mean import harmonic_mean


def main():
    result = 0.0
    numbers = []
    try:
        numbers = [float(num) for num in sys.argv[1:]]
    except ValueError:
        numbers = []
    try:
        result = harmonic_mean(numbers)
    except ZeroDivisionError:
        pass
    cprint(result, "red", "on_cyan", attrs=["bold"])


if __name__ == "__main__":
    main()
