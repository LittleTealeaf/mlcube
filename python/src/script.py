from agent import Agent
from mlcube import PyReplay2x2
from database import Database


replay = PyReplay2x2(10_000)
replay.scramble(1000)
agent = Agent('test-abcd', replay,[400,300,300], database=Database())

for i in range(10_000):
    agent.step_experience(0.1)

agent.train(100, 0.001, 0.8)
agent.save()
