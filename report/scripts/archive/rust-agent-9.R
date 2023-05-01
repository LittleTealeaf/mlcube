source("../src/database.R")
source("../src/graphs.R")
source("../src/functions.R")
library(tidyverse)
library(ggplot2)

df <- get_epochs(2048)

df %>%
  mutate(InterEpoch = Epoch %% 500) %>%
  mutate(Group = factor(floor(Epoch / 500))) %>%
  ggplot(aes(x = InterEpoch, y = Loss, color = Group)) +
  geom_point(size = 0.05)

df %>%
  mutate(InterEpoch = Epoch %% 500) %>%
  mutate(Group = factor(floor(Epoch / 500))) %>%
  ggplot(aes(x = InterEpoch, y = Loss, color = Group)) +
  geom_smooth(size = 0.5, se = FALSE)

df_evals <- get_evaluation_moves(2048)

df_evals %>% group_evaluations() %>% ggplot(aes(x = Epoch, y = Value, color = Metric)) + geom_smooth(se=FALSE)