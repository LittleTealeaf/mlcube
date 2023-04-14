CREATE VIEW EvaluationRewardChange AS
SELECT EM.EvaluationMoveId,
       EM.EvaluationId,
       EM.MoveIndex,
       EM2.Reward - EM.Reward ChangeInReward,
       EM.Reward
FROM EvaluationMove EM
         INNER JOIN EvaluationMove EM2
                    ON EM.EvaluationMoveId + 1 = EM2.EvaluationMoveId AND EM.EvaluationId = EM2.EvaluationId