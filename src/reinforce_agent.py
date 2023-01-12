import reverb
from random import Random
from keras.optimizers import Adam
from tensorflow import Variable, train
from tensorflow._api.v2.data import Dataset
from tensorflow._api.v2.data import Iterator
from tf_agents import trajectories
from tf_agents.drivers.py_driver import PyDriver
from tf_agents.agents.reinforce.reinforce_agent import ReinforceAgent
from tf_agents.environments import TFPyEnvironment
from tf_agents.environments.wrappers import TimeLimit
from tf_agents.networks.actor_distribution_network import ActorDistributionNetwork
from tf_agents.policies import py_tf_eager_policy
from tf_agents.specs import tensor_spec
from tf_agents.utils import common
from environment import CubeEnvironment
from tf_agents.replay_buffers import ReverbReplayBuffer, reverb_utils, reverb_replay_buffer
from matplotlib import pyplot as plt


env_name = "Cubes"  # @param {type:"string"}
num_iterations = 1000  # @param {type: "integer"}
collect_episodes_per_iteration = 100  # @param {type: "integer"}
replay_buffer_capacity = 10000  # @param {type: "integer"}

fc_layer_params = (300, 300, 300,)

learning_rate = 1e-3  # @param {type: "number"}
log_interval = 5  # @param {type: "integer"}
num_eval_episodes = 10  # @param {type: "integer"}
eval_interval = 100  # @param {type: "integer"}

env_timeout = 200

seed = Random().random()

train_py_env = TimeLimit(CubeEnvironment(seed=seed), env_timeout)
eval_py_env = CubeEnvironment(print_steps=True, moves_max=env_timeout)
train_env = TFPyEnvironment(train_py_env)
eval_env = TFPyEnvironment(eval_py_env)


actor_net = ActorDistributionNetwork(train_env.observation_spec(
), train_env.action_spec(), fc_layer_params=fc_layer_params)
optimizer = Adam(learning_rate=learning_rate)

train_step_counter = Variable(0)

print("Creating tf_agent")

tf_agent = ReinforceAgent(
    train_env.time_step_spec(),
    train_env.action_spec(),
    actor_network=actor_net,
    optimizer=optimizer,
    normalize_returns=True,
    train_step_counter=train_step_counter
)
tf_agent.initialize()


eval_policy = tf_agent.policy
collect_policy = tf_agent.collect_policy


def compute_avg_return(environment, policy, num_episodes=10):
    total_return = 0.0
    for _ in range(num_episodes):
        time_step = environment.reset()
        episode_return = 0.0
        while not time_step.is_last():
            action_step = policy.action(time_step)
            time_step = environment.step(action_step.action)
            episode_return += time_step.reward
        total_return += episode_return
    avg_return = total_return / num_episodes
    return avg_return


print("Creating replay")

table_name = 'uniform_table'
replay_buffer_signature = tensor_spec.add_outer_dim(
    tensor_spec.from_spec(tf_agent.collect_data_spec))

table = reverb.Table(
    table_name,
    max_size=replay_buffer_capacity,
    sampler=reverb.selectors.Uniform(),
    remover=reverb.selectors.Fifo(),
    rate_limiter=reverb.rate_limiters.MinSize(1),
    signature=replay_buffer_signature
)

print("Creating reverb server")

reverb_server = reverb.Server([table])

print("Creating replay buffer")

replay_buffer = ReverbReplayBuffer(
    tf_agent.collect_data_spec,
    table_name=table_name,
    sequence_length=None,
    local_server=reverb_server
)

print("Creating reverb observer")

rb_observer = reverb_utils.ReverbAddEpisodeObserver(
    replay_buffer.py_client,
    table_name,
    replay_buffer_capacity
)


def collect_episode(environment, policy, num_episodes):
    driver = PyDriver(
        train_py_env,
        py_tf_eager_policy.PyTFEagerPolicy(
            policy, use_tf_function=True,
        ),
        [rb_observer],
        max_episodes=num_episodes
    )
    initial_time_step = environment.reset()
    driver.run(initial_time_step)


print("creating tf_agent.train")

tf_agent.train = common.function(tf_agent.train)

print("Setting train step counter")

tf_agent.train_step_counter.assign(0)

print("Evaluate the agent's policy once before training")

avg_return = compute_avg_return(eval_env, tf_agent.policy, num_eval_episodes)
returns = [avg_return]


py_env = TimeLimit(CubeEnvironment(seed=seed), env_timeout)

collect_driver = PyDriver(
    py_env,
    py_tf_eager_policy.PyTFEagerPolicy(
        tf_agent.collect_policy,
        use_tf_function=True
    ),
    [rb_observer],
    max_episodes=collect_episodes_per_iteration
)


iterator = iter(replay_buffer.as_dataset(sample_batch_size=1))
time_step = py_env.reset()

print("Starting iterations")
for _ in range(num_iterations):

    time_step, _ = collect_driver.run(time_step)

    trajectories, _ = next(iterator)
    train_loss = tf_agent.train(experience=trajectories)

    replay_buffer.clear()

    step = tf_agent.train_step_counter.numpy()

    if step % log_interval == 0:
        print('step = {0}: loss = {1}'.format(step, train_loss.loss))

    if step % eval_interval == 0:
        avg_return = compute_avg_return(
            eval_env, tf_agent.policy, num_eval_episodes)
        print('step = {0}: Average Return = {1}'.format(step, avg_return))
        returns.append(avg_return)


steps = range(0, num_iterations + 1, eval_interval)
plt.plot(steps, returns)
plt.ylabel('Average Return')
plt.xlabel('Step')
plt.show()
