from src import *
from multiprocessing import Manager, Pool
import os

# THOUGHTS
# - Switch to a legitimate reinforced method that builds a replay that it trains on (from a single solved cube). -> this may fix the whole "repeating the same move" over and over because it's training on what it actually ends up doing (well, also epsilon but start epsilon at 0.75 and it'll be significant towards completion)
# I need to look up how it ends up building the replay

def exponential_decay(initial, index, decay_rate, decay_interval =  1):
    return initial * (decay_rate ** (index // decay_interval))

def linear_trend(start_x,end_x,start_y,end_y,x):
  slope = (end_y - start_y) / (end_x - start_x)
  return (x - start_x) * slope + start_y

if __name__ == "__main__":
  with Pool(24) as pool:
    thread_manager = Manager()

    REWARDS = calculate_rewards(depth=6,decay=0.9,max_count=1_000_000)

    agent = Agent([264, 202, 141, 80],"agents/B-5")

    # 4 - 500 INTERVAL
    # 5 - 30 INTERVAL

    target_interval = 1000
    eval_interval = 7
    save_interval = 10
    # max_gamma = 0.8

    random = Random()

    while not os.path.exists("./stop"):
      epoch = agent.get_epoch()
      # learning_rate = exponential_decay(exponential_decay(1,epoch%target_interval,0.99),epoch,0.94,target_interval)
      # epsilon = exponential_decay(1,epoch%target_interval,0.99)
      # gamma = exponential_decay(linear_trend(0,target_interval,1,0,epoch%target_interval),epoch,0.995,target_interval)

      learning_rate = max(exponential_decay(0.1,epoch,0.9,target_interval),1e-6)
      epsilon = max(exponential_decay(0.75,epoch,0.9,target_interval),0.01)
      # gamma = linear_trend(0,target_interval,1,0,epoch%target_interval)
      # gamma = 1 - (gamma * max_gamma + (1 - max_gamma))

      outputs = agent.run_cycle(
        pool=pool,
        replay_size=10_001,
        epsilon=epsilon,
        learning_rate=learning_rate,
        moves_min=0,
        moves_max=20,
        gamma=0.9,
        rewards=REWARDS,
        random=random
      )
      print(outputs)

      epoch = epoch + 1

      if epoch % target_interval == 0 :
        agent.update_target()

      if epoch % eval_interval == 0:
        eval_result = agent.evaluate_network(rewards=REWARDS, random=random)
        print(eval_result)

      if epoch % save_interval == 0:
        agent.save()

    os.remove("./stop")
