library(ggplot2)
library(tidyverse)
source("database.R")

conn <- connectToDatabase()
rs <- dbSendQuery(conn, "SELECT * FROM Epoch WHERE ModelId > 2040")
df <- dbFetch(rs, n = -1)
dbDisconnect(conn)


df %>%
  mutate(FactoredModelId = factor(ModelId)) %>%
  ggplot(aes(x = Epoch, y = Loss, color = FactoredModelId)) +
  geom_point()

df %>%
  filter(Loss > 100)



df %>%
  filter(ModelId == 2043  &
           20250 < Epoch &
           Epoch < 21750) %>%
  mutate(EpochGroup = Epoch %% 500) %>%
  ggplot(aes(x = Epoch, y = Loss, color = EpochGroup)) +
  geom_vline(xintercept = 21000) +
  geom_vline(xintercept = 20500) +
  geom_vline(xintercept = 21500) +
  geom_line()


df %>%
  filter(ModelId == 2042) %>%
  ggplot(aes(x = Epoch, y = Loss, color = ModelId)) +
  geom_point() +
  geom_smooth(se=FALSE, span = 0.001)


cnn <- connectToDatabase()
rs <- dbSendQuery(cnn, "SELECT * FROM GroupedEpoch WHERE ModelId > 2040")
df_grouped <- dbFetch(rs, n = -1)
dbDisconnect(cnn)

df_grouped %>%
  mutate(Model = factor(ModelId)) %>%
  ggplot(aes(x = EpochGroup, y = AvgLoss, color=Model )) +
  geom_smooth(span=0.1, se=FALSE)


df %>%
  mutate(EpochRange = floor(Epoch %% 500)) %>%
  filter(ModelId == 2042) %>%
  ggplot(aes(x = Epoch, y = Loss, color = EpochRange)) +
  geom_line()