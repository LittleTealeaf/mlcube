CREATE PROCEDURE create_evaluation_move(@EvaluationId INT,
    @MoveIndex INT,
    @Reward FLOAT)
AS
BEGIN
    INSERT INTO EvaluationMoves
        (EvaluationId, MoveIndex, Reward)
    OUTPUT
    inserted.EvaluationMoveId
    VALUES
        (@EvaluationId, @MoveIndex, @Reward)
END
