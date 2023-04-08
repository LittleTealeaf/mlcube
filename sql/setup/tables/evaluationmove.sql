CREATE TABLE EvaluationMove
(
    EvaluationMoveId INT NOT NULL IDENTITY PRIMARY KEY,
    EvaluationId     INT FOREIGN KEY REFERENCES Evaluation (EvaluationId),
    MoveIndex        INT,
    MoveName         VARCHAR(10),
    Reward           FLOAT(53)
)