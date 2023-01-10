from environment import *
from tf_agents.environments import utils
from random import Random

random = Random()

env = CubeEnvironment(seed = 100)
time_step = env.reset()
print(time_step)
cumulative_reward = time_step.reward

for _ in range(10000):
    choice = random.randint(0,18)
    print("Choice is ", choice)
    time_step = env.step(np.array(choice, dtype=np.int32))
    print(time_step)
    cumulative_reward += time_step.reward

    if time_step.is_last():
        break

print("Final Reward = ", cumulative_reward)
