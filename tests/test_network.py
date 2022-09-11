from src import *

def test_new_network():
  assert Network([10,10]) is not None

def test_proper_layers():
  network = Network([5,10,15])
  assert len(network.layers) == 4

def test_applies_to_environment():
  env = Environment()
  env_observations = env.to_observations()
  env_tensor = tf.constant(env_observations, dtype=tf.float32)
  network = Network([10,10])

  result = network.apply(env_tensor)

  assert result != None


def test_copy():
  network = Network([15,15])
  network_copy = network.copy()

  for i in range(len(network.layers)):
    # n is the network, c is the copy
    n_W, n_b = network.layers[i]
    c_W, c_b = network_copy.layers[i]
    assert np.array_equal(n_W.numpy(), c_W.numpy())
    assert np.array_equal(n_b.numpy(), c_b.numpy())

def test_serialize():
  network = Network([1,2,3,4])
  serialized = network.serialize()
  deserialized = Network([1,2,3,4],serialized=serialized)

  for i in range(len(network.layers)):
    n_W, n_b = network.layers[i]
    d_W, d_b = deserialized.layers[i]
    assert np.array_equal(n_W.numpy(), d_W.numpy())
    assert np.array_equal(n_b.numpy(), d_b.numpy())
