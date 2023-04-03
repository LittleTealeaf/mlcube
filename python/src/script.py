from agent import Agent
from mlcube import PyReplay2x2



replay = PyReplay2x2(1000)
replay.scramble(1000)
agent = Agent(replay, [400,300,300])

for i in range(10000):
    agent.step_experience( 1 - i / 100000)


print(replay.sample_replay(1000))
