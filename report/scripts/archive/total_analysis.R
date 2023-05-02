source("../src/database.R")
library(tidyverse)
library(ggplot2)

df <- rbind(
  get_epochs(2043) %>% mutate(Loss = sqrt(Loss)),
  get_epochs(2044),
  get_epochs(2045),
  get_epochs(2046),
  get_epochs(2048)
) %>% mutate(Model = factor(ModelId))