from src import *
import time

def exponential_decay(initial, index, decay_rate, decay_interval):
    return initial * (decay_rate ** (index // decay_interval))

agent = Agent([100,50],"agents/2")


target_interval = 500
eval_interval = 10
save_interval = 5

while True:
  epoch = agent.get_epoch()
  learning_rate = exponential_decay(exponential_decay(0.1,epoch%500,0.9,3),epoch // 500,0.9,1)
  avg_loss, abs_loss = agent.run_cycle(replay_size=10_000,learning_rate=learning_rate)
  print(f'Epoch {epoch}\t Average Loss \t{avg_loss} \t {abs_loss}')

  if epoch % target_interval == 0:
    agent.update_target()

  if epoch % eval_interval == 0:
    eval_result = agent.evaluate_network()
    print(f"Evaluated at {epoch + 1}: {eval_result}")

  if epoch % save_interval == 0:
    agent.save()
