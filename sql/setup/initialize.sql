USE mlcube;

CREATE TABLE Models
(
    ModelId   INT          NOT NULL IDENTITY PRIMARY KEY,
    ModelName VARCHAR(100) NOT NULL,
)

CREATE TABLE Nodes
(
    NodeId  INT NOT NULL IDENTITY PRIMARY KEY,
    Weight  FLOAT(53),
    Bias    FLOAT(53),
    Layer   INT NOT NULL,
    Node    INT NOT NULL,
    ModelId INT FOREIGN KEY REFERENCES Models (ModelId)
)

CREATE TABLE Epochs
(
    EpochId INT NOT NULL IDENTITY PRIMARY KEY,
    ModelId INT FOREIGN KEY REFERENCES Models (ModelId),
    Epoch   INT NOT NULL,
    Loss    FLOAT(53),
    Reward  FLOAT(53)
)
