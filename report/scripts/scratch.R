library(ggplot2)
library(tidyverse)
source("database.R")


df <- get_epochs(2043) %>% mutate(AvgLoss = sqrt(Loss))

df %>%
  mutate(EpochRange = floor(Epoch %% 500)) %>%
  ggplot(aes(x = Epoch, y = AvgLoss, color = EpochRange)) +
  geom_line()


df %>%
  filter(Epoch < 1500) %>%
  mutate(EpochRange = floor(Epoch %% 500)) %>%
  ggplot(aes(x = Epoch, y = AvgLoss, color = EpochRange)) +
  geom_line()

df80 <- get_epochs(2044)

df80 %>%
  mutate(EpochRange = floor(Epoch %% 1000)) %>%
  ggplot(aes(x = Epoch, y = Loss, color=EpochRange)) +
  geom_line()
