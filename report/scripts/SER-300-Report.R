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

get_epochs(2043) %>%
  mutate(Loss = sqrt(Loss)) %>%
  ggplot(aes(x = Epoch, y = Loss, color = ModelId)) +
  geom_line() +
  theme(legend.position = "none")
save_gg("2043_0.png")

get_epochs(2043) %>%
  filter(Epoch < 10000) %>%
  mutate(Loss = sqrt(Loss)) %>%
  ggplot(aes(x = Epoch, y = Loss, color = ModelId)) +
  geom_line() +
  geom_vline(xintercept = seq.int(500, 9500, 500), linetype = "dotted") +
  theme(legend.position = "none")
save_gg("2043_1.png")

get_epochs(2043) %>%
  filter(Epoch > 8250 & Epoch < 9750) %>%
  mutate(Loss = sqrt(Loss)) %>%
  ggplot(aes(x = Epoch, y = Loss, color = ModelId)) +
  geom_line() +
  geom_vline(xintercept = seq.int(8500,9500,500), linetype = "dotted") +
  theme(legend.position = "none")
save_gg("2043_2.png")

get_epochs(2044,2045,2046) %>%
  mutate(Model = factor(ModelId)) %>%
  ggplot(aes(x = Epoch, y = Loss, color = Model)) +
  theme(legend.position = "none") +
  facet_wrap(~Model, ncol=1) +
  geom_line()
save_gg("2044-2046_0.png", height = 6)

get_epochs(2044,2045,2046) %>%
  mutate(Model = factor(ModelId)) %>%
  mutate(Epoch = case_when(Model == 2044 ~ Epoch, Model != 2044 ~ Epoch * 2)) %>%
  filter(Epoch < 10000) %>%
  ggplot(aes(x = Epoch, y = Loss, color = Model)) +
  theme(legend.position = "none") +
  facet_wrap(~Model, ncol=1) +
  geom_line() +
  geom_vline(xintercept = seq.int(1000,9000,1000), linetype="dotted") +
  labs(
    caption = "Models 2045 and 2046 Epoch values are doubled"
  )
save_gg("2044-2046_1.png", height = 6)

