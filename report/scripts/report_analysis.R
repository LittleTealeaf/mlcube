library(tidyverse)
library(ggplot2)
source("src/functions.R")
source("src/database.R")


df_epochs <- rbind(
  get_epochs(2043) %>% mutate(Loss = sqrt(Loss)),
  get_epochs(2044, 2045, 2046, 2048, 2052)
) %>% mutate(Model = factor(ModelId))

df_epochs %>%
  filter(Epoch <= 5000 & Loss < 8) %>%
  ggplot(aes(x = Epoch, y = Loss, color = Model)) +
  geom_point(size = 0.1) +
  facet_wrap(~Model, ncol = 1) +
  theme(legend.position = "none")


get_epochs(2052) %>% ggplot(aes(x = Epoch, y = Loss)) +
  geom_point(size=0.1)
