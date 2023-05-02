CREATE VIEW EvaluationData AS
SELECT ModelId, Epoch, Evaluation.EvaluationId Id, Solved,  Seed, MoveIndex, MoveName, Reward
FROM Evaluation
         LEFT JOIN EvaluationMove ON Evaluation.EvaluationId = EvaluationMove.EvaluationId