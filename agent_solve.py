from src import *
import tensorflow as tf

agent = _Agent([264, 202, 141, 80],"agents/1")

env = Environment()

random = Random()

for _ in range(40):
  env.apply_action(random.choice(ACTIONS))

while not env.is_complete():
  observations = env.to_observations_deprecated()
  action = agent.network.apply(tf.constant(np.array(observations),dtype=tf.float32))
  choice = tf.argmax(action,axis=1).numpy()[0]
  print(ACTIONS[choice].name)
  env.apply_action(ACTIONS[choice])
