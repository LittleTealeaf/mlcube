-- Displays information from the Model table as well as additional information inferred from other sources
CREATE VIEW ModelInfo AS
SELECT models.*, EpochCounts.EpochCount
FROM Model models
         LEFT JOIN (SELECT ModelId, COUNT(*) EpochCount
                    FROM Epoch
                    GROUP BY ModelId) EpochCounts ON EpochCounts.ModelId = models.ModelId