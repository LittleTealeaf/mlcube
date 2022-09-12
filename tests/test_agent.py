from src import *

def test_create_replay():
  agent = Agent([5,10,15],"test")
  agent.create_replay(100)
