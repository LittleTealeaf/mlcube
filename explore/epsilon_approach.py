from matplotlib import pyplot as plt


def iterate(found: float, expected: float, alpha: float) -> float:
    diff = expected - found
    diff *= alpha
    return found + diff


found = 0
expected = 100


data = []

for i in range(100):
    found = iterate(found, expected, 0.1 * 0.95 ** (i + 1))
    data.append(found)

print(data[-1])

# plt.plot(data)
# plt.show()
