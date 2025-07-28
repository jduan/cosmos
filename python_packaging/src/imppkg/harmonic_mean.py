def harmonic_mean(numbers):
    total = 0
    for num in numbers:
        total += 1 / num
    return len(numbers) / total
