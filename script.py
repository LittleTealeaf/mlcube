from src import *
from multiprocessing import Manager
import os

def exponential_decay(initial, index, decay_rate, decay_interval =  1):
    return initial * (decay_rate ** (index // decay_interval))

def linear_trend(start_x,end_x,start_y,end_y,x):
  slope = (end_y - start_y) / (end_x - start_x)
  return (x - start_x) * slope + start_y

if __name__ == "__main__":
  with Pool(24) as pool:
    thread_manager = Manager()

    REWARDS = calculate_rewards(depth=5,decay=0.9)

    agent = Agent([264, 202, 141, 80],"agents/A-2")


    # 4 - 500 INTERVAL
    # 5 - 30 INTERVAL

    target_interval = 500
    eval_interval = 5
    save_interval = 10
    max_gamma = 0.8

    random = Random()

    while not os.path.exists("./stop"):
      epoch = agent.get_epoch()
      # learning_rate = exponential_decay(exponential_decay(1,epoch%target_interval,0.99),epoch,0.94,target_interval)
      # epsilon = exponential_decay(1,epoch%target_interval,0.99)
      # gamma = exponential_decay(linear_trend(0,target_interval,1,0,epoch%target_interval),epoch,0.995,target_interval)

      learning_rate = exponential_decay(1,epoch,0.99,target_interval)
      epsilon = exponential_decay(1,epoch,0.99,target_interval)
      gamma = linear_trend(0,target_interval,1,0,epoch%target_interval)
      gamma = 1 - (gamma * max_gamma + (1 - max_gamma))

      outputs = agent.run_cycle(
        pool=pool,
        replay_size=10_000,
        epsilon=epsilon,
        learning_rate=learning_rate,
        moves_min=1,
        moves_max=30,
        gamma=gamma,
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



    # # get the current time
    # for i in range(100):
    #   start_time = time.time()
    #   mean_loss = agent.run_cycle(
    #     pool=pool,
    #     replay_size=10_000,
    #     learning_rate=0.1,
    #     moves_min=1,
    #     moves_max=50,
    #     epsilon=0.2,
    #     gamma=0.5,
    #     rewards=REWARDS
    #   )
    #   # get the end time
    #   end_time = time.time()

    #   # print the time taken
    #   print(f"Runtime of the program is {end_time - start_time} with loss {mean_loss}")
