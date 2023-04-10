CREATE VIEW EvaluationInfo AS
SELECT Eval.EvaluationId,
       Eval.ModelId,
       Eval.Epoch,
       Eval.Seed,
       Eval.Solved,
       EvalMoveAggregate.AvgReward,
       EvalMoveAggregate.MaxReward,
       EvalMoveAggregate.MinReward,
       LastEvalMove.Reward FinalReward,
       EvalMoveAggregate.MoveCount
FROM Evaluation Eval
         LEFT JOIN (SELECT EvaluationId,
                           AVG(Reward)    AvgReward,
                           MAX(Reward)    MaxReward,
                           MIN(Reward)    MinReward,
                           MAX(MoveIndex) LastMoveIndex,
                           COUNT(*)       MoveCount
                    FROM EvaluationMove
                    GROUP BY EvaluationId) EvalMoveAggregate
                   ON Eval.EvaluationId = EvalMoveAggregate.EvaluationId
         LEFT JOIN (SELECT EvaluationId, Reward, MoveIndex
                    FROM EvaluationMove) LastEvalMove
                   ON LastEvalMove.EvaluationId = Eval.EvaluationId AND
                      LastEvalMove.MoveIndex = EvalMoveAggregate.LastMoveIndex