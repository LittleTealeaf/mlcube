INPUTS = 9 * 6 * 6
OUTPUTS = 18


LAYER_COUNT = 5

RANGE = INPUTS - OUTPUTS

print(RANGE)

DIFF = RANGE // LAYER_COUNT
print(DIFF)


LAYERS = []

for i in range(1,LAYER_COUNT):
  LAYERS.append(INPUTS - DIFF * i)
print(LAYERS)

# [264, 202, 141, 80]
