from src import *
import time

agent = Agent([100,50],"test")

dataset = agent.create_replay(count=100)
dataset = dataset.shuffle(50)
batch = dataset.batch(10)

