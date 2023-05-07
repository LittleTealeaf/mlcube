library(tidyverse)

ggdata_summarize_evaluation_data <- function(data) {
  grouped_by_epoch <- data %>% group_by(Epoch, ModelId)
  return(rbind(
    grouped_by_epoch %>%
      summarize(Metric = "Avg", Value = mean(Reward)),
    grouped_by_epoch %>%
      summarize(Metric = "Min", Value = min(Reward)),
    grouped_by_epoch %>%
      summarize(Metric = "Max", Value = max(Reward))

  ))
}