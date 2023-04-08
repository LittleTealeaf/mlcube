CREATE VIEW GroupedEpoch AS
SELECT
        ROW_NUMBER() OVER (ORDER BY ModelId, Epoch.Epoch - (Epoch.Epoch % 50)) GroupedId,
        Epoch.Epoch - (Epoch.Epoch % 50) EpochCategory,
        ModelId,
        AVG(Loss) AvgLoss,
        AVG(Reward) AvgReward
FROM
    Epoch
GROUP BY ModelId, Epoch.Epoch - (Epoch.Epoch % 50)