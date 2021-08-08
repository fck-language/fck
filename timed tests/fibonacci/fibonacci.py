def fibonacci(n):
    if n < 2:
        return n
    else:
        previous = 0
        out = 1
        stored = 1
        for _ in range(n - 1):
            stored = out
            out += previous
            previous = stored
        return out

print(fibonacci(100))
