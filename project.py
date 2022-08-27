import json
import os
import numpy as np
import tensorflow as tf
from random import Random
from keras.optimizers import SGD
from keras.activations import sigmoid

class Move:
    def __init__(
        self, name: str, loops: list[list[int]], two: bool = False, prime: bool = False
    ):
        self.name = name
        self.matrix: np.ndarray[(54, 54), np.int8] = np.identity(9 * 6, dtype=np.int8)
        for loop in loops:
            first = np.copy(self.matrix[loop[0]])
            for i in range(len(loop) - 1):
                self.matrix[loop[i]] = self.matrix[loop[i + 1]]
            self.matrix[loop[-1]] = first
        if two:
            self.matrix = self.matrix @ self.matrix
        if prime:
            self.matrix = self.matrix.T

    def apply(self, state: np.ndarray[54, np.float32]):
        return state @ self.matrix


def create_move(letter: str, loops: list[list[int]]) -> list[Move]:
    "Creates a move, it's prime syntax, and it's double syntax"
    return [
        Move(letter, loops),
        Move(f"{letter}P", loops, prime=True),
        Move(f"{letter}2", loops, two=True),
    ]


# Behold, python syntax
MOVES = [
    move
    for moves in [
        create_move(
            "R",
            [
                [20, 2, 42, 47],
                [23, 5, 39, 50],
                [26, 8, 36, 53],
                [27, 29, 35, 33],
                [28, 32, 34, 30],
            ],
        ),
        create_move(
            "U",
            [
                [20, 11, 38, 29],
                [19, 10, 37, 28],
                [18, 9, 36, 27],
                [8, 6, 0, 2],
                [7, 3, 1, 5],
            ],
        ),
        create_move(
            "L",
            [
                [18, 45, 44, 0],
                [21, 48, 41, 3],
                [24, 51, 38, 6],
                [11, 17, 15, 9],
                [14, 16, 12, 10],
            ],
        ),
        create_move(
            "D",
            [
                [24, 33, 42, 15],
                [25, 34, 43, 16],
                [26, 35, 44, 17],
                [45, 47, 53, 51],
                [46, 50, 52, 48],
            ],
        ),
        create_move(
            "F",
            [
                [6, 27, 47, 17],
                [7, 30, 46, 14],
                [8, 33, 45, 11],
                [18, 20, 26, 24],
                [19, 23, 25, 21],
            ],
        ),
        create_move(
            "B",
            [
                [36, 38, 44, 42],
                [37, 41, 43, 39],
                [29, 0, 15, 53],
                [32, 1, 12, 52],
                [35, 2, 9, 51],
            ],
        ),
    ]
    for move in moves
]


def create_cube():
    state: np.ndarray[54, np.int8] = np.zeros(54, dtype=np.int8)
    for i in range(54):
        state[i] = i // 9
    return state


# def scramble_cube(cube: np.ndarray[54, np.int8], count: int = 100):
#     random = Random()
#     return cube @ reduce(
#         lambda a, b: a @ b, [random.choice(MOVES).matrix for i in range(count)]
#     )


# def create_scrambled_cube(scramble_length: int):
#     return scramble_cube(create_cube(), scramble_length)


# def create_scrambled_sample(count: int, pool: Pool = None, scramble_length: int = 100):
#     if pool:
#         return pool.map(create_scrambled_cube, [scramble_length] * count)
#     else:
#         return [create_scrambled_cube(scramble_length) for _ in range(count)]


def state_to_tensor(state: np.ndarray[54, np.int8]):
    # converted = [state[i // 6] for i in range(54 * 6)]
    converted = [0] * 54 * 6
    for i in range(len(state)):
        converted[state[i] + i * 6] = 1
    return tf.constant(converted, dtype=tf.float32)


def reward(state: np.ndarray[54, np.int8]):
    for i in range(54):
        if state[i] != i // 9:
            return 0
    return 1



class Network:
    def __init__(self,layer_sizes: list[int] = None, layers = None, serialized_example=None):
        self.trainable_variables = []

        if serialized_example:
            features = {}
            for i in range(len(layer_sizes) + 1):
                features[f'W{i}'] = tf.io.RaggedFeature(dtype=tf.string)
                features[f'b{i}'] = tf.io.RaggedFeature(dtype=tf.string)
            example = tf.io.parse_example(serialized_example,features)
            layers = []

            for i in range(len(layer_sizes) + 1):
                W = tf.Variable(tf.io.parse_tensor(example[f'W{i}'][0],out_type=tf.float32,name=f'W{i}'))
                b = tf.Variable( tf.io.parse_tensor(example[f'b{i}'][0],out_type=tf.float32,name=f'b{i}'))
                layers.append((
                    W,b
                ))



        if layers:
            self.layers = layers

            for W,b in self.layers:
                self.trainable_variables.append(W)
                self.trainable_variables.append(b)

            return

        self.layer_sizes = layer_sizes + [len(MOVES)]
        self.layers = []
        for i in range(len(self.layer_sizes)):
            length_prev = self.layer_sizes[i-1] if i > 0 else 54 * 6
            length_cur = self.layer_sizes[i]
            W = tf.Variable(
                tf.random.normal([length_prev, length_cur],stddev=0.03),dtype=tf.float32
            )
            b = tf.Variable(tf.random.normal([length_cur],stddev=0.03),dtype=tf.float32)
            self.layers.append((W,b))
            self.trainable_variables.append(W)
            self.trainable_variables.append(b)

    def apply(self,input):
        x = input
        for W,b in self.layers:
            x = sigmoid(tf.add(tf.matmul(x,W),b))
        return x

    def copy(self):
        return Network(layers=self.layers)

    def serialize(self):

        features = {}

        for i in range(len(self.layers)):
            W,b = self.layers[i]

            # W_feature = tf.train.Feature(bytes_list=tf.train.BytesList(value=[tf.io.serialize_tensor(W).numpy()]))
            # b_feature = tf.train.Feature(bytes_list=tf.train.BytesList(value=[tf.io.serialize_tensor(b).numpy()]))

            # features.append(W_feature)
            # features.append(b_feature)
            features[f'W{i}'] = tf.train.Feature(bytes_list=tf.train.BytesList(value=[tf.io.serialize_tensor(W).numpy()]))
            features[f'b{i}'] = tf.train.Feature(bytes_list=tf.train.BytesList(value=[tf.io.serialize_tensor(b).numpy()]))

        example = tf.train.Example(
            features = tf.train.Features(feature=features)
        )

        return example

class Agent:
    def __init__(self,layer_sizes: list[int] = [54 * 6, 18], dir: str = None):
        self.network = None
        self.dir = dir
        self.training_history = []
        self.evals = []

        if dir and os.path.exists(dir):
            with open("/".join([self.dir,'training_history.json'])) as file:
                self.training_history = json.load(file)

            with open("/".join([self.dir,'evals.json'])) as file:
                self.evals = json.load(file)

            network_data = tf.io.read_file("/".join([self.dir,'agent']))
            self.network = Network(layer_sizes,serialized_example=network_data)

        if not self.network:
            self.network: Network = Network(layer_sizes)



        self.update_target()

    def update_target(self):
        self.target = self.network.copy()

# TODO: use datasets as batches... something wonky going on here
    def create_replay(self,replay_length, EPSILON = 0.5,min_moves=1,max_moves=20):
        random = Random()
        state_1_cubes = []

        for i in range(replay_length):
            cube = create_cube()
            for _ in range(i%max_moves + min_moves):
                cube = random.choice(MOVES).apply(cube)
            state_1_cubes.append(cube)

        state_1 = tf.constant(np.array([state_to_tensor(state) for state in state_1_cubes]))

        state_1_outputs = self.network.apply(state_1)

        state_1_choices = tf.argmax(state_1_outputs,1)
        state_1_choices = tf.map_fn(lambda i: i if random.random() > EPSILON else random.randint(0,len(MOVES)-1),state_1_choices)

        state_2_cubes = [MOVES[state_1_choices[i]].apply(state_1_cubes[i]) for i in range(replay_length)]

        reward_1 = tf.constant(np.array([
            reward(state) for state in state_2_cubes
        ]),dtype=tf.float32)

        state_2 = tf.constant(np.array([state_to_tensor(state) for state in state_2_cubes]))

        return state_1, state_1_choices, reward_1, state_2

    def train_replays(self,replays):
        with tf.GradientTape() as tape:
            state_1, state_1_choices, reward_1, state_2 = replays

            state_1_output = self.network.apply(state_1)
            state_1_choice_q = tf.gather(state_1_output,state_1_choices,batch_dims=1)

            state_2_output = self.target.apply(state_2)
            state_2_choices = tf.argmax(state_2_output,1)
            state_2_choices_q = tf.gather(state_2_output,state_2_choices, batch_dims=1)

            target_q = tf.add(state_2_choices_q, reward_1)

            predicted_q = state_1_choice_q

            loss = tf.square(tf.subtract(target_q, predicted_q))

            gradient = tape.gradient(loss,self.network.trainable_variables)

            return loss, gradient
    def run_epoch(self, replay_size = 1000, min_moves = 1, max_moves = 50, EPSILON = 0.5):
        epoch = len(self.training_history)

        replay = self.create_replay(replay_size, min_moves=min_moves, max_moves = max_moves, EPSILON=EPSILON)
        loss, gradient = self.train_replays(replay)
        loss_avg = tf.math.reduce_mean(loss)
        optimizer = SGD(learning_rate=min(loss_avg,0.01))
        optimizer.apply_gradients(zip(gradient,self.network.trainable_variables))
        self.training_history.append(float(loss_avg.numpy()))
        return epoch, loss_avg

    def save_agent(self):
        serialized = self.network.serialize()
        tf.io.write_file("/".join([self.dir,'agent']),serialized.SerializeToString())
        with open("/".join([self.dir,'training_history.json']),'w') as file:
            file.write(json.dumps(self.training_history))
        with open("/".join([self.dir,'evals.json']),'w') as file:
            file.write(json.dumps(self.evals))


    def evaluate_self(self,max_moves = 10_000, scramble_length = 100):
        cube = create_cube()
        random = Random()
        for _ in range(scramble_length):
            cube = random.choice(MOVES).apply(cube)

        moves = 0

        goal_reward = reward(create_cube())

        while moves < max_moves and reward(cube) != goal_reward:
            moves = moves + 1
            results = self.network.apply(state_to_tensor(cube))
            print(results)


            # cube = MOVES[move].apply(cube)

        self.evals.append(moves)




agent = Agent(layer_sizes=[100,50,25],dir="./agent")
update_interval = 10
evaluate_interval = 10

epoch = 0

while True:
    epoch, loss_avg = agent.run_epoch(replay_size=100, EPSILON = 0.5)

    if epoch % update_interval == 0:
        agent.update_target()
        agent.save_agent()

    if epoch % evaluate_interval == 0:
        agent.evaluate_self()




    print(f'Epoch #{epoch}\tAverage Loss: {loss_avg}')
