-- Groups epochs from each model into at most 100 groups that average equal amounts of epochs.
-- This is used to send data to be graphed without needing to pull and sift through all of the data application-side
CREATE VIEW GroupedEpoch AS
SELECT ROW_NUMBER() OVER (ORDER BY Epoch.ModelId, Epoch.Epoch - (Epoch.Epoch % GroupSize.GroupSize)) GroupedId,
       -- GroupedId is used as an id column, which is required so that prisma can read this table
       Epoch.Epoch - (Epoch.Epoch % GroupSize.GroupSize)                                             EpochGroup,
       -- The EpochGroup squishes all epochs down to groups based on the group size. Data is grouped by this value
       Epoch.ModelId,
       -- The model id, data is also grouped by this value
       AVG(Loss)                                                                                     AvgLoss,
       -- Average loss in each group
       AVG(Reward)                                                                                   AvgReward
       -- Average Reward in each group
FROM Epoch
         LEFT JOIN (SELECT ModelId,
                           IIF(COUNT(*) > 100, FLOOR(Count(*) / 100), 1) GroupSize
                           -- If there are more than 100 entries, take the total entries and divide by 100.
                           -- Otherwise, make the group size 1 (meaning all data points will be returned)
                    FROM Epoch
                    GROUP BY ModelId) GroupSize ON Epoch.ModelId = GroupSize.ModelId
GROUP BY Epoch.ModelId, Epoch.Epoch - (Epoch.Epoch % GroupSize.GroupSize)