CREATE VIEW ModelInfo AS

SELECT models.*, EpochCounts.EpochCount
FROM Model models
         INNER JOIN (SELECT ModelId, COUNT(*) EpochCount
                     FROM Epoch
                     GROUP BY ModelId) EpochCounts ON EpochCounts.ModelId = models.ModelId