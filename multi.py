import time
from src import *
import tensorflow as tf
from multiprocessing import Manager, Pool


def exponential_decay(initial, index, decay_rate, decay_interval):
    return initial * (decay_rate ** (index // decay_interval))

if __name__ == '__main__':
  with Pool() as pool:
    calculate_rewards(5)
    proc_manager = Manager()
    m_rewards = proc_manager.dict(REWARDS)
    agent = Agent([264, 202, 141, 80],'agents/tmp-1')
    target_interval = 500
    eval_interval = 100
    save_interval = 10
    learning_rate_function = lambda epoch: exponential_decay(exponential_decay(1,epoch%target_interval,0.9,2),epoch,0.95,target_interval)

    # get the current time
    start_time = time.time()
    agent.run_multithread_cycle(replay_size=10_000,learning_rate=0.1, moves_min=1,moves_max=50,epsilon=0.2,gamma=0.8,pool=pool,rewards=m_rewards)
    # get the time after running the function
    end_time = time.time()
    # print the time it took to run the function
    print(f"Time taken in seconds: {end_time - start_time}")

    pool.close()
