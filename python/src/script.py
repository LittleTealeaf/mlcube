from agent import Agent
from mlcube import PyReplay2x2
from database import Database


replay = PyReplay2x2(1000)
replay.scramble(1000)
agent = Agent('test-abcd', replay,[400,300,300], database=Database())
agent.save()
