CREATE PROCEDURE create_evaluation(@ModelId INT,
    @Epoch INT,
    @Seed BIGINT,
    @Solved BIT)
AS
BEGIN
    INSERT INTO Evaluations
        (ModelId, Epoch, Seed, Solved)
    OUTPUT
    inserted.EvaluationId
    VALUES
        (@ModelId, @Epoch, @Seed, @Solved)
END
