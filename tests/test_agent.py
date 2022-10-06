from src import *

def test_create_replay():
  agent = _Agent([5,10,15],"test")
  state_1,choice,state_2,rewards = agent.create_replay_deprecated(100)

  print(state_1)
