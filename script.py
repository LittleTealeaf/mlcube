from src import *
import time

# TODO: Incorporate multithreading

def exponential_decay(initial, index, decay_rate, decay_interval):
    return initial * (decay_rate ** (index // decay_interval))

def main():
  calculate_rewards()

  agent = Agent([264, 202, 141, 80],"agents/3")


  target_interval = 500
  eval_interval = 100
  save_interval = 10
  learning_rate_function = lambda epoch: exponential_decay(exponential_decay(1,epoch%target_interval,0.9,2),epoch,0.95,target_interval)

  while True:
    epoch = agent.get_epoch()
    learning_rate = learning_rate_function(epoch)
    epsilon = exponential_decay(0.5,epoch,0.85,target_interval)
    avg_loss, abs_loss, avg_reward = agent.run_cycle(replay_size=10_000,learning_rate=learning_rate, moves_min=1,moves_max=50,epsilon=epsilon,gamma=0.8)
    print(f'Epoch {epoch}\t Average Loss: \t{abs_loss} \t Reward: {avg_reward}')

    if epoch % target_interval == 0:
      agent.update_target()

    if epoch % eval_interval == 0:
      eval_result = agent.evaluate_network()
      print(f"Evaluated at {epoch}: {eval_result}")

    if epoch % save_interval == 0:
      agent.save()


if __name__ == "__main__":
  main()
