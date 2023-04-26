library(ggplot2)
library(tidyverse)
source("database.R")

df <- rbind(get_epochs(2044), get_epochs(2045), get_epochs(2046)) %>%
  mutate(Model = factor(ModelId)) %>%
  select(Model, Epoch, Loss)

df %>%
  filter(Epoch < 7000) %>%
  ggplot(aes(x = Epoch, y = Loss, color = Model)) +
  geom_point(size = 0.1)


df %>%
  mutate(RelativeEpoch = case_when(Model == 2044 ~ Epoch, Model != 2044 ~ Epoch * 2)) %>%
  filter(RelativeEpoch < 7000) %>%
  ggplot(aes(x = RelativeEpoch, y = Loss, color = Model)) +
  geom_point(size = 0.5) +
  geom_vline(xintercept = c(1000, 2000, 3000, 4000, 5000, 6000), linetype = "dotted") +
  labs(
    x = "Epoch (Scaled to synchronize target update cycles)",
    y = "Average Loss",
    title = "Models 2044-2046 with synchronized update cycles",
    caption="Models 2045 and 2046 have a x2 scale to their epochs",
    color = "Model Id"
  )