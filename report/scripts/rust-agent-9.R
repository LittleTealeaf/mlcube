source("database.R")
library(tidyverse)
library(ggplot2)

df <- get_epochs(2048)

df %>% ggplot(aes(x = Epoch, y = Loss)) + geom_point()