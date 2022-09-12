from src import *
import time

agent = Agent([100,50],"test")

start = time.perf_counter()
agent.create_replay(count=10000)
end = time.perf_counter()

print(f'Elapsed: {end - start}')
