import tensorflow as tf

def create_replay_buffer():
  data_spec = (
    tf.TensorSpec([9*6*6],dtype=tf.float32),
    tf.TensorSpec([1],dtype=tf.int32),
    tf.TensorSpec([9*6*6],dtype=tf.float32)
  )
