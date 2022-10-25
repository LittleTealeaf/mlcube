from src import *

network = Network([1,2,3,4])
print(network.layers[2][0])
new_network = Network([1,2,3,4])
print(new_network.layers[2][0])
new_network.set(network)
print(new_network.layers[2][0])
