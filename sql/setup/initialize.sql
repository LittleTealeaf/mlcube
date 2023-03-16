CREATE TABLE Models
(
    ModelId INT NOT NULL IDENTITY PRIMARY KEY,
    ModelName VARCHAR(100) NOT NULL,
    GitHash VARCHAR(40)
)

CREATE TABLE Nodes
(
    NodeId INT NOT NULL IDENTITY PRIMARY KEY,
    Weight FLOAT(53),
    Bias FLOAT(53),
    Layer INT NOT NULL,
    Node INT NOT NULL,
    ModelId INT NOT NULL FOREIGN KEY REFERENCES Models (ModelId)
)

CREATE TABLE Epochs
(
    EpochId INT NOT NULL IDENTITY PRIMARY KEY,
    ModelId INT NOT NULL FOREIGN KEY REFERENCES Models (ModelId),
    Epoch INT NOT NULL,
    Loss FLOAT(53),
    Reward FLOAT(53)
)

CREATE TABLE Evaluations
(
    EvaluationId INT NOT NULL IDENTITY PRIMARY KEY,
    ModelId INT NOT NULL FOREIGN KEY REFERENCES Models (ModelId),
    Epoch INT NOT NULL,
    Solved BIT,
    MoveCount INT,
    Seed BIGINT,
)

CREATE TABLE EvaluationMoves
(
    EvaluationMoveId INT NOT NULL IDENTITY PRIMARY KEY,
    EvaluationId INT FOREIGN KEY REFERENCES Evaluations (EvaluationId),
    MoveIndex INT,
    MoveName VARCHAR(2),
    Reward FLOAT(53)
)