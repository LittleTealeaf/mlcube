CREATE VIEW GroupedEpoch AS
    SELECT ROW_NUMBER() OVER (ORDER BY Epoch.ModelId, Epoch.Epoch - (Epoch.Epoch % GroupSize.GroupSize)) GroupedId,
           Epoch.Epoch - (Epoch.Epoch % GroupSize.GroupSize)                                             EpochGroup,
           Epoch.ModelId                                                                                 ModelId,
           AVG(Loss)                                                                                     AvgLoss,
           AVG(Reward)                                                                                   AvgReward
    FROM Epoch
             LEFT JOIN (SELECT ModelId,
                               IIF(COUNT(*) > 100, FLOOR(Count(*) / 100), 1) GroupSize
                        FROM Epoch
                        GROUP BY ModelId) GroupSize ON Epoch.ModelId = GroupSize.ModelId
    GROUP BY Epoch.ModelId, Epoch.Epoch - (Epoch.Epoch % GroupSize.GroupSize)