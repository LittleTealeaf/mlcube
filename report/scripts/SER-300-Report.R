library(tidyverse)
library(ggplot2)
source("src/database.R")
source("src/functions.R")

file_name <- function(name) {
  return(paste("../SER-300-Report/assets/", name))
}

save_gg <- function(name, height = 4) {
  ggsave(file_name(name), width = 6.5, height = height, units = "in")
}

get_epochs(31) %>%
  filter(Epoch < 1000) %>%
  ggplot(aes(x = Epoch, y = Loss)) +
  geom_line() +
  geom_vline(xintercept = c(0, 500, 1000), linetype = "dotted")
ggsave(file_name("model_31.png"), width = 6.5, height = 4, units = "in")

get_epochs(39) %>%
  filter(Epoch < 1000) %>%
  ggplot(aes(x = Epoch, y = Loss)) +
  geom_line() +
  geom_vline(xintercept = c(0, 500, 1000), linetype = "dotted")
save_gg("model_39.png")

get_epochs(2043) %>%
  mutate(Loss = sqrt(Loss)) %>%
  filter(Epoch < 10000) %>%
  ggplot(aes(x = Epoch, y = Loss)) +
  geom_line() +
  geom_vline(xintercept = c(500, 1000, 1500, 2000, 2500, 3000, 3500, 4000, 4500, 5000, 5500, 6000, 6500, 7000, 7500, 8000, 8500, 9000, 9500), linetype = "dotted")

save_gg("model_2043_1000.png")

get_epochs(2043) %>%
  mutate(Loss = sqrt(Loss)) %>%
  filter(Epoch > 7750 & Epoch < 9750) %>%
  ggplot(aes(x = Epoch, y = Loss)) +
  geom_line() +
  geom_vline(xintercept = c(8000, 8500, 9000, 9500), linetype = "dotted")

save_gg("model_2043_1.png")


get_epochs(2044, 2045, 2046) %>%
  mutate(Model = factor(ModelId)) %>%
  filter(Epoch < 10000) %>%
  ggplot(aes(x = Epoch, y = Loss, color = Model)) +
  geom_line() +

  facet_wrap(~Model, ncol = 1) +
  theme(legend.position = "none")
save_gg("models_2044_2046_0.png", height = 6)


get_epochs(2044, 2045, 2046) %>%
  mutate(Model = factor(ModelId)) %>%
  mutate(Epoch = case_when(Model == 2044 ~ Epoch, Model != 2044 ~ Epoch * 2)) %>%
  filter(Epoch < 10000) %>%
  ggplot(aes(x = Epoch, y = Loss, color = Model)) +
  geom_line() +
  geom_vline(xintercept = c(1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000), linetype = "dotted") +
  facet_wrap(~Model, ncol = 1) +
  theme(legend.position = "none")
save_gg("models_2044_2046_1.png", height = 6)

get_evaluation_data(2044, 2045, 2046) %>%
  filter(Epoch < 10000) %>%
  ggdata_summarize_evaluation_data() %>%
  ggplot(aes(x = Epoch, y = Value, color = Metric)) +
  facet_wrap(~ModelId, ncol = 1) +
  geom_smooth(se = FALSE) +
  labs(
    y = "Reward"
  )
save_gg("models_2044_2046_2.png", height = 6)