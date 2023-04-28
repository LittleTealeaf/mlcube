library(tidyverse)
library(ggplot2)


gg_eval_data <- function(data) {
  return(rbind(
    data %>%
      group_by(Epoch) %>%
      summarize(Metric = "Avg", Value = mean(Reward)),
    data %>%
      group_by(Epoch) %>%
      summarize(Metric = "Min", Value = min(Reward)),
    data %>%
      group_by(Epoch) %>%
      summarize(Metric = "Max", Value = max(Reward))
  ) %>% ggplot(aes(x = Epoch, y = Value, color = Metric)) + geom_smooth(se = FALSE))
}


