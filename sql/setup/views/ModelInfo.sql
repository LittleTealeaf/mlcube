CREATE VIEW ModelInfo AS

SELECT models.*, EpochCounts.EpochCount
FROM Models models
         INNER JOIN (SELECT ModelId, COUNT(*) EpochCount
                     FROM Epochs
                     GROUP BY ModelId) EpochCounts ON EpochCounts.ModelId = models.ModelId