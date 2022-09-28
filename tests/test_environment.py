from src import *
from random import Random

random = Random()

def test_action_list_has_18_actions():
  assert len(ACTIONS) == 18

def test_action_can_apply_to_environment():
  env = Environment()
  env.apply_action(random.choice(ACTIONS))

def test_any_action_can_apply_to_environment():
  env = Environment()
  for action in ACTIONS:
    env.apply_action(action)

def test_scrambled_cube_is_not_complete():
  env = Environment()
  env.apply_action(random.choice(ACTIONS))
  assert not env.is_complete()

def test_new_environment_is_complete():
  env = Environment()
  assert env.is_complete()

def test_reset_environment_is_complete():
  env = Environment()
  env.apply_action(random.choice(ACTIONS))
  env.reset()
  assert env.is_complete()

def test_simple_action_loops():
  env = Environment()
  for action in ACTIONS:
    env.reset()
    for _ in range(4):
      env.apply_action(action)
    assert env.is_complete()

def test_actions_contains_required_moves():
  REQUIRED_MOVES = [
    "R","RP","R2","U","UP","U2","F","FP","F2","L","LP","L2","D","DP","D2","B","BP","B2"
  ]
  for action in ACTIONS:
    assert action.name in REQUIRED_MOVES
    REQUIRED_MOVES.remove(action.name)
  assert len(REQUIRED_MOVES) == 0

def test_actions_have_permutation_matricies():
  for action in ACTIONS:
    assert action.matrix is not None

def test_cube_can_be_scrambled():
  env = Environment()
  for _ in range(100):
    env.apply_action(random.choice(ACTIONS))

def test_to_observation_returns_array():
  env = Environment()
  assert len(env.to_observations_deprecated()) > 0

def test_observation_formats_correctly():
  env =  Environment()
  observations = env.to_observations_deprecated()[0]
  for i in range(0,len(observations),6):
    assert sum(observations[i:i+6]) == 1

def test_observation_formats_on_scrambled_cube():
  env = Environment()
  for _ in range(100):
    env.apply_action(random.choice(ACTIONS))
  observations = env.to_observations_deprecated()[0]
  for i in range(0,len(observations),6):
    assert sum(observations[i:i+6]) == 1

def test_create_from_observations():
  env = Environment()
  for _ in range(100):
    env.apply_action(random.choice(ACTIONS))
  env_observations = env.to_observations_deprecated()
  env_from_observations = Environment(env_observations)
  for i in range(9 * 6):
    assert env_from_observations.state[i] == env.state[i]
