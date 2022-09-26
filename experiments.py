from multiprocessing import Pool
from src import *

if __name__ == '__main__':
  agent = Agent([1,2,3,4,5],'agents/test-2')
  with Pool() as pool:
    calculate_rewards()
    agent.create_replay(replay_size=10_000,epsilon=0.2,moves_min=1,moves_max=40,pool=pool)
